#![cfg(test)]

// #[ignore]
#[test_log::test(tokio::test)]
async fn test_load_01() -> Result<(), ekg_error::Error> {
    tracing::info!("test_load_01");

    ekg_identifier::EkgIdentifierContexts::default_test();
    std::env::set_var(
        "EKG_SPARQL_LOADER_ENDPOINT",
        "http://localhost:8787/loader",
    );
    std::env::set_var(
        "EKG_SPARQL_QUERY_ENDPOINT",
        "http://localhost:8787/sparql",
    );
    std::env::set_var(
        "EKG_SPARQL_UPDATE_ENDPOINT",
        "http://localhost:8787/sparql",
    );
    std::env::set_var(
        "AWS_NEPTUNE_LOAD_IAM_ROLE_ARN",
        "arn:aws:iam::12345:role/ekgf-dt-dev-neptune-load",
    );
    std::env::set_var("AWS_REGION", "antartica-01");
    let aws_config = aws_config::load_from_env().await;
    let clients = crate::Clients {
        // Create the NeptuneData client
        aws_neptunedata_client: ekg_aws_util::neptune::get_neptunedata_client(&aws_config)?,
        // Create the HTTP SPARQL client (which strangely enough is not part of the
        // aws_sdk_neptunedata or aws_sdk_neptune crates, we had to build one ourselves)
        sparql_client:          ekg_sparql::SPARQLClient::from_env().await?,
    };
    let event = include_str!("../event.json");
    let request_as_value: serde_json::Value = serde_json::from_str(event).unwrap();
    println!("result: {:#?}", request_as_value);
    let request = serde_json::from_value::<crate::Request>(request_as_value.clone())?;
    println!("result: {:#?}", request);
    let lambda_output = crate::handle_lambda_request(&request, clients).await?;
    println!("result: {:#?}", lambda_output);
    assert_eq!(lambda_output.status_code, 500);
    assert_eq!(
        lambda_output.message,
        "Dispatch failure: io error"
    );
    Ok(())
}
