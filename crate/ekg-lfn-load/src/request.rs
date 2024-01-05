use ekg_aws_util::neptune::AwsNeptuneLoadRequest;
use {
    ekg_aws_util::ARN,
    serde::{Deserialize, Serialize},
};

/// Initiate the bulk-loading of a given S3 based RDF file by Neptune
#[derive(Deserialize, Serialize, Debug)]
pub struct Request {
    pub load_request: AwsNeptuneLoadRequest,
    pub rdf_load_sfn_arn: ARN,
}
