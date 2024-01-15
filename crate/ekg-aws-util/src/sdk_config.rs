use aws_config::SdkConfig;

pub async fn create() -> Result<SdkConfig, ekg_error::Error> {
    let hyper_client = crate::http::hyper_client_builder().await?;

    let sdk_config = aws_config::from_env()
        .http_client(hyper_client)
        .load()
        .await;

    Ok(sdk_config)
}
