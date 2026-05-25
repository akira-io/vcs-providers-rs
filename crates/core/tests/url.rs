use vcs_provider_core::url;

#[test]
fn url_builder_appends_query_parameters() {
    let request_url = url("https://api.example.test/items")
        .query_param("per_page", "10")
        .build();

    assert_eq!(
        request_url.as_str(),
        "https://api.example.test/items?per_page=10"
    );
}

#[test]
fn url_builder_joins_base_url_and_path() {
    let request_url = url("https://api.example.test")
        .path("/repos")
        .query_param("per_page", "100")
        .build();

    assert_eq!(
        request_url.as_str(),
        "https://api.example.test/repos?per_page=100"
    );
}

#[test]
fn url_builder_accepts_optional_and_multiple_query_parameters() {
    let request_url = url("https://api.example.test/items?sort=updated")
        .optional_query_param("cursor", Some("next page"))
        .optional_query_param("ignored", None::<String>)
        .query_params([("state", "open"), ("labels", "rust,core")])
        .build();

    assert_eq!(
        request_url.as_str(),
        "https://api.example.test/items?sort=updated&cursor=next%20page&state=open&labels=rust%2Ccore"
    );
}

#[test]
fn url_builder_preserves_fragments_after_query_parameters() {
    let request_url = url("https://api.example.test/items#section")
        .query_param("per_page", "10")
        .build();

    assert_eq!(
        request_url.as_str(),
        "https://api.example.test/items?per_page=10#section"
    );
}
