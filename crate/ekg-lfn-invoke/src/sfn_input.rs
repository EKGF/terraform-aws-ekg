use {
    requests::{AwsARN, AwsNeptuneLoadRequest},
    serde::Serialize,
};

/// The request structure for the AWS Step Function that we invoke.
#[derive(Serialize, Debug)]
pub struct StepFunctionInput {
    pub load_request:     AwsNeptuneLoadRequest,
    pub rdf_load_sfn_arn: AwsARN,
}

impl StepFunctionInput {
    pub fn from_load_request(
        load_request: AwsNeptuneLoadRequest,
        rdf_load_sfn_arn: AwsARN,
    ) -> Self {
        Self { load_request, rdf_load_sfn_arn }
    }
}
