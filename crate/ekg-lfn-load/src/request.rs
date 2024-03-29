use {
    ekg_aws_util::{neptune::LoadRequest, ARN},
    serde::{Deserialize, Serialize},
};

/// Initiate the bulk-loading of a given S3 based RDF file by Neptune
#[derive(Deserialize, Serialize, Debug)]
pub struct Request {
    pub load_request:     LoadRequest,
    pub pipeline_id:      String,
    pub rdf_load_sfn_arn: ARN,
}
