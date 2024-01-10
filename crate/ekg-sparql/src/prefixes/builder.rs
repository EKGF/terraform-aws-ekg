use {super::Prefixes, ekg_namespace::Namespace};

#[derive(Default)]
pub struct PrefixesBuilder {
    prefixes: Vec<Namespace>,
}

impl<'a> PrefixesBuilder {
    pub fn default_builder() -> Self { PrefixesBuilder { prefixes: Vec::new() } }

    pub fn declare_with_name_and_iri(mut self, name: &str, iri: &hyper::Uri) -> Self {
        self.prefixes.push(Namespace::declare(name, iri));
        self
    }

    pub fn declare(mut self, namespace: &Namespace) -> Self {
        self.prefixes.push(namespace.clone());
        self
    }

    pub fn build(self) -> Result<Prefixes, ekg_error::Error> {
        let mut to_build = Prefixes::empty()?;
        for namespace in self.prefixes.iter() {
            to_build.declare_namespace(namespace)?;
        }
        Ok(to_build)
    }
}
