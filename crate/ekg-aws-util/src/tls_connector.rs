//! Create an AWS SDK specific TLS 1.3 connector to be used with Hyper, AWS SDK,
//! etc. This is very similar to ekg_util::tls_connector but it uses the
//! HttpConnector from the aws-smithy-runtime crate instead of the rustls crate.
use {hyper::client::HttpConnector, hyper_rustls::HttpsConnector};

/// Create a TLS 1.3 connector to be used with Hyper, AWS SDK, etc.
/// This is very similar to ekg_util::tls_connector but it uses the
/// HttpConnector from the aws-smithy-runtime crate instead of the rustls crate.
pub async fn create() -> Result<HttpsConnector<HttpConnector>, ekg_error::Error> {
    tracing::info!("Attempting to create a TLS 1.3 connector:");

    let tls_config = ekg_util::tls_config::create().await?;

    // Finish setup of the Hyper connector.
    let hyper_connector = hyper_rustls::HttpsConnectorBuilder::new()
        .with_tls_config(tls_config)
        .https_only()
        .enable_http2()
        .build();

    Ok(hyper_connector)
}

// fn default_tls() -> HttpsConnector<HttpConnector> {
//     use hyper_rustls::ConfigBuilderExt;
//     hyper_rustls::HttpsConnectorBuilder::new()
//         .with_tls_config(
//             rustls::ClientConfig::builder()
//                 .with_cipher_suites(&[
//                     // TLS1.3 suites
//                     rustls::cipher_suite::TLS13_AES_256_GCM_SHA384,
//                     rustls::cipher_suite::TLS13_AES_128_GCM_SHA256,
//                     // TLS1.2 suites
//
// rustls::cipher_suite::TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384,
// rustls::cipher_suite::TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256,
// rustls::cipher_suite::TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384,
// rustls::cipher_suite::TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256,
// rustls::cipher_suite::TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256,
//                 ])
//                 .with_safe_default_kx_groups()
//                 .with_safe_default_protocol_versions()
//                 .expect("Error with the TLS configuration. Please file a bug report under https://github.com/smithy-lang/smithy-rs/issues.")
//                 .with_native_roots()
//                 .with_no_client_auth()
//         )
//         .https_or_http()
//         .enable_http1()
//         .enable_http2()
//         .build()
// }
