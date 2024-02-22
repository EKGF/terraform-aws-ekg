/// See https://github.com/awslabs/aws-lambda-rust-runtime for more info on Rust runtime for AWS Lambda
use lambda_runtime::{service_fn, Error as LambdaError, LambdaEvent};
pub use request::Request;
use {
    crate::sfn_state_machine::StateMachine,
    ekg_aws_util::{S3EventRecord, S3EventRecords, SnsEventRecord},
    ekg_error::Error,
    ekg_identifier::EkgIdentifierContexts,
    ekg_util::env::{mandatory_env_var, mandatory_env_var_static},
    serde::Serialize,
    serde_json::{json, Value},
};

mod request;
mod sfn_state_machine;
#[cfg(test)]
mod tests;

/// A simple Lambda response structure.
#[derive(Serialize)]
struct Response {
    req_id: String,
    msg:    String,
}

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    ekg_util::tracing::aws_lfn_init();

    let pipeline_id = mandatory_env_var_static("EKG_PIPELINE_ID", None)?;

    // Get the AWS config
    let aws_config = aws_config::load_from_env().await;
    let aws_sfn_client = aws_sdk_sfn::Client::new(&aws_config);

    // call the actual handler of the request
    let func = service_fn(move |req| handle_lambda_event(req, pipeline_id, aws_sfn_client.clone()));
    lambda_runtime::run(func).await?;
    Ok(())
}

// noinspection DuplicatedCode
/// The actual handler of the Lambda request.
pub(crate) async fn handle_lambda_event(
    event: LambdaEvent<Value>,
    pipeline_id: &'static str,
    aws_sfn_client: aws_sdk_sfn::Client,
) -> Result<Value, LambdaError> {
    tracing::trace!("Event {:#?}\n\n", event.clone());

    let (payload, _ctx) = event.into_parts();

    handle_lambda_payload(payload, pipeline_id, aws_sfn_client).await
}

pub(crate) async fn handle_lambda_payload(
    payload: Value,
    pipeline_id: &'static str,
    aws_sfn_client: aws_sdk_sfn::Client,
) -> Result<Value, LambdaError> {
    tracing::trace!(
        "Payload {}",
        serde_json::to_string_pretty(&payload)?
    );

    let request = serde_json::from_value::<Request>(payload).map_err(|e| {
        tracing::error!("Error parsing request: {}", e);
        e
    })?;

    handle_lambda_request(&request, pipeline_id, aws_sfn_client)
        .await
        .map_err(|e| {
            tracing::error!("Error handling request: {}", e);
            e.into()
        })
}

pub(crate) async fn handle_lambda_request(
    request: &Request,
    pipeline_id: &'static str,
    aws_sfn_client: aws_sdk_sfn::Client,
) -> Result<Value, Error> {
    let identifier_contexts = EkgIdentifierContexts::from_env()?;

    for record in &request.records {
        handle_sns_event_record(
            &record,
            pipeline_id,
            &identifier_contexts,
            aws_sfn_client.clone(),
        )
        .await?;
    }

    Ok(json!({"statusCode": 200}))
}

async fn handle_sns_event_record(
    s3_event_record: &SnsEventRecord,
    pipeline_id: &'static str,
    identifier_contexts: &EkgIdentifierContexts,
    aws_sfn_client: aws_sdk_sfn::Client,
) -> Result<(), Error> {
    let sns = &s3_event_record.sns;
    tracing::trace!("SNS record: {:#?}", sns);
    // Get the embedded JSON message
    let message = &sns.message;
    tracing::trace!("SNS Message: {:#?}", message);
    // Convert to serde Value first, not straight to S3EventRecords to get better
    // errors
    let s3_event_records_as_value = serde_json::from_str::<Value>(&message)?;
    let s3_event_records = serde_json::from_value::<S3EventRecords>(s3_event_records_as_value)?;
    if s3_event_records.records.len() == 0 {
        return Err(Error::NoInputRecords);
    }
    for s3_event_record in s3_event_records.records {
        handle_s3_event_record(
            s3_event_record,
            pipeline_id,
            &identifier_contexts,
            aws_sfn_client.clone(),
        )
        .await?;
    }
    Ok(())
}

async fn handle_s3_event_record(
    s3_event_record: S3EventRecord,
    pipeline_id: &'static str,
    identifier_contexts: &EkgIdentifierContexts,
    aws_sfn_client: aws_sdk_sfn::Client,
) -> Result<(), Error> {
    tracing::trace!("S3 Event Record: {:#?}", s3_event_record);

    // Convert the S3 event record to a Neptune LoadRequest
    let load_request = ekg_aws_util::neptune::LoadRequest::from_s3_event_record(
        &s3_event_record,
        &identifier_contexts,
    )?;
    // Wrap that Neptune Load Request into an EKG Load Request adding the pipeline
    // ID and the ARN of the Step Function that orchestrates the RDF Load
    let sfn_input = ekg_lfn_load::Request {
        load_request,
        pipeline_id: pipeline_id.to_string(),
        rdf_load_sfn_arn: mandatory_env_var("rdf_load_sfn_arn", None)?,
    };
    tracing::trace!("{:#?}", sfn_input);

    // Kick the Step Function off to start the RDF Load
    StateMachine::new(aws_sfn_client)
        .start_execution(
            mandatory_env_var("rdf_load_sfn_arn", None)?.as_str(),
            serde_json::to_value(sfn_input)?,
        )
        .await?;

    Ok(())
}
