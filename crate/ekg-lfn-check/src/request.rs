use ekg_aws_util::neptune::LoadRequest;
use {
    ekg_aws_util::lambda::LambdaResponse,
    ekg_aws_util::ARN,
    serde::{Deserialize, Serialize},
};

/// Initiate the check with the Amazon Neptune bulk loader whether a given
/// bulk load request has finished.
#[derive(Deserialize, Serialize, Debug)]
pub struct Request {
    pub load_request: LoadRequest,
    pub rdf_load_sfn_arn: ARN,
    pub load_output: LambdaResponse,
}
