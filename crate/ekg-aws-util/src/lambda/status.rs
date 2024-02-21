use {
    aws_sdk_neptunedata::types::error::BadRequestException,
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum LambdaDetailStatus {
    Timedout,
    IOError,
    MaxLoadTaskQueueSizeLimitBreached,
    MaxConcurrentLoadLimitBreached,
    LoaderJobInQueue,
    LoaderJobNotStarted,
    LoaderJobInProgress,
    LoaderJobCompleted,
    LoaderJobCancelledByUser,
    LoaderJobCancelledDueToErrors,
    LoaderJobUnexpectedError,
    LoaderJobFailed,
    LoaderJobS3ReadError,
    LoaderJobS3AccessDeniedError,
    LoaderJobCommittedWithWriteConflicts,
    LoaderJobDataDeadlock,
    LoaderJobDataFailedDueToFeedModifiedOrDeleted,
    LoaderJobFailedBecauseDependencyNotSatisfied,
    LoaderJobFailedInvalidRequest,
    LoaderJobStatusUnknown,
    UserError,
}

impl LambdaDetailStatus {
    pub fn from_bad_request_exception(exc: &BadRequestException) -> Option<Self> {
        if exc.code.as_str() != "400" {
            if let Some(message) = exc.message.as_deref() {
                if message.contains("Max load task queue size limit breached") {
                    return Some(Self::MaxLoadTaskQueueSizeLimitBreached);
                } else if message.contains("Max concurrent load limit breached") {
                    return Some(Self::MaxConcurrentLoadLimitBreached);
                }
            }
        }
        None
    }
}
