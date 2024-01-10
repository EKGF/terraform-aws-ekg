#![allow(missing_docs)]
#![allow(clippy::wildcard_imports)]

use {
    crate::{consts::*, Namespace},
    hyper::Uri,
    lazy_static::lazy_static,
};

// Namespaces
#[rustfmt::skip]
lazy_static! {
    pub static ref PREFIX_BN: Namespace = Namespace::declare(PREFIX_NAME_BN, NS_BN.deref());
    pub static ref PREFIX_SD: Namespace = Namespace::declare(PREFIX_NAME_SD, NS_SD.deref());
    pub static ref PREFIX_WF: Namespace = Namespace::declare(PREFIX_NAME_WF, NS_WF.deref());
    pub static ref PREFIX_SBE: Namespace = Namespace::declare(PREFIX_NAME_SBE, NS_SBE.deref());
    pub static ref PREFIX_API: Namespace = Namespace::declare(PREFIX_NAME_API, NS_API.deref());
    pub static ref PREFIX_DCT: Namespace = Namespace::declare(PREFIX_NAME_DCT, NS_DCT.deref());
    pub static ref PREFIX_OWL: Namespace = Namespace::declare(PREFIX_NAME_OWL, NS_OWL.deref());
    pub static ref PREFIX_RAW: Namespace = Namespace::declare(PREFIX_NAME_RAW, NS_RAW.deref());
    pub static ref PREFIX_RDF: Namespace = Namespace::declare(PREFIX_NAME_RDF, NS_RDF.deref());
    pub static ref PREFIX_XSD: Namespace = Namespace::declare(PREFIX_NAME_XSD, NS_XSD.deref());
    pub static ref PREFIX_DCAT: Namespace = Namespace::declare(PREFIX_NAME_DCAT, NS_DCAT.deref());
    pub static ref PREFIX_JIRA: Namespace = Namespace::declare(PREFIX_NAME_JIRA, NS_JIRA.deref());
    pub static ref PREFIX_PROV: Namespace = Namespace::declare(PREFIX_NAME_PROV, NS_PROV.deref());
    pub static ref PREFIX_RDFS: Namespace = Namespace::declare(PREFIX_NAME_RDFS, NS_RDFS.deref());
    pub static ref PREFIX_SDLC: Namespace = Namespace::declare(PREFIX_NAME_SDLC, NS_SDLC.deref());
    pub static ref PREFIX_SKOS: Namespace = Namespace::declare(PREFIX_NAME_SKOS, NS_SKOS.deref());
    pub static ref PREFIX_EKGMM: Namespace = Namespace::declare(PREFIX_NAME_EKGMM, NS_EKGMM.deref());
    pub static ref PREFIX_RDFOX: Namespace = Namespace::declare(PREFIX_NAME_RDFOX, NS_RDFOX.deref());
    pub static ref PREFIX_STORY: Namespace = Namespace::declare(PREFIX_NAME_STORY, NS_STORY.deref());
    pub static ref PREFIX_DATAOPS: Namespace = Namespace::declare(PREFIX_NAME_DATAOPS, NS_DATAOPS.deref());
    pub static ref PREFIX_CONCEPT: Namespace = Namespace::declare(PREFIX_NAME_CONCEPT, NS_CONCEPT.deref());
    pub static ref PREFIX_DATASET: Namespace = Namespace::declare(PREFIX_NAME_DATASET, NS_DATASET.deref());
    pub static ref PREFIX_PERSONA: Namespace = Namespace::declare(PREFIX_NAME_PERSONA, NS_PERSONA.deref());
    pub static ref PREFIX_USE_CASE: Namespace = Namespace::declare(PREFIX_NAME_USE_CASE, NS_USE_CASE.deref());
    pub static ref PREFIX_STORY_IMPL_SPARQL: Namespace = Namespace::declare(PREFIX_NAME_STORY_IMPL_SPARQL, NS_STORY_IMPL_SPARQL.deref());
}

// Namespaces
#[rustfmt::skip]
lazy_static! {
    pub static ref NS_BN: Uri = Uri::from_static("https://ekgf.org/ontology/blank-node/");
    pub static ref NS_SD: Uri = Uri::from_static("http://www.w3.org/ns/sparql-service-description#");
    pub static ref NS_WF: Uri = Uri::from_static("https://ekgf.org/ontology/workflow-definition/");
    pub static ref NS_SBE: Uri = Uri::from_static("https://ekgf.org/ontology/specification-by-example/");
    pub static ref NS_API: Uri = Uri::from_static("https://ekgf.org/ontology/api/");
    pub static ref NS_DCT: Uri = Uri::from_static("http://purl.org/dc/terms/");
    pub static ref NS_OWL: Uri = Uri::from_static("http://www.w3.org/2002/07/owl#");
    pub static ref NS_RAW: Uri = Uri::from_static("https://ekgf.org/ontology/raw/");
    pub static ref NS_RDF: Uri = Uri::from_static("http://www.w3.org/1999/02/22-rdf-syntax-ns#");
    pub static ref NS_XSD: Uri = Uri::from_static("http://www.w3.org/2001/XMLSchema#");
    pub static ref NS_RDFS: Uri = Uri::from_static("http://www.w3.org/2000/01/rdf-schema#");
    pub static ref NS_DCAT: Uri = Uri::from_static("http://www.w3.org/ns/dcat#");
    pub static ref NS_PROV: Uri = Uri::from_static("http://www.w3.org/ns/prov#");
    pub static ref NS_JIRA: Uri = Uri::from_static("https://ekgf.org/ontology/jira/");
    pub static ref NS_SDLC: Uri = Uri::from_static("https://ekgf.org/ontology/software-development-life-cycle/");
    pub static ref NS_SKOS: Uri = Uri::from_static("http://www.w3.org/2004/02/skos/core#");
    pub static ref NS_RDFOX: Uri = Uri::from_static("http://oxfordsemantic.tech/RDFox#");
    pub static ref NS_STORY: Uri = Uri::from_static("https://ekgf.org/ontology/story/");
    pub static ref NS_EKGMM: Uri = Uri::from_static("https://ekgf.github.io/ekglib/ontology/maturity-model/");
    pub static ref NS_CONCEPT: Uri = Uri::from_static("https://ekgf.org/ontology/concept/");
    pub static ref NS_DATAOPS: Uri = Uri::from_static("https://ekgf.org/ontology/dataops/");
    pub static ref NS_DATASET: Uri = Uri::from_static("https://ekgf.org/ontology/dataset/");
    pub static ref NS_PERSONA: Uri = Uri::from_static("https://ekgf.org/ontology/persona/");
    pub static ref NS_USE_CASE: Uri = Uri::from_static("https://ekgf.org/ontology/use-case/");
    pub static ref NS_STORY_IMPL_SPARQL: Uri = Uri::from_static("https://ekgf.org/ontology/story-impl-sparql#");
}
