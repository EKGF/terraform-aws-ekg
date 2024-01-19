use aws_config::{timeout, SdkConfig};

pub async fn create() -> Result<SdkConfig, ekg_error::Error> {
    let hyper_client = crate::http::hyper_client_builder().await?;

    let timeout_config = timeout::TimeoutConfig::builder()
        .operation_timeout(std::time::Duration::from_secs(60))
        .build();

    let sdk_config = aws_config::from_env()
        .timeout_config(timeout_config)
        .http_client(hyper_client)
        .load()
        .await;

    Ok(sdk_config)
}
