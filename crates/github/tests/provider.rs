use git_cognition_core::{
    AuthHeaderStyle, AuthKind, Capability, CognitionError, CognitionResult, Provider, Visibility,
    auth, cognition, provider, provider_id, repo, run_async_test,
};
use git_cognition_github::{DISPLAY_NAME, PROVIDER_ID, github};

#[test]
fn github_provider_exposes_provider_descriptor() {
    let descriptor = github().descriptor();

    assert_eq!(descriptor.id().as_str(), PROVIDER_ID);
    assert_eq!(descriptor.display_name(), DISPLAY_NAME);
    assert!(
        descriptor
            .capabilities()
            .supports(&Capability::Authentication)
    );
    assert!(
        descriptor
            .capabilities()
            .supports(&Capability::AuthenticationValidate)
    );
    assert!(descriptor.capabilities().supports(&Capability::Repos));
    assert!(
        descriptor
            .capabilities()
            .supports(&Capability::Organizations)
    );
    assert!(
        descriptor
            .capabilities()
            .supports(&Capability::OrganizationList)
    );
    assert!(!descriptor.capabilities().supports(&Capability::Discussions));
    assert!(!descriptor.capabilities().supports(&Capability::Webhooks));
}

#[test]
fn github_provider_exposes_universal_contracts() {
    let provider = github();

    drop(provider.authentication());
    drop(provider.organizations());
    assert!(provider.capabilities().supports(&Capability::Repos));
    drop(Provider::repos(&provider));
    drop(Provider::issues(&provider));
    drop(Provider::code_reviews(&provider));
    drop(Provider::pipelines(&provider));
    drop(Provider::releases(&provider));
}

#[test]
fn github_provider_uses_bearer_auth_for_tokens() {
    let style = github().auth_header_style(AuthKind::PersonalAccessToken);

    assert_eq!(style, AuthHeaderStyle::AuthorizationBearer);
}

#[test]
fn github_provider_maps_personal_access_token_header() {
    let credential = auth().personal_access_token("test-token");
    let header = github().auth_header(&credential);

    assert_eq!(
        header.map(|header| (
            header.name().as_str().to_owned(),
            header.value().as_str().to_owned()
        )),
        Some(("authorization".into(), "Bearer test-token".into()))
    );
}

#[test]
fn github_client_routes_auth_and_middleware_through_transport() -> CognitionResult<()> {
    let transport = github()
        .body(
            r#"{"full_name":"akira-io/git-cognition-rs","private":false,"archived":false,"disabled":false}"#,
        )
        .record();
    run_async_test(async {
        let repository = cognition()
            .provider(github())
            .middleware(transport.clone())
            .header("x-cognition-trace", "trace-1")
            .auth(auth().personal_access_token("test-token"))
            .repos()
            .get(repo().owner("akira-io").name("git-cognition-rs").get())
            .await?;

        let requests = transport.requests();

        assert_eq!(repository.provider().as_str(), PROVIDER_ID);
        assert_eq!(repository.visibility(), &Visibility::Public);
        assert_eq!(
            requests.first().map(|request| request.headers().len()),
            Some(4)
        );
        assert_eq!(
            requests
                .first()
                .and_then(|request| request.headers().first())
                .map(|header| header.name().as_str()),
            Some("accept")
        );
        assert_eq!(
            requests
                .first()
                .and_then(|request| request.headers().get(2))
                .map(|header| header.value().as_str()),
            Some("Bearer test-token")
        );
        assert_eq!(
            requests
                .first()
                .and_then(|request| request.headers().get(3))
                .map(|header| header.name().as_str()),
            Some("x-cognition-trace")
        );

        Ok(())
    })
}

#[test]
fn github_provider_registers_through_core_registry() -> CognitionResult<()> {
    let registry = provider().register(github())?.build();

    assert!(registry.contains_provider(&provider_id(PROVIDER_ID)));

    Ok(())
}

#[test]
fn github_provider_registry_rejects_duplicate_provider_ids() -> CognitionResult<()> {
    let result = provider().register(github())?.register(github());

    assert_eq!(
        result.err(),
        Some(CognitionError::ProviderAlreadyRegistered(
            PROVIDER_ID.into()
        ))
    );

    Ok(())
}

#[test]
fn github_provider_registry_filters_by_capability() -> CognitionResult<()> {
    let registry = provider().register(github())?.build();
    let providers = registry
        .providers_supporting(Capability::Repos)
        .collect::<Vec<_>>();

    assert_eq!(providers.len(), 1);

    Ok(())
}
