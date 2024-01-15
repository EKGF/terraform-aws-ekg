#![allow(missing_docs)]

use {
    ekg_namespace::{
        StaticIRI,
        CLASS_STORY_IMPL_SPARQL_SPARQL10,
        CLASS_STORY_IMPL_SPARQL_SPARQL11,
    },
    lazy_static::lazy_static,
};

#[rustfmt::skip]
lazy_static! {
    pub static ref IRI_SPARQL10: StaticIRI = CLASS_STORY_IMPL_SPARQL_SPARQL10.as_iri().unwrap();
    pub static ref IRI_SPARQL11: StaticIRI = CLASS_STORY_IMPL_SPARQL_SPARQL11.as_iri().unwrap();
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum SPARQLFlavor {
    SPARQL10,
    SPARQL11,
}

impl SPARQLFlavor {
    pub fn from_iri(iri: Option<&fluent_uri::Uri<&str>>) -> Option<Self> {
        match iri {
            Some(iri) if iri == *IRI_SPARQL10 => Some(Self::SPARQL10),
            Some(iri) if iri == *IRI_SPARQL11 => Some(Self::SPARQL11),
            _ => None,
        }
    }
}
