/// See https://github.com/awslabs/aws-lambda-rust-runtime for more info on Rust runtime for AWS Lambda
use lambda_runtime::{service_fn, Error as LambdaError, LambdaEvent};
use {
    ekg_aws_util::lambda::{LambdaDetailStatus, LambdaResponse},
    serde_json::Value,
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
/// - `request`: The output of the ekg_lfn_load Lambda function is the input to
///   this one.
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
            Ok(LambdaResponse::ok(
                msg,
                detailed_message.as_deref(),
                Some(detail_status),
            ))
        },
        Err(error) => Ok(error.into()),
    }
}
