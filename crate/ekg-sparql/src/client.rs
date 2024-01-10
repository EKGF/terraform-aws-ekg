use {
    crate::{ParsedStatement, Statement},
    ekg_error::Error,
    ekg_util::env::mandatory_env_var,
    hyper::Uri,
};

/// Simple SPARQL client for sending SPARQL queries (or update statements) to a
/// SPARQL endpoint.
#[derive(Clone)]
pub struct SPARQLClient {
    pub(crate) client: hyper::client::Client<
        hyper_rustls::HttpsConnector<hyper::client::HttpConnector>,
        hyper::Body,
    >,
    pub(crate) query_endpoint:  Uri,
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
            client:          http_client,
            query_endpoint:  query_endpoint.clone(),
            update_endpoint: if let Some(update_endpoint) = update_endpoint {
                update_endpoint
            } else {
                query_endpoint.clone()
            },
        })
    }

    /// Convert a SPARQL statement into a hyper::Body, properly encoded.
    fn statement_as_body(parsed_statement: &ParsedStatement) -> Result<hyper::Body, Error> {
        let operation = if parsed_statement.statement_type.is_update_statement() {
            "update"
        } else {
            "query"
        };
        let meal = &[(operation, parsed_statement.statement.as_str())];
        let body_str = serde_urlencoded::to_string(meal)?;
        Ok(hyper::Body::from(body_str))
    }

    async fn build_request(
        &self,
        statement: &Statement,
    ) -> Result<hyper::Request<hyper::Body>, ekg_error::Error> {
        let parsed_statement = ParsedStatement::parse(statement, None)?;
        let uri = if parsed_statement.statement_type.is_query_statement() {
            &self.query_endpoint
        } else {
            &self.update_endpoint
        };
        let accept_header = parsed_statement
            .statement_type
            .default_statement_response_mime_type();

        tracing::info!("SPARQL endpoint: {:}", uri);
        let request = hyper::http::request::Builder::new()
            .method(hyper::http::method::Method::POST)
            .uri(uri.clone())
            .header(
                hyper::http::header::CONTENT_TYPE,
                "application/x-www-form-urlencoded",
            )
            .header(
                hyper::http::header::ACCEPT,
                accept_header,
            )
            // See https://docs.aws.amazon.com/neptune/latest/userguide/access-graph-sparql-http-trailing-headers.html
            .header(hyper::http::header::TE, "trailers, deflate, gzip")
            .body(Self::statement_as_body(&parsed_statement)?)?;
        tracing::info!("request: {:?}", request);
        Ok(request)
    }

    pub async fn execute(&self, statement: &Statement) -> Result<(), Error> {
        tracing::info!("SPARQL statement: {}", statement);

        let req = self.build_request(statement).await?;
        match self.client.request(req).await {
            Ok(response) => {
                tracing::info!("response1: {:?}", response);
                let (parts, body) = response.into_parts();
                tracing::info!(
                    "response2: status={:} headers={:#?}",
                    parts.status.as_str(),
                    parts.headers
                );
                // TODO: limit the amount of memory used here
                let body_bytes = hyper::body::to_bytes(body).await?;
                let v: serde_json::Value = serde_json::from_slice::<serde_json::Value>(&body_bytes)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                tracing::info!("response3: {:?}", v);
                Ok(())
            },
            Err(error) => {
                tracing::error!("error: {:?}", error);
                Err(Error::from(error))
            },
        }
    }
}
