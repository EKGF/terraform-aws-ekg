use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SnsEventRecord {
    pub event_source: String,
    pub event_subscription_arn: String,
    pub event_version: String,
    pub sns: SnsRecord,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SnsRecord {
    #[serde(rename = "Type")]
    pub type_: String,
    pub message_id: String,
    pub topic_arn: String,
    pub subject: String,
    // The stringified JSON message embedded in the SNS message.
    pub message: String,
    pub timestamp: String,
    pub signature_version: String,
    pub signature: String,
    pub signing_cert_url: String,
    pub unsubscribe_url: String,
    pub message_attributes: Value,
}
