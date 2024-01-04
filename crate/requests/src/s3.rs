use {
    crate::{OwnerIdentity, RequestParameters, ResponseElements, UserId},
    serde::Deserialize,
};

/// The embedded JSON message structure.
/// For example:
/// {
///     "Records": [
///         {
///             "eventVersion": "2.1",
///             "eventSource": "aws:s3",
///             "awsRegion": "antartica-01",
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
///                 "x-amz-id-2":
/// "wSZ0gf3XaMj63uKcY7A43KSJ3fAMm27hZcWZQRTNzhFIq4oaTZ7fO1RaIL35VbG3g9LIU/
/// B6+IuDLN1N1lAoeJapphdeOaTu"             },
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
///                     "key":
/// "static-dataset/personas/ekgf-group-internal-auditor.ttl",                  
/// "size": 1206,                     "eTag":
/// "455c556f7d1b7f8587ecabe2dd8184af",                     "versionId":
/// "LBK4atYjFZR7h5v_.bUVAuWLbYpwCeB2",                     "sequencer":
/// "0065082063F0F5766D"                 }
///             }
///         }
///     ]
/// }
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct S3EventRecords {
    pub records: Vec<S3EventRecord>,
}

/// See https://docs.aws.amazon.com/AmazonS3/latest/userguide/notification-content-structure.html
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct S3EventRecord {
    pub event_source:       String,
    pub event_version:      String,
    pub aws_region:         String,
    /// The time when Amazon S3 finished processing the request
    pub event_time:         String,
    /// The `event_name` references the list of [event notification
    /// types](https://docs.aws.amazon.com/AmazonS3/latest/userguide/notification-how-to-event-types-and-destinations.html)
    /// but doesn't contain the s3: prefix.
    /// TODO: Convert to enum
    pub event_name:         String,
    /// User who caused the event
    pub user_identity:      UserId,
    pub request_parameters: RequestParameters,
    pub response_elements:  ResponseElements,
    pub s3:                 S3,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct S3 {
    pub s3_schema_version: String,
    /// ID found in the bucket notification configuration
    pub configuration_id:  String,
    pub bucket:            S3Bucket,
    pub object:            S3Object,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct S3Bucket {
    /// Bucket name
    pub name:           String,
    pub owner_identity: OwnerIdentity,
    /// Bucket ARN
    pub arn:            String,
}

/// See https://docs.aws.amazon.com/AmazonS3/latest/userguide/notification-content-structure.html
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct S3Object {
    /// Object key
    pub key:        String,
    /// Size in bytes
    pub size:       u64,
    /// Object eTag
    pub e_tag:      String,
    /// object version if bucket is versioning-enabled, otherwise null
    pub version_id: Option<String>,
    /// a string representation of a hexadecimal value used to
    /// determine event sequence, only used with PUTs and DELETEs
    pub sequencer:  String,
}
