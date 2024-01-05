#![cfg(test)]

use {ekg_error::Error, ekg_identifier::EkgIdentifierContexts};

#[test_log::test(tokio::test)]
async fn test_invoke_01() -> Result<(), Error> {
    tracing::info!("test_invoke_01");
    let aws_config = aws_config::load_from_env().await;
    let aws_sfn_client = aws_sdk_sfn::Client::new(&aws_config);

    EkgIdentifierContexts::default_test();
    std::env::set_var(
        "AWS_NEPTUNE_LOAD_IAM_ROLE_ARN",
        "arn:aws:iam::12345:role/ekgf-dt-dev-neptune-load",
    );
    std::env::set_var("AWS_REGION", "antartica-01");
    std::env::set_var(
        "rdf_load_sfn_arn",
        "arn:aws:states:antartica-01:123456789012:stateMachine:rdf_load",
    );
    let event = r#"{
          "Records": [
            {
              "EventSource": "aws:sns",
              "EventVersion": "1.0",
              "EventSubscriptionArn": "arn:aws:sns:antartica-01:123456789012:rdf_load:3b82635e-59ba-41a6-8c9a-d7c0cb697fc4",
              "Sns": {
                "Type": "Notification",
                "MessageId": "642a53e8-260d-55e9-8bbc-0e6a04a9b18a",
                "TopicArn": "arn:aws:sns:antartica-01:123456789012:rdf_load",
                "Subject": "Amazon S3 Notification",
                "Message": "{\"Records\":[{\"eventVersion\":\"2.1\",\"eventSource\":\"aws:s3\",\"awsRegion\":\"antartica-01\",\"eventTime\":\"2023-09-18T10:03:15.979Z\",\"eventName\":\"ObjectCreated:Put\",\"userIdentity\":{\"principalId\":\"AWS:AIDAWVGREJ265Q72HOJUP\"},\"requestParameters\":{\"sourceIPAddress\":\"193.237.90.75\"},\"responseElements\":{\"x-amz-request-id\":\"JJ807NMA5B2VMJ0D\",\"x-amz-id-2\":\"wSZ0gf3XaMj63uKcY7A43KSJ3fAMm27hZcWZQRTNzhFIq4oaTZ7fO1RaIL35VbG3g9LIU/B6+IuDLN1N1lAoeJapphdeOaTu\"},\"s3\":{\"s3SchemaVersion\":\"1.0\",\"configurationId\":\"tf-s3-topic-20230915095940816500000001\",\"bucket\":{\"name\":\"ekgf-dt-dev-metadata\",\"ownerIdentity\":{\"principalId\":\"A1M8OTUP4LUCQC\"},\"arn\":\"arn:aws:s3:::ekgf-dt-dev-metadata\"},\"object\":{\"key\":\"static-dataset/personas/ekgf-group-internal-auditor.ttl\",\"size\":1206,\"eTag\":\"455c556f7d1b7f8587ecabe2dd8184af\",\"versionId\":\"LBK4atYjFZR7h5v_.bUVAuWLbYpwCeB2\",\"sequencer\":\"0065082063F0F5766D\"}}}]}",
                "Timestamp": "2023-09-18T10:03:16.801Z",
                "SignatureVersion": "1",
                "Signature": "Gk7YCHZkRtzgMWJL7m8bC5Yuit6ph/Mor90UB62QFY1LvITw6IvWi9jTp9v4UC/IRU2/os6ofhh09rMst39pAqqH4Tz47LqoL53SdPCcVvaSFWIrRtbFO3Gi89L2nMCO0Kis49sWc783WSMQnju100AXjzKR7eiwSHzaQzZhrxD71pl67q9m1oB7HaWzTLyV8mpcsbJYnDqchyNZjOrbbYua+VeV6FShbMAuq482rf59dyiakj/VziByp2o0gjQf/9QtwXdOB+HiWSWabtrmcmVXZUzoZvuKnaq0UAnPKelL1AOeu2Nw2a067oYFRcoIwX/izWWGNm4bw3euqUQq7A==",
                "SigningCertUrl": "https://sns.antartica-01.amazonaws.com/SimpleNotificationService-01d088a6f77103d0fe307c0069e40ed6.pem",
                "UnsubscribeUrl": "https://sns.antartica-01.amazonaws.com/?Action=Unsubscribe&SubscriptionArn=arn:aws:sns:antartica-01:123456789012:rdf_load:3b82635e-59ba-41a6-8c9a-d7c0cb697fc4",
                "MessageAttributes": {}
              }
            }
          ]
        }"#;
    let request_as_value: serde_json::Value = serde_json::from_str(event).unwrap();
    println!("result: {:#?}", request_as_value);
    let lambda_output = crate::handle_lambda_payload(request_as_value, aws_sfn_client).await?;
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
