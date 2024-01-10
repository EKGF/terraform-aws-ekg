/// A `Namespace` represents a namespace IRI that can also be shown
/// in abbreviated format, also known as "prefix".
///
/// For instance, the namespace IRI <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
/// can also be shown (in [RDF Turtle](https://www.w3.org/TR/turtle/#prefixed-name)
/// or SPARQL for instance) as `rdf:`.
/// A "local name" such as "type" in such a namespace would look
/// like <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> or like `rdf:type`.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Namespace {
    /// assumed to end with ':'
    pub name: String,
    /// assumed to end with either '/' or '#'
    pub iri:  hyper::Uri,
}

impl std::fmt::Display for Namespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} <{}>", self.name.as_str(), self.iri)
    }
}

impl Namespace {
    pub fn declare(name: &str, iri: &hyper::Uri) -> Self {
        match iri.to_string().chars().last() {
            Some('/') | Some('#') => Self { name: name.to_string(), iri: iri.clone() },
            _ => {
                Self {
                    name: name.to_string(),
                    iri:  hyper::Uri::try_from(format!("{}/", iri)).unwrap(),
                }
            },
        }
    }

    pub fn declare_from_str(name: &str, iri: &str) -> Self {
        Self::declare(name, &hyper::Uri::try_from(iri).unwrap())
    }

    /// Return an identifier based on the current namespace IRI and the given
    /// local name within that namespace.
    pub fn with_local_name(&self, name: &str) -> Result<hyper::Uri, ekg_error::Error> {
        let iri_str = match self.iri.to_string().chars().last().unwrap() as char {
            '/' | '#' => format!("{}{name}", self.iri),
            _ => {
                panic!("{} does not end with either / or #", self.iri)
            },
        };

        Ok(hyper::Uri::try_from(iri_str)?)
    }

    #[cfg(all(feature = "rdftk-support", not(target_arch = "wasm32")))]
    pub fn as_rdftk_iri_ref(&self) -> Result<rdftk_iri::IRIRef, rdftk_iri::error::Error> {
        Ok(rdftk_iri::IRIRef::new(self.as_rdftk_iri()?))
    }

    #[cfg(all(feature = "rdftk-support", not(target_arch = "wasm32")))]
    pub fn as_rdftk_iri(&self) -> Result<rdftk_iri::IRI, rdftk_iri::error::Error> {
        use std::str::FromStr;
        rdftk_iri::IRI::from_str(self.iri.as_str())
    }

    pub fn as_sparql_prefix(&self) -> String { format!("PREFIX {} <{}>", self.name, self.iri) }

    pub fn as_turtle_prefix(&self) -> String { format!("@prefix {} <{}> .", self.name, self.iri) }
}

#[cfg(test)]
mod tests {
    use iref::Iri;

    #[test_log::test]
    fn test_a_prefix() -> Result<(), ekg_error::Error> {
        let namespace = Namespace::declare(
            "test:",
            Iri::new("http://whatever.kom/test#").unwrap(),
        );
        let x = namespace.with_local_name("abc")?;

        assert_eq!(x.as_str(), "http://whatever.kom/test#abc");
        Ok(())
    }

    #[test_log::test]
    fn test_b_prefix() -> Result<(), ekg_error::Error> {
        let namespace = Namespace::declare(
            "test:",
            Iri::new("http://whatever.kom/test/").unwrap(),
        );
        let x = namespace.with_local_name("abc")?;

        assert_eq!(x.as_str(), "http://whatever.kom/test/abc");
        Ok(())
    }
}
