/// See https://github.com/awslabs/aws-lambda-rust-runtime for more info on Rust runtime for AWS Lambda
use lambda_runtime::{service_fn, Error as LambdaError, LambdaEvent};
use {
    aws_sdk_neptunedata::config::timeout::TimeoutConfig,
    aws_types::region::Region,
    ekg_error::Error,
    ekg_identifier::EkgIdentifierContexts,
    ekg_util::env::mandatory_env_var,
    requests::EkgLfnRequest,
    response::LambdaResponse,
    serde_json::Value,
};

mod response;
#[cfg(test)]
mod tests;

fn get_neptunedata_client_config(
    aws_sdk_config: &aws_types::SdkConfig,
) -> Result<aws_sdk_neptunedata::Config, Error> {
    let region = Region::new(mandatory_env_var("AWS_REGION", None)?);
    let loader_endpoint = mandatory_env_var("EKG_SPARQL_LOADER_ENDPOINT", None)?;
    let loader_endpoint = loader_endpoint
        .strip_suffix("/loader")
        .unwrap_or(loader_endpoint.as_str());
    let timeout_config = TimeoutConfig::builder()
        // .operation_attempt_timeout(std::time::Duration::from_secs(5 * 60))
        .operation_attempt_timeout(std::time::Duration::from_secs(5))
        .build();
    let neptunedata_client_config = aws_sdk_neptunedata::Config::new(aws_sdk_config)
        .to_builder()
        .region(region)
        .endpoint_url(loader_endpoint)
        .timeout_config(timeout_config)
        .use_dual_stack(false)
        .build();
    tracing::trace!(
        "neptunedata_client_config: {:#?}",
        neptunedata_client_config
    );
    Ok(neptunedata_client_config)
}

fn get_neptunedata_client(
    aws_sdk_config: &aws_types::SdkConfig,
) -> Result<aws_sdk_neptunedata::Client, Error> {
    // tracing::info!("aws_sdk_config: {:#?}", aws_sdk_config);

    let neptune_client_config = get_neptunedata_client_config(aws_sdk_config)?;
    Ok(aws_sdk_neptunedata::Client::from_conf(
        neptune_client_config,
    ))
}

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    ekg_util::tracing::aws_lfn_init();
    // Get the AWS config
    let aws_config = aws_config::load_from_env().await;
    // Create the NeptuneData client
    let aws_neptunedata_client = get_neptunedata_client(&aws_config)?;

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

    tracing::trace!("Payload {:#?}\n\n", payload);

    let request = serde_json::from_value::<EkgLfnRequest>(payload).map_err(|e| {
        tracing::error!("Error parsing request: {}", e);
        e
    })?;

    match handle_lambda_payload(&request, aws_neptunedata_client).await {
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

async fn handle_lambda_payload(
    request: &EkgLfnRequest,
    aws_neptunedata_client: aws_sdk_neptunedata::Client,
) -> Result<LambdaResponse, LambdaError> {
    let _identifier_contexts = EkgIdentifierContexts::from_env()?;
    let load_request = &request.load_request;

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
        Ok(_) => {
            Ok(LambdaResponse::ok(
                "Loader job started successfully",
            ))
        },
        Err(error) => Ok(error.into()),
    }
}
