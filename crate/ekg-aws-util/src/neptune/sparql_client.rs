use hyper::Uri;
use serde::Deserialize;

// use aws_smithy_runtime::client::http::hyper_014::HyperClientBuilder;
use ekg_error::Error;
use ekg_util::env::mandatory_env_var;

/// Simple SPARQL client for sending SPARQL queries (or update statements) to a SPARQL endpoint.
pub struct SPARQLClient {
    pub(crate) client: hyper::client::Client<
        hyper_rustls::HttpsConnector<hyper::client::HttpConnector>,
        hyper::Body,
    >,
    pub(crate) query_endpoint: Uri,
    pub(crate) update_endpoint: Uri,
}

impl SPARQLClient {
    pub async fn from_env() -> Result<Self, Error> {
        let query_endpoint = mandatory_env_var("EKG_SPARQL_QUERY_ENDPOINT", None)?;
        let update_endpoint = mandatory_env_var("EKG_SPARQL_UPDATE_ENDPOINT", None)?;

        Self::new(
            query_endpoint.try_into()?,
            Some(update_endpoint.try_into()?),
        )
        .await
    }

    pub async fn new(query_endpoint: Uri, update_endpoint: Option<Uri>) -> Result<Self, Error> {
        // Let webpki load the Mozilla root certificates.
        let mut root_store = rustls::RootCertStore::empty();
        root_store.add_trust_anchors(webpki_roots::TLS_SERVER_ROOTS.iter().map(|ta| {
            rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
                ta.subject.to_vec(),
                ta.subject_public_key_info.to_vec(),
                ta.name_constraints.clone().map(|v| v.to_vec()),
            )
        }));

        let tls_config = rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        let tls_connector = hyper_rustls::HttpsConnectorBuilder::new()
            .with_tls_config(tls_config)
            .https_only()
            .enable_http2()
            .build();

        // Build the hyper client from the HTTPS connector.
        let builder = hyper::client::Client::builder();
        let http_client = builder.build(tls_connector);

        Ok(Self {
            client: http_client,
            query_endpoint: query_endpoint.clone(),
            update_endpoint: if let Some(update_endpoint) = update_endpoint {
                update_endpoint
            } else {
                query_endpoint.clone()
            },
        })
    }

    fn get_request_builder(&self, update: bool) -> hyper::http::request::Builder {
        let uri = if update {
            &self.update_endpoint
        } else {
            &self.query_endpoint
        };
        hyper::http::request::Builder::new()
            .method(hyper::http::method::Method::POST)
            .uri(uri.clone())
            .header(
                hyper::http::header::CONTENT_TYPE,
                "application/sparql-query",
            )
            .header(
                hyper::http::header::ACCEPT,
                "application/sparql-results+json",
            )
    }

    /// Send a SPARQL query to the SPARQL endpoint and return the results as JSON.
    /// Note that this method does not work for SPARQL UPDATE statements, see `update`.
    pub async fn query_as_json<T>(&self, sparql_statement: &str) -> Result<T, Error>
    where
        T: for<'a> Deserialize<'a> + core::fmt::Debug,
    {
        let req = self
            .get_request_builder(false)
            .body(hyper::Body::from(sparql_statement.to_string()))
            .expect("request builder");
        match self.client.request(req).await {
            Ok(response) => {
                let (_parts, body) = response.into_parts();
                // TODO: limit the amount of memory used here
                let body_bytes = hyper::body::to_bytes(body).await?;
                let v: T = serde_json::from_slice::<T>(&body_bytes)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                // println!("response: {:?}", v);
                Ok(v)
            },
            Err(error) => {
                tracing::error!("error: {:?}", error);
                Err(Error::from(error))
            },
        }
    }

    pub async fn update(&self, sparql_statement: &str) -> Result<(), Error> {
        let req = self
            .get_request_builder(true)
            .body(hyper::Body::from(sparql_statement.to_string()))
            .expect("request builder");
        match self.client.request(req).await {
            Ok(response) => {
                tracing::info!("response: {:?}", response);
                println!("response: {:?}", response);
                Ok(())
            },
            Err(error) => {
                tracing::error!("error: {:?}", error);
                Err(Error::from(error))
            },
        }
    }
}
