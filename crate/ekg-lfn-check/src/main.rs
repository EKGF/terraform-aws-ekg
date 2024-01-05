/// See https://github.com/awslabs/aws-lambda-rust-runtime for more info on Rust runtime for AWS Lambda
use lambda_runtime::{service_fn, Error as LambdaError, LambdaEvent};

use {ekg_aws_util::lambda::LambdaResponse, serde_json::Value};

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

/// The actual handler of the Lambda payload.
///
/// - `request`: The output of the ekg_lfn_load Lambda function is
///   the input to this one.
async fn handle_lambda_request(
    request: &LambdaResponse,
    aws_neptunedata_client: aws_sdk_neptunedata::Client,
) -> Result<LambdaResponse, LambdaError> {
    let load_id = request
        .result_identifier
        .as_deref()
        .ok_or(LambdaError::from(
            "Missing result_identifier in request",
        ))?;

    tracing::info!(
        "Check whether load job has finished: {:?}",
        load_id
    );

    let result = aws_neptunedata_client
        .get_loader_job_status()
        .load_id(load_id)
        .send()
        .await;

    match result {
        Ok(loader_job_status) => Ok(LambdaResponse::ok(
            format!(
                "Loader job status is {}",
                loader_job_status.status()
            )
            .as_str(),
            Some(format!("{:?}", loader_job_status.payload()).as_str()),
        )),
        Err(error) => Ok(error.into()),
    }
}
