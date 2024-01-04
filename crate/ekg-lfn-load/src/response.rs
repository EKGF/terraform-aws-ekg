use {
    aws_sdk_neptunedata::{
        error::SdkError,
        operation::start_loader_job::StartLoaderJobError,
        types::error::BadRequestException,
    },
    aws_smithy_runtime_api::client::result::TimeoutError,
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum LambdaDetailError {
    MaxLoadTaskQueueSizeLimitBreached,
    MaxConcurrentLoadLimitBreached,
}

impl LambdaDetailError {
    fn from(exc: &BadRequestException) -> Option<Self> {
        if exc.code.as_str() != "400" {
            if let Some(message) = exc.message.as_deref() {
                if message.contains("Max load task queue size limit breached") {
                    return Some(Self::MaxLoadTaskQueueSizeLimitBreached)
                } else if message.contains("Max concurrent load limit breached") {
                    return Some(Self::MaxConcurrentLoadLimitBreached)
                }
            }
        }
        None
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LambdaResponse {
    pub status_code:      u16,
    pub message:          String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_error:     Option<LambdaDetailError>,
}

impl LambdaResponse {
    pub fn clean(&mut self) -> Self {
        Self {
            status_code:      self.status_code,
            message:          self.message.clone(),
            detailed_message: if Some(self.message.as_str()) == self.detailed_message.as_deref() {
                None
            } else {
                self.detailed_message.clone()
            },
            detail_error:     self.detail_error.clone(),
        }
    }

    pub fn ok(message: &str) -> Self {
        tracing::info!(message);
        Self {
            status_code:      200,
            message:          message.to_string(),
            detailed_message: None,
            detail_error:     None,
        }
    }
}

impl From<&BadRequestException> for LambdaResponse {
    fn from(exc: &BadRequestException) -> Self {
        Self {
            status_code:      400,
            message:          exc.message.clone().unwrap_or("unknown message".to_string()),
            detailed_message: Some(exc.detailed_message.clone()),
            detail_error:     LambdaDetailError::from(exc),
        }
    }
}

impl From<&StartLoaderJobError> for LambdaResponse {
    fn from(error: &StartLoaderJobError) -> Self {
        tracing::error!("Unknown service error: {:?}", error);
        Self {
            status_code:      500,
            message:          format!("Service Error: {:?}", error),
            detailed_message: None,
            detail_error:     None,
        }
    }
}

impl From<TimeoutError> for LambdaResponse {
    fn from(error: TimeoutError) -> Self {
        tracing::error!("Timeout Error: {:?}", error);
        Self {
            status_code:      504,
            message:          "Timeout".to_string(),
            detailed_message: None,
            detail_error:     None,
        }
    }
}

impl<R> From<SdkError<StartLoaderJobError, R>> for LambdaResponse {
    fn from(error: SdkError<StartLoaderJobError, R>) -> Self {
        match error {
            SdkError::ServiceError(service_error) => {
                match service_error.err() {
                    StartLoaderJobError::BadRequestException(exc) => exc.into(),
                    source @ _ => source.into(),
                }
            },
            SdkError::TimeoutError(timeout_error) => timeout_error.into(),
            _ => {
                tracing::error!("Unknown error starting the RDF load: {:}", error);
                Self {
                    status_code:      500,
                    message:          format!("{:}", error),
                    detailed_message: None,
                    detail_error:     None,
                }
            },
        }
    }
}
