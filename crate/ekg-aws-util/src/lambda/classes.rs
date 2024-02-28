use {ekg_identifier::NS_DATAOPS, ekg_metadata::Class, lazy_static::lazy_static, std::ops::Deref};

pub const LN_LOAD_REQUEST: &str = "LoadRequest";
pub const LN_QUEUED_LOAD_REQUEST: &str = "QueuedLoadRequest";
pub const LN_LOADING_LOAD_REQUEST: &str = "LoadingLoadRequest";
pub const LN_FINISHED_LOAD_REQUEST: &str = "FinishedLoadRequest";
pub const LN_FAILED_LOAD_REQUEST: &str = "FailedLoadRequest";

#[rustfmt::skip]
lazy_static! {
    pub static ref CLASS_DATAOPS_LOAD_REQUEST: Class = Class::declare(NS_DATAOPS.clone(), LN_LOAD_REQUEST);
    pub static ref CLASS_DATAOPS_QUEUED_LOAD_REQUEST: Class = Class::declare(NS_DATAOPS.clone(), LN_QUEUED_LOAD_REQUEST);
    pub static ref CLASS_DATAOPS_LOADING_LOAD_REQUEST: Class = Class::declare(NS_DATAOPS.clone(), LN_LOADING_LOAD_REQUEST);
    pub static ref CLASS_DATAOPS_FINISHED_LOAD_REQUEST: Class = Class::declare(NS_DATAOPS.clone(), LN_FINISHED_LOAD_REQUEST);
    pub static ref CLASS_DATAOPS_FAILED_LOAD_REQUEST: Class = Class::declare(NS_DATAOPS.clone(), LN_FAILED_LOAD_REQUEST);
}

pub fn default_load_request_label(
    clazz: &Class,
    load_request_id: &str,
    source_iri: &str,
) -> String {
    if *clazz == *CLASS_DATAOPS_QUEUED_LOAD_REQUEST.deref() {
        format!(
            "Queued {} (load request {})",
            source_iri, load_request_id
        )
    } else if *clazz == *CLASS_DATAOPS_LOADING_LOAD_REQUEST {
        format!(
            "Loading {} (load request {})",
            source_iri, load_request_id
        )
    } else if *clazz == *CLASS_DATAOPS_FINISHED_LOAD_REQUEST {
        format!(
            "Finished loading {} (load request {})",
            source_iri, load_request_id
        )
    } else {
        format!(
            "Failed loading {} (load request {})",
            source_iri, load_request_id
        )
    }
}
