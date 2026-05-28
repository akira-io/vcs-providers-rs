use git_cognition_core::{
    AuthHeaderStyle, AuthKind, Capability, CognitionError, CognitionResult, Provider, Visibility,
    auth, cognition, provider, provider_id, repo, run_async_test,
};
use git_cognition_gitlab::{DISPLAY_NAME, PROVIDER_ID, gitlab};

#[test]
fn gitlab_provider_exposes_provider_descriptor() {
    let descriptor = gitlab().descriptor();

    assert_eq!(descriptor.id().as_str(), PROVIDER_ID);
    assert_eq!(descriptor.display_name(), DISPLAY_NAME);
    assert!(descriptor.capabilities().supports(&Capability::SelfHosted));
    assert!(
        descriptor
            .capabilities()
            .supports(&Capability::Organizations)
    );
    assert!(!descriptor.capabilities().supports(&Capability::Discussions));
    assert!(!descriptor.capabilities().supports(&Capability::Webhooks));
}

#[test]
fn gitlab_provider_exposes_universal_contracts() {
    let provider = gitlab();

    assert!(provider.capabilities().supports(&Capability::Repos));
    drop(Provider::authentication(&provider));
    drop(Provider::organizations(&provider));
    drop(Provider::repos(&provider));
    drop(Provider::issues(&provider));
    drop(Provider::code_reviews(&provider));
    drop(Provider::pipelines(&provider));
    drop(Provider::releases(&provider));
}

#[test]
fn gitlab_provider_uses_private_token_header_for_personal_access_tokens() {
    let style = gitlab().auth_header_style(AuthKind::PersonalAccessToken);

    assert_eq!(style, AuthHeaderStyle::CustomHeader("private-token".into()));
}

#[test]
fn gitlab_provider_maps_personal_access_token_header() {
    let credential = auth().personal_access_token("test-token");
    let header = gitlab().auth_header(&credential);

    assert_eq!(
        header.map(|header| (
            header.name().as_str().to_owned(),
            header.value().as_str().to_owned()
        )),
        Some(("private-token".into(), "test-token".into()))
    );
}

#[test]
fn gitlab_client_routes_auth_and_middleware_through_transport() -> CognitionResult<()> {
    let transport = gitlab()
        .body(
            r#"{"path_with_namespace":"akira-io/git-cognition-rs","visibility":"private","archived":false}"#,
        )
        .record();
    run_async_test(async {
        let repository = cognition()
            .provider(gitlab())
            .middleware(transport.clone())
            .header("x-cognition-trace", "trace-1")
            .auth(auth().personal_access_token("test-token"))
            .repos()
            .get(repo().owner("akira-io").name("git-cognition-rs").get())
            .await?;

        let requests = transport.requests();

        assert_eq!(repository.provider().as_str(), PROVIDER_ID);
        assert_eq!(repository.visibility(), &Visibility::Private);
        assert_eq!(
            requests.first().map(|request| request.headers().len()),
            Some(3)
        );
        assert_eq!(
            requests
                .first()
                .and_then(|request| request.headers().first())
                .map(|header| header.value().as_str()),
            Some("application/json")
        );
        assert_eq!(
            requests
                .first()
                .and_then(|request| request.headers().get(1))
                .map(|header| header.name().as_str()),
            Some("private-token")
        );
        assert_eq!(
            requests
                .first()
                .and_then(|request| request.headers().get(2))
                .map(|header| header.name().as_str()),
            Some("x-cognition-trace")
        );

        Ok(())
    })
}

#[test]
fn gitlab_provider_registers_through_core_registry() -> CognitionResult<()> {
    let registry = provider().register(gitlab())?.build();

    assert!(registry.contains_provider(&provider_id(PROVIDER_ID)));

    Ok(())
}

#[test]
fn gitlab_provider_registry_rejects_duplicate_provider_ids() -> CognitionResult<()> {
    let result = provider().register(gitlab())?.register(gitlab());

    assert_eq!(
        result.err(),
        Some(CognitionError::ProviderAlreadyRegistered(
            PROVIDER_ID.into()
        ))
    );

    Ok(())
}

#[test]
fn gitlab_provider_registry_filters_by_capability() -> CognitionResult<()> {
    let registry = provider().register(gitlab())?.build();
    let providers = registry
        .providers_supporting(Capability::Repos)
        .collect::<Vec<_>>();

    assert_eq!(providers.len(), 1);

    Ok(())
}
