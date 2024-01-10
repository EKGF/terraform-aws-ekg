use {clients::Clients, std::ops::Deref};
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
    // Get the AWS config
    let aws_config = aws_config::load_from_env().await;
    let clients = Clients {
        // Create the NeptuneData client
        aws_neptunedata_client: ekg_aws_util::neptune::get_neptunedata_client(&aws_config)?,
        // Create the HTTP SPARQL client (which strangely enough is not part of the
        // aws_sdk_neptunedata or aws_sdk_neptune crates, we had to build one ourselves)
        sparql_client:          ekg_sparql::SPARQLClient::from_env().await?,
    };

    // Call the actual handler of the request
    let func = service_fn(move |req| handle_lambda_event(req, clients.clone()));
    lambda_runtime::run(func).await?;
    Ok(())
}

/// The actual handler of the Lambda request.
async fn handle_lambda_event(
    event: LambdaEvent<Value>,
    clients: Clients,
) -> Result<LambdaResponse, LambdaError> {
    tracing::trace!("Event {:#?}\n\n", event.clone());

    let (payload, _ctx) = event.into_parts();

    handle_lambda_payload(payload, clients).await
}

async fn handle_lambda_payload(
    payload: Value,
    clients: Clients,
) -> Result<LambdaResponse, LambdaError> {
    tracing::trace!(
        "Payload {}",
        serde_json::to_string_pretty(&payload)?
    );

    let request = serde_json::from_value::<crate::Request>(payload).map_err(|e| {
        tracing::error!("Error parsing request: {}", e);
        e
    })?;

    match handle_lambda_request(&request, clients).await {
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
    clients: Clients,
) -> Result<LambdaResponse, LambdaError> {
    let identifier_contexts = EkgIdentifierContexts::from_env()?;
    let load_request = &request.load_request;

    // First, initiate the load request using the NeptuneData API which gives us
    // a load request ID
    let result = handle_load_request(load_request, clients.clone()).await?;
    if let Some(result_identifier) = &result.result_identifier {
        tracing::info!("Load request ID: {:?}", result_identifier);
        // First, register the load request in the database itself using SPARQL
        handle_load_request_registration(
            load_request,
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
    load_request_id: &str,
    ekg_identifier_contexts: &EkgIdentifierContexts,
    clients: Clients,
) -> Result<LambdaResponse, LambdaError> {
    // TODO: the string "load-requests" should be based on the name of the terraform
    // module
    let graph_load_requests = format!(
        "{}/{}",
        ekg_identifier_contexts.internal.ekg_graph_base, "load-requests"
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
                    <{load_request_iri}> a dataops:LoadRequest ;
                        rdfs:label "Load request for {s3_file}" .
                }}
            }}
        "#,
        graph_load_requests = graph_load_requests.as_str(),
        load_request_iri = format!("{}/{}", ekg_identifier_contexts.internal.ekg_id_base.as_str(), load_request_id),
        s3_file = load_request.source,
    };
    let statement = ekg_sparql::Statement::new(
        &vec![
            ekg_namespace::PREFIX_RDFS.deref(),
            ekg_namespace::PREFIX_DATAOPS.deref(),
        ]
        .into(),
        std::borrow::Cow::Borrowed(sparql.as_str()),
    )?;

    clients.sparql_client.execute(&statement).await?;

    Ok(LambdaResponse::ok(
        "Load request registered successfully",
        None,
    ))
}

/// Initiate the load request using the NeptuneData API.
async fn handle_load_request(
    load_request: &LoadRequest,
    clients: Clients,
) -> Result<LambdaResponse, LambdaError> {
    tracing::info!(
        "Load request for RDF file {:}",
        load_request.source
    );

    let result = clients
        .aws_neptunedata_client
        .start_loader_job()
        .source(&load_request.source)
        .format(load_request.format.as_str().into())
        .iam_role_arn(&load_request.iam_role_arn)
        .mode(load_request.mode.clone().into())
        .s3_bucket_region(load_request.region.as_str().into())
        .fail_on_error(load_request.fail_on_error.as_str() == "TRUE")
        .parallelism(load_request.parallelism.as_str().into())
        .set_parser_configuration(Some(
            load_request.parser_configuration.as_hash_map(),
        ))
        .update_single_cardinality_properties(
            load_request.update_single_cardinality_properties.as_str() == "TRUE",
        )
        .queue_request(load_request.queue_request)
        .set_dependencies(Some(load_request.dependencies.clone()))
        .send()
        .await;

    match result {
        Ok(ref loader_job) => Ok(loader_job.into()),
        Err(error) => Ok(error.into()),
    }
}
