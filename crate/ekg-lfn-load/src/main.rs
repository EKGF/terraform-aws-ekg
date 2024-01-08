use indoc::formatdoc;
/// See https://github.com/awslabs/aws-lambda-rust-runtime for more info on Rust runtime for AWS Lambda
use lambda_runtime::{service_fn, Error as LambdaError, LambdaEvent};

use ekg_aws_util::neptune::LoadRequest;
pub use request::Request;
use {
    ekg_aws_util::lambda::LambdaResponse, ekg_identifier::EkgIdentifierContexts, serde_json::Value,
};

mod request;

#[cfg(test)]
mod tests;

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    ekg_util::tracing::aws_lfn_init();
    // Get the AWS config
    let aws_config = aws_config::load_from_env().await;
    // Create the NeptuneData client
    let aws_neptunedata_client = ekg_aws_util::neptune::get_neptunedata_client(&aws_config)?;

    // Call the actual handler of the request
    let func = service_fn(move |req| handle_lambda_event(req, aws_neptunedata_client.clone()));
    lambda_runtime::run(func).await?;
    Ok(())
}

/// The actual handler of the Lambda request.
async fn handle_lambda_event(
    event: LambdaEvent<Value>,
    aws_neptunedata_client: aws_sdk_neptunedata::Client,
) -> Result<LambdaResponse, LambdaError> {
    tracing::trace!("Event {:#?}\n\n", event.clone());

    let (payload, _ctx) = event.into_parts();

    handle_lambda_payload(payload, aws_neptunedata_client).await
}

async fn handle_lambda_payload(
    payload: Value,
    aws_neptunedata_client: aws_sdk_neptunedata::Client,
) -> Result<LambdaResponse, LambdaError> {
    tracing::trace!(
        "Payload {}",
        serde_json::to_string_pretty(&payload)?
    );

    let request = serde_json::from_value::<crate::Request>(payload).map_err(|e| {
        tracing::error!("Error parsing request: {}", e);
        e
    })?;

    match handle_lambda_request(&request, aws_neptunedata_client).await {
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
    aws_neptunedata_client: aws_sdk_neptunedata::Client,
) -> Result<LambdaResponse, LambdaError> {
    let identifier_contexts = EkgIdentifierContexts::from_env()?;
    let load_request = &request.load_request;

    // First, register the load request in the database itself using SPARQL
    handle_load_request_registration(
        load_request,
        &identifier_contexts,
        aws_neptunedata_client.clone(),
    )
    .await?;
    // Then, initiate the load request using the NeptuneData API
    handle_load_request(load_request, aws_neptunedata_client).await
}

/// Handle the registration (using SPARQL) of the load request in the database itself.
async fn handle_load_request_registration(
    load_request: &LoadRequest,
    ekg_identifier_contexts: &EkgIdentifierContexts,
    aws_neptunedata_client: aws_sdk_neptunedata::Client,
) -> Result<LambdaResponse, LambdaError> {
    // TODO: the string "load-requests" should be based on the name of the terraform module
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
            PREFIX rdfs:   <http://www.w3.org/2000/01/rdf-schema#>
            PREFIX ekgops: <https://ekgf.org/ontology/ekgops#>
    
            INSERT DATA {
                GRAPH <{graph_load_requests}> {
                    <{load_request}> a ekgops:LoadRequest ;
                        rdfs:label "Load request for {s3_file}" .
                }
            }
        "#,
        graph_load_requests = graph_load_requests,
        s3_file = load_request.source,
    };

    let result = aws_neptunedata_client
        .sparql()
        .query(sparql.as_str())
        .send()
        .await;

    match result {
        Ok(_) => Ok(LambdaResponse::ok(
            "Load request registered successfully",
            None,
        )),
        Err(error) => Ok(error.into()),
    }
}

/// Initiate the load request using the NeptuneData API.
async fn handle_load_request(
    load_request: &LoadRequest,
    aws_neptunedata_client: aws_sdk_neptunedata::Client,
) -> Result<LambdaResponse, LambdaError> {
    tracing::info!(
        "Load request for RDF file {:}",
        load_request.source
    );

    let result = aws_neptunedata_client
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
