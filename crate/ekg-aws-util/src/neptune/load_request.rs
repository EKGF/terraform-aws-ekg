use serde::{Deserialize, Serialize};

use ekg_error::Error;
use ekg_identifier::EkgIdentifierContexts;
use ekg_util::env::mandatory_env_var;

use crate::{Region, S3EventRecord, ARN, S3URI};

/// AWS Neptune Load Request
/// See https://docs.aws.amazon.com/neptune/latest/userguide/load-api-reference-load.html
///
/// We do not support a source string that is a list of S3 objects because we
/// want to load each individual file into its own named graph initially, where
/// the IRI of the named graph is equal to the given S3 URL. Then after the load
/// we can merge the named graphs into a single graph and record the original S3
/// URL as the source of the triples for proper lineage/provenance purposes.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadRequest {
    pub source: S3URI,
    #[serde(serialize_with = "serialize_format")]
    #[serde(deserialize_with = "deserialize_format_from_str")]
    pub format: aws_sdk_neptunedata::types::Format,
    pub iam_role_arn: ARN,
    pub mode: Mode,
    pub region: Region,
    pub fail_on_error: String,
    pub parallelism: String,
    pub parser_configuration: ParserConfiguration,
    pub update_single_cardinality_properties: String,
    pub queue_request: bool,
    pub dependencies: Vec<String>,
}

impl LoadRequest {
    pub fn from_s3_event_record(
        s3_event_record: &S3EventRecord,
        identifier_contexts: &EkgIdentifierContexts,
    ) -> Result<Self, Error> {
        let s3_uri = format!(
            "s3://{}/{}",
            s3_event_record.s3.bucket.name,
            s3_event_record.s3.object.key.clone()
        );
        Ok(Self {
            source: s3_uri.clone(),
            format: aws_sdk_neptunedata::types::Format::Turtle,
            iam_role_arn: mandatory_env_var("AWS_NEPTUNE_LOAD_IAM_ROLE_ARN", None)?,
            mode: Mode::AUTO,
            region: mandatory_env_var("AWS_REGION", None)?,
            fail_on_error: "TRUE".to_string(),
            parallelism: "MEDIUM".to_string(),
            parser_configuration: ParserConfiguration {
                base_uri: identifier_contexts.internal.ekg_id_base.clone(),
                named_graph_uri: s3_uri,
            },
            update_single_cardinality_properties: "FALSE".to_string(),
            queue_request: true,
            dependencies: vec![],
        })
    }
}

fn serialize_format<S>(
    format: &aws_sdk_neptunedata::types::Format,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(format.as_str())
}

/// Deserialize a type `S` by deserializing a string, then using the `FromStr`
/// impl of `S` to create the result. The generic type `S` is not required to
/// implement `Deserialize`.
fn deserialize_format_from_str<'de, D>(
    deserializer: D,
) -> Result<aws_sdk_neptunedata::types::Format, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    aws_sdk_neptunedata::types::Format::try_parse(s.as_str()).map_err(serde::de::Error::custom)
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "UPPERCASE")]
#[allow(dead_code)]
pub enum Mode {
    NEW,
    RESUME,
    AUTO,
}

impl Into<aws_sdk_neptunedata::types::Mode> for Mode {
    fn into(self) -> aws_sdk_neptunedata::types::Mode {
        match self {
            Mode::NEW => aws_sdk_neptunedata::types::Mode::New,
            Mode::RESUME => aws_sdk_neptunedata::types::Mode::Resume,
            Mode::AUTO => aws_sdk_neptunedata::types::Mode::Auto,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParserConfiguration {
    pub base_uri: String,
    pub named_graph_uri: String,
}

impl ParserConfiguration {
    pub fn as_hash_map(&self) -> std::collections::HashMap<String, String> {
        let mut map = std::collections::HashMap::new();
        map.insert("baseUri".to_string(), self.base_uri.clone());
        map.insert(
            "namedGraphUri".to_string(),
            self.named_graph_uri.clone(),
        );
        map
    }
}
