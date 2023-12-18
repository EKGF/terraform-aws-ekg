/// See https://github.com/awslabs/aws-lambda-rust-runtime for more info on Rust runtime for AWS Lambda
use lambda_runtime::{service_fn, Error, LambdaEvent};
use {
    ekg_identifier::EkgIdentifierContexts,
    requests::{InvokeRequest, S3EventRecord, S3EventRecords, SnsEventRecord},
    serde::Serialize,
    serde_json::{json, Value},
};

#[cfg(test)]
mod tests;

/// A simple Lambda response structure.
#[derive(Serialize)]
struct Response {
    req_id: String,
    msg:    String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    // call the actual handler of the request
    let func = service_fn(handle_lambda_event);
    lambda_runtime::run(func).await?;
    Ok(())
}

/// The actual handler of the Lambda request.
#[tracing::instrument]
pub(crate) async fn handle_lambda_event(event: LambdaEvent<Value>) -> Result<Value, Error> {
    // tracing::info!("XXXXXX Event XXXX {:#?}\n\n", event.clone());
    let (payload, _ctx) = event.into_parts();

    tracing::info!("XXXXXX Payload XXXX {:#?}\n\n", payload);

    let request = serde_json::from_value::<requests::InvokeRequest>(payload).map_err(|e| {
        tracing::error!("Error parsing request: {}", e);
        e
    })?;

    handle_lambda_payload(&request).await
}

pub(crate) async fn handle_lambda_payload(request: &InvokeRequest) -> Result<Value, Error> {
    let identifier_contexts = ekg_identifier::EkgIdentifierContexts::from_env()?;

    for record in &request.records {
        handle_sns_event_record(&record, &identifier_contexts).await?;
    }

    Ok(json!({
        "statusCode": 200,
        "body": json!({
            "message": "Hello from Rust!",
        }),
    }))
}

async fn handle_sns_event_record(
    s3_event_record: &SnsEventRecord,
    identifier_contexts: &EkgIdentifierContexts,
) -> Result<Value, Error> {
    let sns = &s3_event_record.sns;
    tracing::info!("XXXXXX SNS XXXX {:#?}\n\n", sns);
    // Get the embedded JSON message
    let message = &sns.message;
    tracing::info!("XXXXXX SNS Message XXXX {:#?}\n\n", message);
    // Convert to serde Value first, not straight to S3EventRecords to get better
    // errors
    let s3_event_records_as_value = serde_json::from_str::<Value>(&message)?;
    println!(
        "s3_event_records_as_value: {:?}",
        s3_event_records_as_value
    );
    let s3_event_records = serde_json::from_value::<S3EventRecords>(s3_event_records_as_value)?;
    for s3_event_record in s3_event_records.records {
        handle_s3_event_record(s3_event_record, &identifier_contexts).await?;
    }
    Ok(Value::Bool(true))
}

async fn handle_s3_event_record(
    s3_event_record: S3EventRecord,
    _identifier_contexts: &EkgIdentifierContexts,
) -> Result<Value, Error> {
    println!("S3 Event Record: {:#?}", s3_event_record);
    Ok(Value::Bool(true))
}
