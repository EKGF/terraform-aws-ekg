#![cfg(test)]

#[test_log::test(tokio::test)]
async fn test_sparql_client() -> Result<(), ekg_error::Error> {
    let sparql_client =
        crate::neptune::SPARQLClient::new("https://dbpedia.org/sparql".try_into()?, None).await?;

    let result: serde_json::Value = sparql_client
        .query_as_json("SELECT * WHERE { ?s ?p ?o } LIMIT 10")
        .await?;

    assert!(result.is_object());

    let head = result["head"].as_object().unwrap();

    let vars = head["vars"]
        .as_array()
        .unwrap()
        .into_iter()
        .map(|v| v.as_str().unwrap())
        .collect::<Vec<_>>()
        .join(",");

    assert_eq!(vars, "s,p,o");

    Ok(())
}
