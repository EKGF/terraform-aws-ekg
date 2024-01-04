#![cfg(test)]

#[test_log::test(tokio::test)]
async fn test_load_01() -> Result<(), ekg_error::Error> {
    tracing::info!("test_load_01");

    ekg_identifier::EkgIdentifierContexts::default_test();
    std::env::set_var(
        "EKG_SPARQL_LOADER_ENDPOINT",
        "http://localhost:8787/loader",
    );
    std::env::set_var(
        "AWS_NEPTUNE_LOAD_IAM_ROLE_ARN",
        "arn:aws:iam::12345:role/ekgf-dt-dev-neptune-load",
    );
    std::env::set_var("AWS_REGION", "antartica-01");
    let aws_config = aws_config::load_from_env().await;
    let aws_neptunedata_client = crate::get_neptunedata_client(&aws_config)?;
    let event = include_str!("../event.json");
    let request_as_value: serde_json::Value = serde_json::from_str(event).unwrap();
    println!("result: {:#?}", request_as_value);
    let request = serde_json::from_value::<requests::EkgLfnRequest>(request_as_value.clone())?;
    println!("result: {:#?}", request);
    let lambda_output = crate::handle_lambda_payload(&request, aws_neptunedata_client).await?;
    println!("result: {:#?}", lambda_output);
    if let serde_json::Value::Object(map) = lambda_output {
        assert_eq!(map.len(), 1);
        assert!(map.contains_key("statusCode"));
        if let serde_json::Value::Number(result) = map.get("statusCode").unwrap() {
            assert_eq!(result.as_u64(), Some(200u64));
        } else {
            panic!("lambda output statusCode is not a number");
        }
    } else {
        panic!("lambda output is not a JSON object");
    }
    Ok(())
}
