use serde::Deserialize;

pub use invoke_request::InvokeRequest;
pub use s3::S3Bucket;
pub use s3::S3EventRecord;
pub use s3::S3EventRecords;
pub use s3::S3Object;
pub use sns::SnsEventRecord;
pub use sns::SnsRecord;

mod invoke_request;
mod sns;
mod s3;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserId {
    pub principal_id: String,
}

#[derive(Deserialize, Debug)]
pub struct RequestParameters {
    #[serde(rename = "sourceIPAddress")]
    pub source_ip_address: String,
}

#[derive(Deserialize, Debug)]
pub struct ResponseElements {
    #[serde(rename = "x-amz-request-id")]
    pub x_amz_request_id: String,
    #[serde(rename = "x-amz-id-2")]
    pub x_amz_id_2: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OwnerIdentity {
    pub principal_id: String,
}

#[cfg(test)]
mod tests {
    #[test_log::test]
    fn invoke() {
        let event = r#"{
          "Records": [
            {
              "EventSource": "aws:sns",
              "EventVersion": "1.0",
              "EventSubscriptionArn": "arn:aws:sns:eu-west-2:457852604093:rdf_load:3b82635e-59ba-41a6-8c9a-d7c0cb697fc4",
              "Sns": {
                "Type": "Notification",
                "MessageId": "642a53e8-260d-55e9-8bbc-0e6a04a9b18a",
                "TopicArn": "arn:aws:sns:eu-west-2:457852604093:rdf_load",
                "Subject": "Amazon S3 Notification",
                "Message": "{\"Records\":[{\"eventVersion\":\"2.1\",\"eventSource\":\"aws:s3\",\"awsRegion\":\"eu-west-2\",\"eventTime\":\"2023-09-18T10:03:15.979Z\",\"eventName\":\"ObjectCreated:Put\",\"userIdentity\":{\"principalId\":\"AWS:AIDAWVGREJ265Q72HOJUP\"},\"requestParameters\":{\"sourceIPAddress\":\"193.237.90.75\"},\"responseElements\":{\"x-amz-request-id\":\"JJ807NMA5B2VMJ0D\",\"x-amz-id-2\":\"wSZ0gf3XaMj63uKcY7A43KSJ3fAMm27hZcWZQRTNzhFIq4oaTZ7fO1RaIL35VbG3g9LIU/B6+IuDLN1N1lAoeJapphdeOaTu\"},\"s3\":{\"s3SchemaVersion\":\"1.0\",\"configurationId\":\"tf-s3-topic-20230915095940816500000001\",\"bucket\":{\"name\":\"ekgf-dt-dev-metadata\",\"ownerIdentity\":{\"principalId\":\"A1M8OTUP4LUCQC\"},\"arn\":\"arn:aws:s3:::ekgf-dt-dev-metadata\"},\"object\":{\"key\":\"static-dataset/personas/ekgf-group-internal-auditor.ttl\",\"size\":1206,\"eTag\":\"455c556f7d1b7f8587ecabe2dd8184af\",\"versionId\":\"LBK4atYjFZR7h5v_.bUVAuWLbYpwCeB2\",\"sequencer\":\"0065082063F0F5766D\"}}}]}",
                "Timestamp": "2023-09-18T10:03:16.801Z",
                "SignatureVersion": "1",
                "Signature": "Gk7YCHZkRtzgMWJL7m8bC5Yuit6ph/Mor90UB62QFY1LvITw6IvWi9jTp9v4UC/IRU2/os6ofhh09rMst39pAqqH4Tz47LqoL53SdPCcVvaSFWIrRtbFO3Gi89L2nMCO0Kis49sWc783WSMQnju100AXjzKR7eiwSHzaQzZhrxD71pl67q9m1oB7HaWzTLyV8mpcsbJYnDqchyNZjOrbbYua+VeV6FShbMAuq482rf59dyiakj/VziByp2o0gjQf/9QtwXdOB+HiWSWabtrmcmVXZUzoZvuKnaq0UAnPKelL1AOeu2Nw2a067oYFRcoIwX/izWWGNm4bw3euqUQq7A==",
                "SigningCertUrl": "https://sns.eu-west-2.amazonaws.com/SimpleNotificationService-01d088a6f77103d0fe307c0069e40ed6.pem",
                "UnsubscribeUrl": "https://sns.eu-west-2.amazonaws.com/?Action=Unsubscribe&SubscriptionArn=arn:aws:sns:eu-west-2:457852604093:rdf_load:3b82635e-59ba-41a6-8c9a-d7c0cb697fc4",
                "MessageAttributes": {}
              }
            }
          ]
        }"#;
        let request_as_value: serde_json::Value = serde_json::from_str(event).unwrap();
        println!("result: {:#?}", request_as_value);
        let request = serde_json::from_value::<super::InvokeRequest>(request_as_value.clone()).unwrap();
        println!("result: {:#?}", request);
        for record in request.records {
            let sns = record.sns;
            tracing::info!("XXXXXX SNS XXXX {:#?}\n\n", sns);
            // Get the embedded JSON message
            let s3_event_record_as_value = sns.message;
            tracing::info!("XXXXXX S3 Event Record 1 XXXX {:#?}", s3_event_record_as_value);
            let s3_event_record = serde_json::from_str::<S3EventRecords>(&s3_event_record_as_value).unwrap();
            tracing::info!("XXXXXX S3 Event Record 2 XXXX {:#?}", s3_event_record);
        }
    }
}
