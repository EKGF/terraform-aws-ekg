use {aws_sdk_sfn, ekg_error::Error};

pub struct StateMachine {
    aws_sfn_client: aws_sdk_sfn::Client,
}

impl StateMachine {
    pub fn new(aws_sfn_client: aws_sdk_sfn::Client) -> Self { Self { aws_sfn_client } }

    pub async fn start_execution(
        &self,
        state_machine_arn: &str,
        input: &serde_json::Value,
    ) -> Result<(), Error> {
        let output = self
            .aws_sfn_client
            .start_execution()
            .state_machine_arn(state_machine_arn)
            .input(serde_json::to_string(&input)?)
            .send()
            .await
            .map_err(|err| {
                let msg = format!("Error starting step function: {:}", err);
                tracing::error!(msg);
                Error::ServiceError(msg)
            })?;
        tracing::info!("Step function started: {:}", output.execution_arn);
        Ok(())
    }
}
