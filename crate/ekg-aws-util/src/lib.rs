use serde::Deserialize;

pub use {
    s3::{S3Bucket, S3EventRecord, S3EventRecords, S3Object},
    sns::{SnsEventRecord, SnsRecord},
};

pub mod lambda;
pub mod neptune;
pub mod s3;
pub mod sns;

#[cfg(test)]
mod tests;

pub type ARN = String;
pub type S3URI = String;
pub type Region = String;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserId {
    pub principal_id: String,
}

#[derive(Deserialize, Debug)]
pub struct RequestParameters {
    /// ip-address-where-request-came-from
    #[serde(rename = "sourceIPAddress")]
    pub source_ip_address: String,
}

#[derive(Deserialize, Debug)]
pub struct ResponseElements {
    /// Amazon S3 generated request ID
    #[serde(rename = "x-amz-request-id")]
    pub x_amz_request_id: String,
    /// Amazon S3 host that processed the request
    #[serde(rename = "x-amz-id-2")]
    pub x_amz_id_2: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OwnerIdentity {
    /// Amazon-customer-ID-of-the-bucket-owner
    pub principal_id: String,
}
