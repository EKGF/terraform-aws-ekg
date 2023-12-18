use ekg_error::Error;

pub struct EkgIdentifierContext {
    pub ekg_base:          String,
    pub ekg_id_base:       String,
    pub ekg_graph_base:    String,
    pub ekg_ontology_base: String,
}

impl EkgIdentifierContext {
    pub fn from_env(suffix: &str) -> Result<Self, Error> {
        let var = |name: &str| -> Result<String, Error> {
            let env_var_name = format!("{}{}", name, suffix);
            let val = match std::env::var(env_var_name.as_str()) {
                Ok(val) => {
                    if val.trim().len() == 0 {
                        Err(Error::EnvironmentVariableEmpty(
                            env_var_name.to_string(),
                        ))
                    } else {
                        Ok(val)
                    }
                },
                Err(_) => {
                    Err(Error::MandatoryEnvironmentVariableMissing(
                        env_var_name.to_string(),
                    ))
                },
            };
            val
        };

        Ok(Self {
            ekg_base:          var("EKG_BASE")?,
            ekg_id_base:       var("EKG_ID_BASE")?,
            ekg_graph_base:    var("EKG_GRAPH_BASE")?,
            ekg_ontology_base: var("EKG_ONTOLOGY_BASE")?,
        })
    }
}

pub struct EkgIdentifierContexts {
    pub internal: EkgIdentifierContext,
    pub external: EkgIdentifierContext,
}

impl EkgIdentifierContexts {
    pub fn from_env() -> Result<Self, Error> {
        Ok(Self {
            internal: EkgIdentifierContext::from_env("_INTERNAL")?,
            external: EkgIdentifierContext::from_env("_EXTERNAL")?,
        })
    }

    // #[cfg(test)]
    pub fn default_test() {
        std::env::set_var("EKG_BASE_INTERNAL", "https://placeholder.kg");
        std::env::set_var(
            "EKG_ID_BASE_INTERNAL",
            "https://placeholder.kg/id",
        );
        std::env::set_var(
            "EKG_GRAPH_BASE_INTERNAL",
            "https://placeholder.kg/graph",
        );
        std::env::set_var(
            "EKG_ONTOLOGY_BASE_INTERNAL",
            "https://placeholder.kg/ontology",
        );
        std::env::set_var("EKG_BASE_EXTERNAL", "http://localhost:3000");
        std::env::set_var("EKG_ID_BASE_EXTERNAL", "http://localhost:3000/id");
        std::env::set_var(
            "EKG_GRAPH_BASE_EXTERNAL",
            "http://localhost:3000/graph",
        );
        std::env::set_var(
            "EKG_ONTOLOGY_BASE_EXTERNAL",
            "http://localhost:3000/ontology",
        );
    }
}
