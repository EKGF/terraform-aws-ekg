use {
    aws_smithy_runtime::client::http::hyper_014::HyperClientBuilder,
    aws_smithy_runtime_api::client::http::SharedHttpClient,
};

pub async fn hyper_client_builder() -> Result<SharedHttpClient, ekg_error::Error> {
    let tls_connector = crate::tls_connector::create().await?;

    // See https://github.com/awslabs/smithy-rs/discussions/3022 for the HyperClientBuilder
    Ok(HyperClientBuilder::new().build(tls_connector))
}
