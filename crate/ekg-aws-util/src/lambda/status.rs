use {
    aws_sdk_neptunedata::types::error::BadRequestException,
    serde::{Deserialize, Serialize},
    std::ops::Deref,
};

// noinspection SpellCheckingInspection
#[derive(Clone, Debug, Deserialize, Serialize, Default)]
pub enum LambdaDetailStatus {
    PipelineIdNotMatching,
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
    #[default]
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

    pub fn from_loader_job_status(status: &str) -> Self {
        match status {
            "LOAD_IN_QUEUE" => Self::LoaderJobInQueue,
            "LOAD_NOT_STARTED" => Self::LoaderJobNotStarted,
            "LOAD_IN_PROGRESS" => Self::LoaderJobInProgress,
            "LOAD_COMPLETED" => Self::LoaderJobCompleted,
            "LOAD_CANCELLED_BY_USER" => Self::LoaderJobCancelledByUser,
            "LOAD_CANCELLED_DUE_TO_ERRORS" => Self::LoaderJobCancelledDueToErrors,
            "LOAD_UNEXPECTED_ERROR" => Self::LoaderJobUnexpectedError,
            "LOAD_FAILED" => Self::LoaderJobFailed,
            "LOAD_S3_READ_ERROR" => Self::LoaderJobS3ReadError,
            "LOAD_S3_ACCESS_DENIED_ERROR" => Self::LoaderJobS3AccessDeniedError,
            "LOAD_COMMITTED_W_WRITE_CONFLICTS" => Self::LoaderJobCommittedWithWriteConflicts,
            "LOAD_DATA_DEADLOCK" => Self::LoaderJobDataDeadlock,
            "LOAD_DATA_FAILED_DUE_TO_FEED_MODIFIED_OR_DELETED" => {
                Self::LoaderJobDataFailedDueToFeedModifiedOrDeleted
            },
            "LOAD_FAILED_BECAUSE_DEPENDENCY_NOT_SATISFIED" => {
                Self::LoaderJobFailedBecauseDependencyNotSatisfied
            },
            "LOAD_FAILED_INVALID_REQUEST" => Self::LoaderJobFailedInvalidRequest,
            _ => {
                tracing::error!("Unknown loader job status: {}", status);
                Self::LoaderJobStatusUnknown
            },
        }
    }

    pub const fn message(&self) -> &'static str {
        match self {
            Self::PipelineIdNotMatching => "Pipeline ID not matching",
            Self::Timedout => "Timed out",
            Self::IOError => "I/O error",
            Self::MaxLoadTaskQueueSizeLimitBreached => "Max load task queue size limit breached",
            Self::MaxConcurrentLoadLimitBreached => "Max concurrent load limit breached",
            Self::LoaderJobInQueue => "Loader job is in the queue",
            Self::LoaderJobNotStarted => "Loader job has not started yet",
            Self::LoaderJobInProgress => "Loader job is still in progress",
            Self::LoaderJobCompleted => "Loader job completed",
            Self::LoaderJobCancelledByUser => "Loader job cancelled by user",
            Self::LoaderJobCancelledDueToErrors => "Loader job cancelled due to errors",
            Self::LoaderJobUnexpectedError => "Loader job failed due to unexpected error",
            Self::LoaderJobFailed => "Loader job failed",
            Self::LoaderJobS3ReadError => "Loader job failed due to S3 read error",
            Self::LoaderJobS3AccessDeniedError => "Loader job failed due to S3 access denied error",
            Self::LoaderJobCommittedWithWriteConflicts => {
                "Loader job failed due to write conflicts"
            },
            Self::LoaderJobDataDeadlock => "Loader job failed due to data deadlock",
            Self::LoaderJobDataFailedDueToFeedModifiedOrDeleted => {
                "Loader job failed because file was deleted or updated after load start."
            },
            Self::LoaderJobFailedBecauseDependencyNotSatisfied => {
                "Loader job failed because dependency was not satisfied."
            },
            Self::LoaderJobFailedInvalidRequest => "Loader job failed due to invalid request",
            Self::LoaderJobStatusUnknown => "Loader job status unknown",
            Self::UserError => "User error",
        }
    }

    pub fn should_show_detail(&self) -> bool {
        match self {
            Self::LoaderJobInQueue | Self::LoaderJobNotStarted | Self::LoaderJobInProgress => false,
            _ => true,
        }
    }

    /// Return true if the caller should check the status of the load again
    /// later
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::LoaderJobInQueue | Self::LoaderJobNotStarted | Self::LoaderJobInProgress => true,
            _ => false,
        }
    }

    /// Return the RDF (or RDFS or OWL) class name of the dataops:LoadRequest
    /// that corresponds to this status. The dataops:LoadRequest is the
    /// superclass of the following four classes:
    ///
    /// - dataops:QueuedLoadRequest
    /// - dataops:LoadingLoadRequest
    /// - dataops:FinishedLoadRequest
    /// - dataops:FailedLoadRequest
    ///
    /// We only return the local part, not the dataops ontology IRI part.
    pub fn rdf_class(&self) -> &ekg_metadata::Class {
        match self {
            LambdaDetailStatus::LoaderJobInQueue | LambdaDetailStatus::LoaderJobNotStarted => {
                crate::lambda::CLASS_DATAOPS_QUEUED_LOAD_REQUEST.deref()
            },
            LambdaDetailStatus::LoaderJobInProgress => {
                crate::lambda::CLASS_DATAOPS_LOADING_LOAD_REQUEST.deref()
            },
            LambdaDetailStatus::LoaderJobCompleted => {
                crate::lambda::CLASS_DATAOPS_FINISHED_LOAD_REQUEST.deref()
            },
            _ => crate::lambda::CLASS_DATAOPS_FAILED_LOAD_REQUEST.deref(),
        }
    }
}
