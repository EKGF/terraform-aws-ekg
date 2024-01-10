use lazy_static::lazy_static;

#[rustfmt::skip]
lazy_static! {
    pub static ref IRI_SELECT: hyper::Uri = crate::CLASS_STORY_IMPL_SPARQL_SELECT.as_iri().unwrap();
    pub static ref IRI_ASK: hyper::Uri = crate::CLASS_STORY_IMPL_SPARQL_ASK.as_iri().unwrap();
    pub static ref IRI_CONSTRUCT: hyper::Uri = crate::CLASS_STORY_IMPL_SPARQL_CONSTRUCT.as_iri().unwrap();
    pub static ref IRI_DESCRIBE: hyper::Uri = crate::CLASS_STORY_IMPL_SPARQL_DESCRIBE.as_iri().unwrap();
    pub static ref IRI_UPDATE: hyper::Uri = crate::CLASS_STORY_IMPL_SPARQL_UPDATE.as_iri().unwrap();
    pub static ref IRI_DELETE: hyper::Uri = crate::CLASS_STORY_IMPL_SPARQL_DELETE.as_iri().unwrap();
}
