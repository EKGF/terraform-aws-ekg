use {
    crate::{
        serde_util::{deserialize_format_from_str, serialize_format},
        Region,
        S3EventRecord,
        ARN,
        S3URI,
    },
    ekg_error::Error,
    ekg_identifier::EkgIdentifierContexts,
    ekg_util::{
        env::mandatory_env_var,
        iri::BaseIRI,
        serde_util::{deserialize_bool_as_uppercase, serialize_bool_as_uppercase},
    },
    serde::{Deserialize, Serialize},
};

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
    pub source:                               S3URI,
    #[serde(serialize_with = "serialize_format")]
    #[serde(deserialize_with = "deserialize_format_from_str")]
    pub format:                               aws_sdk_neptunedata::types::Format,
    pub iam_role_arn:                         ARN,
    pub mode:                                 Mode,
    pub region:                               Region,
    #[serde(serialize_with = "serialize_bool_as_uppercase")]
    #[serde(deserialize_with = "deserialize_bool_as_uppercase")]
    pub fail_on_error:                        bool,
    pub parallelism:                          String,
    pub parser_configuration:                 ParserConfiguration,
    #[serde(serialize_with = "serialize_bool_as_uppercase")]
    #[serde(deserialize_with = "deserialize_bool_as_uppercase")]
    pub update_single_cardinality_properties: bool,
    #[serde(serialize_with = "serialize_bool_as_uppercase")]
    #[serde(deserialize_with = "deserialize_bool_as_uppercase")]
    pub queue_request:                        bool,
    pub dependencies:                         Vec<String>,
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
            source:                               s3_uri.clone(),
            format:                               aws_sdk_neptunedata::types::Format::Turtle,
            iam_role_arn:                         mandatory_env_var(
                "AWS_NEPTUNE_LOAD_IAM_ROLE_ARN",
                None,
            )?,
            mode:                                 Mode::NEW,
            region:                               mandatory_env_var("AWS_REGION", None)?,
            fail_on_error:                        true,
            parallelism:                          "OVERSUBSCRIBE".to_string(),
            parser_configuration:                 ParserConfiguration {
                base_uri:            identifier_contexts.internal.ekg_id_base.clone(),
                named_graph_uri:     s3_uri,
                allow_empty_strings: false,
            },
            update_single_cardinality_properties: false,
            queue_request:                        true,
            dependencies:                         vec![],
        })
    }
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
    pub base_uri:            BaseIRI,
    pub named_graph_uri:     String,
    #[serde(serialize_with = "serialize_bool_as_uppercase")]
    #[serde(deserialize_with = "deserialize_bool_as_uppercase")]
    pub allow_empty_strings: bool,
}

impl ParserConfiguration {
    pub fn as_hash_map(&self) -> std::collections::HashMap<String, String> {
        let mut map = std::collections::HashMap::new();
        map.insert("baseUri".to_string(), self.base_uri.as_base_iri());
        map.insert(
            "namedGraphUri".to_string(),
            self.named_graph_uri.to_string(),
        );
        map
    }
}
