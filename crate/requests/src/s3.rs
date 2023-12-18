use serde::Deserialize;

use crate::{OwnerIdentity, RequestParameters, ResponseElements, UserId};

/// The embedded JSON message structure.
/// For example:
/// {
///     "Records": [
///         {
///             "eventVersion": "2.1",
///             "eventSource": "aws:s3",
///             "awsRegion": "eu-west-2",
///             "eventTime": "2023-09-18T10:03:15.979Z",
///             "eventName": "ObjectCreated:Put",
///             "userIdentity": {
///                 "principalId": "AWS:AIDAWVGREJ265Q72HOJUP"
///             },
///             "requestParameters": {
///                 "sourceIPAddress": "193.237.90.75"
///             },
///             "responseElements": {
///                 "x-amz-request-id": "JJ807NMA5B2VMJ0D",
///                 "x-amz-id-2": "wSZ0gf3XaMj63uKcY7A43KSJ3fAMm27hZcWZQRTNzhFIq4oaTZ7fO1RaIL35VbG3g9LIU/B6+IuDLN1N1lAoeJapphdeOaTu"
///             },
///             "s3": {
///                 "s3SchemaVersion":"1.0",
///                 "configurationId": "tf-s3-topic-20230915095940816500000001",
///                 "bucket": {
///                     "name": "ekgf-dt-dev-metadata",
///                     "ownerIdentity": {
///                         "principalId": "A1M8OTUP4LUCQC"
///                     },
///                     "arn": "arn:aws:s3:::ekgf-dt-dev-metadata"
///                 },
///                 "object": {
///                     "key": "static-dataset/personas/ekgf-group-internal-auditor.ttl",
///                     "size": 1206,
///                     "eTag": "455c556f7d1b7f8587ecabe2dd8184af",
///                     "versionId": "LBK4atYjFZR7h5v_.bUVAuWLbYpwCeB2",
///                     "sequencer": "0065082063F0F5766D"
///                 }
///             }
///         }
///     ]
/// }
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct S3EventRecords {
    pub records: Vec<S3EventRecord>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct S3EventRecord {
    pub event_source: String,
    pub event_version: String,
    pub aws_region: String,
    pub event_time: String,
    pub event_name: String,
    pub user_identity: UserId,
    pub request_parameters: RequestParameters,
    pub response_elements: ResponseElements,
    pub s3: S3,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct S3 {
    pub s3_schema_version: String,
    pub configuration_id: String,
    pub bucket: S3Bucket,
    pub object: S3Object,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct S3Bucket {
    pub name: String,
    pub owner_identity: OwnerIdentity,
    pub arn: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct S3Object {
    pub key: String,
    pub size: u64,
    pub e_tag: String,
    pub version_id: String,
    pub sequencer: String,
}
