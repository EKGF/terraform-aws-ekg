use aws_config::timeout::TimeoutConfig;
use aws_types::region::Region;

use ekg_error::Error;
use ekg_util::env::mandatory_env_var;

pub fn get_neptunedata_client_config(
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

pub fn get_neptunedata_client(
    aws_sdk_config: &aws_types::SdkConfig,
) -> Result<aws_sdk_neptunedata::Client, Error> {
    // tracing::info!("aws_sdk_config: {:#?}", aws_sdk_config);

    let neptune_client_config = get_neptunedata_client_config(aws_sdk_config)?;
    Ok(aws_sdk_neptunedata::Client::from_conf(
        neptune_client_config,
    ))
}
