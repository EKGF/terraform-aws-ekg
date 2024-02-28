use {
    crate::lambda::LambdaDetailStatus,
    aws_sdk_neptunedata::{
        error::SdkError,
        operation::{
            get_loader_job_status::GetLoaderJobStatusError,
            start_loader_job::{StartLoaderJobError, StartLoaderJobOutput},
        },
        types::error::BadRequestException,
    },
    aws_smithy_runtime_api::{
        client::result::{
            ConstructionFailure,
            DispatchFailure,
            ResponseError,
            ServiceError,
            TimeoutError,
        },
        http::Response,
    },
    rand::Rng,
    serde::{Deserialize, Serialize},
    std::error::Error,
};

/// Generic response type that suits most of our lambda functions
#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct LambdaResponse {
    pub status_code:             u16,
    pub message:                 String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_message:        Option<String>,
    pub detail_status:           LambdaDetailStatus,
    /// A generic slot that can be used to pass back a result identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_identifier:       Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_retry_seconds: Option<u16>,
}

const MIN_RETRY_WAIT_SECONDS: u16 = 10;
const MAX_RETRY_WAIT_SECONDS: u16 = 60;

impl LambdaResponse {
    /// deprecated
    pub fn clean(&mut self) {
        // Remove the detailed message if it is the same as the message
        if Some(self.message.as_str()) == self.detailed_message.as_deref() {
            self.detailed_message = None;
        }
        // Suggest a random number of seconds to wait before retrying the request
        // in case of a recognized error. Since the Neptune loader queue has 64 slots
        // we don't want all hundreds of other requests to wait the same amount of time
        // before trying again adding their requests to that limited queue.
        let mut rng = rand::thread_rng();
        self.suggested_retry_seconds =
            Some(rng.gen_range(MIN_RETRY_WAIT_SECONDS..MAX_RETRY_WAIT_SECONDS))
    }

    pub fn retryable(self) -> Self {
        Self {
            // Remove the detailed message if it is the same as the message
            detailed_message: if Some(self.message.as_str()) == self.detailed_message.as_deref() {
                None
            } else {
                self.detailed_message
            },
            suggested_retry_seconds: {
                // Suggest a random number of seconds to wait before retrying the request
                // in case of a recognized error. Since the Neptune loader queue has 64 slots
                // we don't want all hundreds of other requests to wait the same amount of time
                // before trying again adding their requests to that limited queue.
                let mut rng = rand::thread_rng();
                Some(rng.gen_range(MIN_RETRY_WAIT_SECONDS..MAX_RETRY_WAIT_SECONDS))
            },
            ..self
        }
    }

    pub fn pipeline_id_not_matching(
        received_pipeline_id: &str,
        required_pipeline_id: &str,
    ) -> Self {
        Self {
            status_code: 400,
            message: format!(
                "Pipeline ID not matching (received: {}, required: {}",
                received_pipeline_id, required_pipeline_id
            ),
            detail_status: LambdaDetailStatus::PipelineIdNotMatching,
            ..Default::default()
        }
    }

    pub fn ok(detail_status: LambdaDetailStatus, detailed_message: Option<&str>) -> Self {
        let retryable = detail_status.is_retryable();
        tracing::info!(
            "{}{}{}",
            &detail_status.message(),
            if retryable { " (will retry later)" } else { "" },
            detailed_message
                .map(|s| format!(", {}", s))
                .unwrap_or_default()
        );
        let response = Self {
            status_code: 200,
            message: detail_status.message().to_string(),
            detailed_message: detailed_message.map(|s| s.to_string()),
            detail_status,
            ..Default::default()
        };
        if retryable {
            response.retryable()
        } else {
            response
        }
    }
}

impl From<&BadRequestException> for LambdaResponse {
    fn from(error: &BadRequestException) -> Self {
        Self {
            status_code: 400,
            message: error
                .message
                .clone()
                .unwrap_or("unknown message".to_string()),
            detailed_message: Some(error.detailed_message.clone()),
            detail_status: LambdaDetailStatus::from_bad_request_exception(error)
                .unwrap_or(LambdaDetailStatus::UserError),
            ..Default::default()
        }
    }
}

impl From<&StartLoaderJobError> for LambdaResponse {
    fn from(error: &StartLoaderJobError) -> Self {
        tracing::error!("Unknown service error: {:?}", error);
        Self {
            status_code: 500,
            message: format!("Service Error: {:?}", error),
            ..Default::default()
        }
    }
}

impl From<ConstructionFailure> for LambdaResponse {
    fn from(error: ConstructionFailure) -> Self {
        let msg = format!("Construction failure: {:?}", error);
        tracing::error!(msg);
        Self {
            status_code: 504,
            message: msg,
            ..Default::default()
        }
    }
}

impl From<TimeoutError> for LambdaResponse {
    fn from(error: TimeoutError) -> Self {
        let msg = format!("Timeout Error: {:?}", error);
        tracing::error!(msg);
        Self {
            status_code: 504,
            message: msg,
            detail_status: LambdaDetailStatus::Timedout,
            ..Default::default()
        }
        .retryable()
    }
}

impl From<DispatchFailure> for LambdaResponse {
    fn from(error: DispatchFailure) -> Self {
        let cause = error.as_connector_error().unwrap();
        let msg = format!("Dispatch failure: {cause}");
        tracing::error!(msg);
        let detail_status = if cause.is_timeout() {
            LambdaDetailStatus::Timedout
        } else if cause.is_io() {
            LambdaDetailStatus::IOError
        } else if cause.is_user() {
            LambdaDetailStatus::UserError
        } else {
            LambdaDetailStatus::LoaderJobUnexpectedError
        };
        Self {
            status_code: 500,
            message: msg,
            detailed_message: Some(format!("{}", cause.source().unwrap())),
            detail_status,
            ..Default::default()
        }
    }
}

// impl<R> From<ResponseError<R>> for LambdaResponse {
//     fn from(error: ResponseError<R>) -> Self {
//         let msg = format!("Response error");
//         tracing::error!(msg);
//         Self {
//             status_code: 500,
//             message: msg,
//             ..Default::default()
//         }
//     }
// }

impl From<ResponseError<Response>> for LambdaResponse {
    fn from(error: ResponseError<Response>) -> Self {
        let msg = format!("Response error: {:?}", error);
        tracing::error!(msg);
        Self {
            status_code: 500,
            message: msg,
            ..Default::default()
        }
    }
}

impl<E: std::fmt::Debug, R> From<ServiceError<E, R>> for LambdaResponse {
    fn from(error: ServiceError<E, R>) -> Self {
        let service_error = error.into_err();
        tracing::error!("Known service error: {:?}", service_error);
        Self {
            status_code: 500,
            message: format!("Service Error: {:?}", service_error),
            ..Default::default()
        }
    }
}

impl From<SdkError<StartLoaderJobError, Response>> for LambdaResponse {
    fn from(error: SdkError<StartLoaderJobError, Response>) -> Self {
        match error {
            SdkError::ServiceError(service_error) => {
                let service_error = service_error.into_err();
                match service_error {
                    StartLoaderJobError::BadRequestException(exc) => return exc.into(),
                    _ => (),
                }
                service_error.into()
            },
            SdkError::TimeoutError(timeout_error) => timeout_error.into(),
            SdkError::DispatchFailure(dispatch_failure) => dispatch_failure.into(),
            SdkError::ResponseError(response_error) => response_error.into(),
            SdkError::ConstructionFailure(construction_failure) => construction_failure.into(),
            _ => todo!(),
        }
    }
}

impl From<BadRequestException> for LambdaResponse {
    fn from(error: BadRequestException) -> Self {
        Self {
            status_code: 400,
            message: error
                .message
                .clone()
                .unwrap_or("unknown message".to_string()),
            detailed_message: Some(error.detailed_message.clone()),
            detail_status: LambdaDetailStatus::from_bad_request_exception(&error)
                .unwrap_or(LambdaDetailStatus::UserError),
            ..Default::default()
        }
    }
}

impl From<&StartLoaderJobOutput> for LambdaResponse {
    fn from(loader_job: &StartLoaderJobOutput) -> Self {
        let load_id = loader_job.payload.get("loadId").cloned();
        Self {
            status_code: 200,
            message: "Loader job started successfully".to_string(),
            result_identifier: load_id,
            detail_status: LambdaDetailStatus::LoaderJobInQueue,
            suggested_retry_seconds: None,
            ..Default::default()
        }
    }
}

impl From<SdkError<GetLoaderJobStatusError, Response>> for LambdaResponse {
    fn from(error: SdkError<GetLoaderJobStatusError, Response>) -> Self {
        match error {
            SdkError::ServiceError(service_error) => service_error.into(),
            SdkError::TimeoutError(timeout_error) => timeout_error.into(),
            SdkError::DispatchFailure(dispatch_failure) => dispatch_failure.into(),
            SdkError::ResponseError(response_error) => response_error.into(),
            SdkError::ConstructionFailure(construction_failure) => construction_failure.into(),
            _ => todo!(),
        }
    }
}

impl From<&GetLoaderJobStatusError> for LambdaResponse {
    fn from(error: &GetLoaderJobStatusError) -> Self {
        match error {
            GetLoaderJobStatusError::BadRequestException(exc) => exc.into(),
            source @ _ => {
                tracing::error!(
                    "Unknown service error checking the status of a loader job: {:}",
                    source
                );
                Self {
                    status_code: 500,
                    message: format!("{:}", source),
                    ..Default::default()
                }
            },
        }
    }
}

impl From<StartLoaderJobError> for LambdaResponse {
    fn from(error: StartLoaderJobError) -> Self {
        match error {
            StartLoaderJobError::BadRequestException(exc) => exc.into(),
            ref source @ _ => {
                let meta = error.meta();
                let msg = format!(
                    "Unknown error starting a loader job: {} {:?}",
                    source, meta
                );
                tracing::error!(msg);
                Self {
                    status_code: 500,
                    message: msg,
                    ..Default::default()
                }
            },
        }
    }
}
