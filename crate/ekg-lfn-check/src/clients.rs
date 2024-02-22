#[derive(Clone)]
pub struct Clients {
    pub aws_neptunedata_client: aws_sdk_neptunedata::Client,
    pub sparql_client:          ekg_sparql::SPARQLClient,
}
