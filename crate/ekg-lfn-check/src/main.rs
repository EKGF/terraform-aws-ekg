/// See https://github.com/awslabs/aws-lambda-rust-runtime for more info on Rust runtime for AWS Lambda
use lambda_runtime::{service_fn, Error as LambdaError, LambdaEvent};
use {
    clients::Clients,
    ekg_aws_util::lambda::{
        LambdaDetailStatus::{self},
        LambdaResponse,
    },
    ekg_identifier::{EkgIdentifierContexts, NS_DATAOPS, NS_RDFS},
    ekg_sparql::Prefixes,
    ekg_util::env::mandatory_env_var_static,
    indoc::formatdoc,
    serde_json::Value,
    std::ops::Deref,
};

mod request;

mod clients;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    ekg_util::tracing::aws_lfn_init();
    // Get the AWS config
    let aws_sdk_config = ekg_aws_util::sdk_config::create().await?;
    let clients = Clients {
        // Create the NeptuneData client
        aws_neptunedata_client: ekg_aws_util::neptune::get_neptunedata_client(&aws_sdk_config)?,
        // Create the HTTP SPARQL client (which strangely enough is not part of the
        // aws_sdk_neptunedata or aws_sdk_neptune crates, we had to build one ourselves)
        sparql_client:          ekg_sparql::SPARQLClient::from_env().await?,
    };
    let pipeline_id = mandatory_env_var_static("EKG_PIPELINE_ID", None)?;

    // Call the actual handler of the request
    let func = service_fn(move |req| handle_lambda_event(req, pipeline_id, clients.clone()));
    lambda_runtime::run(func).await?;
    Ok(())
}

// noinspection DuplicatedCode
/// The actual handler of the Lambda request.
async fn handle_lambda_event(
    event: LambdaEvent<Value>,
    pipeline_id: &'static str,
    clients: Clients,
) -> Result<LambdaResponse, LambdaError> {
    tracing::trace!("Event {:#?}\n\n", event.clone());

    let ekg_identifier_contexts = EkgIdentifierContexts::from_env()?;

    let (payload, _ctx) = event.into_parts();

    handle_lambda_payload(
        payload,
        &ekg_identifier_contexts,
        pipeline_id,
        clients,
    )
    .await
}

async fn handle_lambda_payload(
    payload: Value,
    ekg_identifier_contexts: &EkgIdentifierContexts,
    pipeline_id: &'static str,
    clients: Clients,
) -> Result<LambdaResponse, LambdaError> {
    tracing::info!(
        "Payload {}",
        serde_json::to_string_pretty(&payload)?
    );

    let load_output = payload
        .as_object()
        .unwrap()
        .get("LoadOutput")
        .ok_or(LambdaError::from("Missing LoadOutput in payload"))?
        .clone();

    tracing::info!(
        "Load output: {:}",
        serde_json::to_string(&load_output)?
    );

    // The output of the ekg_lfn_load Lambda function is the input to this one.
    let request = serde_json::from_value::<LambdaResponse>(load_output).map_err(|e| {
        tracing::error!("Error parsing request: {}", e);
        e
    })?;
    let load_request_id = request.result_identifier.as_ref();
    if load_request_id.is_none() {
        return Err(LambdaError::from(
            "Missing result_identifier in request",
        ));
    }
    let load_request_id = load_request_id.unwrap();

    match handle_lambda_request(
        &request,
        &ekg_identifier_contexts,
        pipeline_id,
        load_request_id.as_str(),
        clients.clone(),
    )
    .await
    {
        Ok(mut response) => {
            response.clean();
            tracing::info!("Response: {:}", serde_json::to_string(&response)?);
            Ok(response)
        },
        Err(error) => {
            tracing::error!("Error handling request: {:?}", error);
            register_load_request_status(
                Err(&error),
                ekg_identifier_contexts,
                pipeline_id,
                load_request_id.as_str(),
                None,
                clients.clone(),
            )
            .await?;
            Err(error)
        },
    }
}

/// The actual handler of the Lambda payload.
///
/// - `request`: The output of the ekg_lfn_load Lambda function is the input to
///   this one.
async fn handle_lambda_request(
    load_status_response: &LambdaResponse,
    ekg_identifier_contexts: &EkgIdentifierContexts,
    pipeline_id: &'static str,
    load_request_id: &str,
    clients: Clients,
) -> Result<LambdaResponse, LambdaError> {
    let load_id = load_status_response
        .result_identifier
        .as_deref()
        .ok_or(LambdaError::from(
            "Missing result_identifier in request",
        ))?;

    tracing::info!(
        "Check whether load job has finished: {:?}",
        load_id
    );

    let result = clients
        .aws_neptunedata_client
        .get_loader_job_status()
        .load_id(load_id)
        .errors(true)
        .send()
        .await;

    match result {
        Ok(loader_job_status) => {
            let overall_status = &loader_job_status
                .payload()
                .as_object()
                .unwrap()
                .get("overallStatus");
            if overall_status.is_none() {
                return Ok(LambdaResponse::ok(
                    "Missing overallStatus in loader job status",
                    None,
                    Some(LambdaDetailStatus::LoaderJobStatusUnknown),
                ));
            }
            let status = overall_status.unwrap().as_object().unwrap()["status"]
                .as_string()
                .unwrap();
            let (msg, detail_status, show_detail) = match status {
                "LOAD_IN_QUEUE" => {
                    (
                        "Loader job is still in the queue",
                        LambdaDetailStatus::LoaderJobInQueue,
                        false,
                    )
                },
                "LOAD_NOT_STARTED" => {
                    (
                        "Loader job has not started yet",
                        LambdaDetailStatus::LoaderJobNotStarted,
                        false,
                    )
                },
                "LOAD_IN_PROGRESS" => {
                    (
                        "Loader job is still in progress",
                        LambdaDetailStatus::LoaderJobInProgress,
                        false,
                    )
                },
                "LOAD_COMPLETED" => {
                    (
                        "Loader job completed",
                        LambdaDetailStatus::LoaderJobCompleted,
                        true,
                    )
                },
                "LOAD_CANCELLED_BY_USER" => {
                    (
                        "Loader job cancelled by user",
                        LambdaDetailStatus::LoaderJobCancelledByUser,
                        true,
                    )
                },
                "LOAD_CANCELLED_DUE_TO_ERRORS" => {
                    (
                        "Loader job cancelled due to errors",
                        LambdaDetailStatus::LoaderJobCancelledDueToErrors,
                        true,
                    )
                },
                "LOAD_UNEXPECTED_ERROR" => {
                    (
                        "Loader job failed due to unexpected error",
                        LambdaDetailStatus::LoaderJobUnexpectedError,
                        true,
                    )
                },
                "LOAD_FAILED" => {
                    (
                        "Loader job failed",
                        LambdaDetailStatus::LoaderJobFailed,
                        true,
                    )
                },
                "LOAD_S3_READ_ERROR" => {
                    (
                        "Loader job failed due to S3 read error",
                        LambdaDetailStatus::LoaderJobS3ReadError,
                        true,
                    )
                },
                "LOAD_S3_ACCESS_DENIED_ERROR" => {
                    (
                        "Loader job failed due to S3 access denied error",
                        LambdaDetailStatus::LoaderJobS3AccessDeniedError,
                        true,
                    )
                },
                "LOAD_COMMITTED_W_WRITE_CONFLICTS" => {
                    (
                        "Loader job failed due to write conflicts",
                        LambdaDetailStatus::LoaderJobCommittedWithWriteConflicts,
                        true,
                    )
                },
                "LOAD_DATA_DEADLOCK" => {
                    (
                        "Loader job failed due to data deadlock",
                        LambdaDetailStatus::LoaderJobDataDeadlock,
                        true,
                    )
                },
                "LOAD_DATA_FAILED_DUE_TO_FEED_MODIFIED_OR_DELETED" => {
                    (
                        "Loader job failed because file was deleted or updated after load start.",
                        LambdaDetailStatus::LoaderJobDataFailedDueToFeedModifiedOrDeleted,
                        true,
                    )
                },
                "LOAD_FAILED_BECAUSE_DEPENDENCY_NOT_SATISFIED" => {
                    (
                        "Loader job failed because dependency was not satisfied.",
                        LambdaDetailStatus::LoaderJobFailedBecauseDependencyNotSatisfied,
                        true,
                    )
                },
                "LOAD_FAILED_INVALID_REQUEST" => {
                    (
                        "Loader job failed due to invalid request",
                        LambdaDetailStatus::LoaderJobFailedInvalidRequest,
                        true,
                    )
                },
                _ => {
                    (
                        "Loader job status unknown",
                        LambdaDetailStatus::LoaderJobStatusUnknown,
                        true,
                    )
                },
            };
            let detailed_message = if show_detail {
                Some(format!(
                    "Loader job status is {} with payload {:?}",
                    loader_job_status.status(),
                    loader_job_status.payload()
                ))
            } else {
                None
            };
            // Don't want to use aws_sdk_unstable here (enabling serde support), so we'll
            // just use the debug output to get the payload as a string and then parse that
            // string back into a serde_json::Value
            let payload_string = format!("{:?}", loader_job_status.payload().as_object());
            let response = LambdaResponse::ok(
                msg,
                detailed_message.as_deref(),
                Some(detail_status),
            );
            register_load_request_status(
                Ok(&response),
                ekg_identifier_contexts,
                pipeline_id,
                load_request_id,
                Some(payload_string),
                clients.clone(),
            )
            .await?;

            Ok(response)
        },
        Err(error) => Ok(error.into()),
    }
}

/// After we checked Neptune for the load status, we need to register that
/// status back into the database (if at all possible, if the result of the load
/// status check points out that there's a problem with the database we may not
/// be able to update the status).
async fn register_load_request_status(
    check_result: Result<&LambdaResponse, &LambdaError>,
    ekg_identifier_contexts: &EkgIdentifierContexts,
    pipeline_id: &str,
    load_request_id: &str,
    payload_string: Option<String>,
    clients: Clients,
) -> Result<(), LambdaError> {
    // TODO: the string "load-requests" should be based on the name of the terraform
    //       module
    let graph_load_requests = format!(
        "{}{}-{}",
        ekg_identifier_contexts
            .internal
            .ekg_graph_base
            .as_base_iri(),
        "load-requests",
        pipeline_id
    );

    tracing::info!(
        "Load request status registration for load request {} in pipeline {}",
        load_request_id,
        pipeline_id
    );

    let load_request_type = match check_result {
        Ok(response) => {
            match response.detail_status {
                Some(LambdaDetailStatus::LoaderJobInQueue) => "QueuedLoadRequest",
                Some(LambdaDetailStatus::LoaderJobNotStarted) => "QueuedLoadRequest",
                Some(LambdaDetailStatus::LoaderJobInProgress) => "LoadingLoadRequest",
                Some(LambdaDetailStatus::LoaderJobCompleted) => "FinishedLoadRequest",
                _ => "FailedLoadRequest",
            }
        },
        Err(_) => "FailedLoadRequest",
    };

    let sparql = formatdoc! {
        r#"
            DELETE {{
                GRAPH <{graph_load_requests}> {{
                    <{load_request_iri}> a ?loadRequestType .
                    <{load_request_iri}> rdfs:label ?loadRequestLabel .
                }}
            }}
            INSERT {{
                GRAPH <{graph_load_requests}> {{
                    <{load_request_iri}> a dataops:LoadRequest .
                    <{load_request_iri}> a dataops:{load_request_type} .
                    <{load_request_iri}> rdfs:comment """{payload_string}""" .
                }}
            }}
            WHERE {{
                GRAPH <{graph_load_requests}> {{
                    <{load_request_iri}> a ?loadRequestType .
                    <{load_request_iri}> rdfs:label ?loadRequestLabel .
                }}
            }}
        "#,
        graph_load_requests = graph_load_requests.as_str(),
        load_request_iri = format!("{}uuid:{}", ekg_identifier_contexts.internal.ekg_id_base.as_base_iri(), load_request_id),
        load_request_type = load_request_type,
        payload_string = payload_string.unwrap_or_default()
    };
    let statement = ekg_sparql::Statement::new(
        Prefixes::builder()
            .declare(NS_DATAOPS.deref())
            .declare(NS_RDFS.deref())
            .build()?,
        std::borrow::Cow::Borrowed(sparql.as_str()),
    )?;

    clients.sparql_client.execute(&statement).await?;

    tracing::info!(
        "Load request status \"{}\" registered for load request {} in pipeline {}",
        load_request_type,
        load_request_id,
        pipeline_id
    );

    Ok(())
}
