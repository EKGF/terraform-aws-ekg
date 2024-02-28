/// See https://github.com/awslabs/aws-lambda-rust-runtime for more info on Rust runtime for AWS Lambda
use lambda_runtime::{service_fn, Error as LambdaError, LambdaEvent};
use {
    clients::Clients,
    ekg_aws_util::lambda::{
        default_load_request_label,
        LambdaDetailStatus::{self},
        LambdaResponse,
        CLASS_DATAOPS_LOAD_REQUEST,
    },
    ekg_identifier::{
        EkgIdentifierContexts,
        NS_DATAOPS,
        NS_PREFIX_DATAOPS,
        NS_PREFIX_RDFS,
        NS_RDFS,
    },
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

    let source_iri = payload
        .as_object()
        .ok_or(LambdaError::from("Payload is not an object"))?
        .get("load_request")
        .ok_or(LambdaError::from(
            "Missing load_request in payload",
        ))?
        .as_object()
        .ok_or(LambdaError::from("load_request is not an object"))?
        .get("source")
        .ok_or(LambdaError::from("source is not a string"))?
        .as_str()
        .ok_or(LambdaError::from("source is not a string"))?;

    match handle_lambda_request(
        &request,
        &ekg_identifier_contexts,
        pipeline_id,
        load_request_id.as_str(),
        source_iri,
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
                source_iri,
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
    source_iri: &str,
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
                    LambdaDetailStatus::LoaderJobStatusUnknown,
                    Some("Missing overallStatus field in loader job status"),
                ));
            }
            let loader_job_status_str = overall_status.unwrap().as_object().unwrap()["status"]
                .as_string()
                .unwrap();
            let status = LambdaDetailStatus::from_loader_job_status(loader_job_status_str);
            let detailed_message = if status.should_show_detail() {
                Some(format!(
                    "Loader job status for {} is {} with payload {:?}",
                    source_iri,
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
            let response = LambdaResponse::ok(status, detailed_message.as_deref());
            register_load_request_status(
                Ok(&response),
                ekg_identifier_contexts,
                pipeline_id,
                load_request_id,
                source_iri,
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
    source_iri: &str,
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
        "Load request status registration for load request {} in pipeline {} for source IRI {}",
        load_request_id,
        pipeline_id,
        source_iri
    );

    let load_request_type = match check_result {
        Ok(response) => response.detail_status.rdf_class(),
        Err(_) => LambdaDetailStatus::LoaderJobStatusUnknown.rdf_class(),
    };

    let sparql = formatdoc! {
        r#"
            WITH <{graph_load_requests}>
            DELETE {{
                ?loadRequest a {load_request_type} .
                ?loadRequest a ?loadRequestType .
                ?loadRequest {rdfs}label ?loadRequestLabel .
                ?loadRequest {dataops}source ?loadRequestSource .
                ?loadRequest {dataops}graph ?loadRequestGraph .
            }}
            INSERT {{
                ?loadRequest a {load_request_type} .
                ?loadRequest a {load_request_status_type} .
                ?loadRequest {rdfs}label "{load_request_label}" .
                ?loadRequest {rdfs}comment """{payload_string}""" .
                ?loadRequest {dataops}source <{source_iri}> .
                ?loadRequest {dataops}graph <{source_iri}> .
            }}
            WHERE {{
                VALUES ?loadRequest {{
                    <{load_request_iri}>
                }}
                ?loadRequest a {load_request_type} .
                ?loadRequest a ?loadRequestType .
                OPTIONAL {{
                    ?loadRequest {dataops}source ?loadRequestSource .
                }}
                OPTIONAL {{
                    ?loadRequest {dataops}graph ?loadRequestGraph .
                }}
                OPTIONAL {{
                    ?loadRequest {rdfs}label ?loadRequestLabel .
                }}
            }}
        "#,
        dataops = NS_PREFIX_DATAOPS,
        rdfs = NS_PREFIX_RDFS,
        graph_load_requests = graph_load_requests.as_str(),
        load_request_iri = format!("{}uuid:{}", ekg_identifier_contexts.internal.ekg_id_base.as_base_iri(), load_request_id),
        load_request_type = CLASS_DATAOPS_LOAD_REQUEST.display_turtle(),
        load_request_status_type = load_request_type.display_turtle(),
        load_request_label = default_load_request_label(load_request_type, load_request_id, source_iri),
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
