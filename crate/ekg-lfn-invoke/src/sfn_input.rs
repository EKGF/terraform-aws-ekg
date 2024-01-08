use ekg_aws_util::ARN;
use {ekg_aws_util::neptune::LoadRequest, serde::Serialize};

/// The request structure for the AWS Step Function that we invoke.
#[derive(Serialize, Debug)]
pub struct StepFunctionInput {
    pub load_request: LoadRequest,
    pub rdf_load_sfn_arn: ARN,
}

impl StepFunctionInput {
    pub fn from_load_request(load_request: LoadRequest, rdf_load_sfn_arn: ARN) -> Self {
        Self { load_request, rdf_load_sfn_arn }
    }
}
