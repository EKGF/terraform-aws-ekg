use {
    clients::Clients,
    ekg_aws_util::lambda::LambdaDetailStatus::LoaderJobInQueue,
    ekg_identifier::{NS_DATAOPS, NS_RDFS},
    ekg_sparql::Prefixes,
    ekg_util::env::mandatory_env_var_static,
    std::ops::Deref,
};
/// See https://github.com/awslabs/aws-lambda-rust-runtime for more info on Rust runtime for AWS Lambda
use {
    ekg_aws_util::{lambda::LambdaResponse, neptune::LoadRequest},
    ekg_identifier::EkgIdentifierContexts,
    indoc::formatdoc,
    serde_json::Value,
};
pub use {
    lambda_runtime::{service_fn, Error as LambdaError, LambdaEvent},
    request::Request,
};

mod request;

mod clients;
#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    ekg_util::tracing::aws_lfn_init();

    // Get the AWS SDK config
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

    let (payload, _ctx) = event.into_parts();

    handle_lambda_payload(payload, pipeline_id, clients).await
}

async fn handle_lambda_payload(
    payload: Value,
    pipeline_id: &'static str,
    clients: Clients,
) -> Result<LambdaResponse, LambdaError> {
    tracing::trace!(
        "Payload {}",
        serde_json::to_string_pretty(&payload)?
    );

    let request = serde_json::from_value::<Request>(payload).map_err(|e| {
        tracing::error!("Error parsing request: {}", e);
        e
    })?;

    match handle_lambda_request(&request, pipeline_id, clients).await {
        Ok(mut response) => {
            response.clean();
            tracing::info!("Response: {:}", serde_json::to_string(&response)?);
            Ok(response)
        },
        Err(error) => {
            tracing::error!("Error handling request: {:?}", error);
            Err(error.into())
        },
    }
}

async fn handle_lambda_request(
    request: &crate::Request,
    pipeline_id: &'static str,
    clients: Clients,
) -> Result<LambdaResponse, LambdaError> {
    let identifier_contexts = EkgIdentifierContexts::from_env()?;
    let load_request = &request.load_request;
    if request.pipeline_id != pipeline_id {
        return Ok(LambdaResponse::pipeline_id_not_matching(
            request.pipeline_id.as_str(),
            pipeline_id,
        ));
    }

    // First, initiate the load request using the NeptuneData API which gives us
    // a load request ID
    let result = handle_load_request(load_request, pipeline_id, clients.clone()).await?;
    if let Some(result_identifier) = &result.result_identifier {
        tracing::info!("Load request ID: {:?}", result_identifier);
        // First, register the load request in the database itself using SPARQL
        handle_load_request_registration(
            load_request,
            pipeline_id,
            result_identifier.as_str(),
            &identifier_contexts,
            clients.clone(),
        )
        .await?;
    }
    Ok(result)
}

/// Handle the registration (using SPARQL) of the load request in the database
/// itself.
async fn handle_load_request_registration(
    load_request: &LoadRequest,
    pipeline_id: &str,
    load_request_id: &str,
    ekg_identifier_contexts: &EkgIdentifierContexts,
    clients: Clients,
) -> Result<LambdaResponse, LambdaError> {
    // TODO: the string "load-requests" should be based on the name of the terraform
    // module
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
        "Load request registration for RDF file {:} in graph {}",
        load_request.source,
        graph_load_requests
    );

    let sparql = formatdoc! {
        r#"
            INSERT DATA {{
                GRAPH <{graph_load_requests}> {{
                    <{pipeline_iri}> a dataops:Pipeline ;
                        rdfs:label "Pipeline {pipeline_id}" .
                    <{load_request_iri}> a dataops:LoadRequest ; a dataops:QueuedLoadRequest ;
                        rdfs:label "Queued load request for {s3_file}" ;
                        dataops:inPipeline <{pipeline_iri}> .
                    <{s3_iri}> a dataops:Dataset ; a dataops:SingleGraphDataset ;
                        rdfs:label "S3 file {s3_file}" ;
                        dataops:loadedByLoadRequest <{load_request_iri}> .
                }}
            }}
        "#,
        pipeline_id = pipeline_id,
        pipeline_iri = format!("{}dataops-pipeline-{}", ekg_identifier_contexts.internal.ekg_id_base.as_base_iri(), pipeline_id),
        graph_load_requests = graph_load_requests.as_str(),
        load_request_iri = format!("{}uuid:{}", ekg_identifier_contexts.internal.ekg_id_base.as_base_iri(), load_request_id),
        s3_iri = load_request.source,
        s3_file = load_request.source,
    };
    let statement = ekg_sparql::Statement::new(
        Prefixes::builder()
            .declare(NS_DATAOPS.deref())
            .declare(NS_RDFS.deref())
            .build()?,
        std::borrow::Cow::Borrowed(sparql.as_str()),
    )?;

    clients.sparql_client.execute(&statement).await?;

    Ok(LambdaResponse::ok(
        LoaderJobInQueue,
        Some("Load request registered successfully"),
    ))
}

/// Initiate the load request using the NeptuneData API.
async fn handle_load_request(
    load_request: &LoadRequest,
    pipeline_id: &str,
    clients: Clients,
) -> Result<LambdaResponse, LambdaError> {
    tracing::info!(
        "Load request for RDF file {:} (pipeline {:})",
        load_request.source,
        pipeline_id
    );

    let result = clients
        .aws_neptunedata_client
        .start_loader_job()
        .source(&load_request.source)
        .format(load_request.format.as_str().into())
        .iam_role_arn(&load_request.iam_role_arn)
        .mode(load_request.mode.clone().into())
        .s3_bucket_region(load_request.region.as_str().into())
        .fail_on_error(load_request.fail_on_error)
        .parallelism(load_request.parallelism.as_str().into())
        .set_parser_configuration(Some(
            load_request.parser_configuration.as_hash_map(),
        ))
        .update_single_cardinality_properties(load_request.update_single_cardinality_properties)
        .queue_request(load_request.queue_request)
        .set_dependencies(Some(load_request.dependencies.clone()))
        .send()
        .await;

    match result {
        Ok(ref loader_job_output) => Ok(LambdaResponse::from(loader_job_output)),
        Err(error) => Ok(error.into()),
    }
}
