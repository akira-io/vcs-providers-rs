use git_cognition_bitbucket::{DISPLAY_NAME, PROVIDER_ID, bitbucket};
use git_cognition_core::{
    AuthHeaderStyle, AuthKind, Capability, CognitionError, CognitionResult, Provider,
    ReleasesFluent, Visibility, auth, cognition, provider, provider_id, repo, run_async_test,
};

#[test]
fn bitbucket_provider_exposes_provider_descriptor() {
    let descriptor = bitbucket().descriptor();

    assert_eq!(descriptor.id().as_str(), PROVIDER_ID);
    assert_eq!(descriptor.display_name(), DISPLAY_NAME);
    assert!(descriptor.capabilities().supports(&Capability::Pipelines));
    assert!(
        descriptor
            .capabilities()
            .supports(&Capability::PipelineCancel)
    );
    assert!(
        !descriptor
            .capabilities()
            .supports(&Capability::PipelineRerun)
    );
    assert!(descriptor.capabilities().supports(&Capability::Issues));
    assert!(
        descriptor
            .capabilities()
            .supports(&Capability::Organizations)
    );
    assert!(!descriptor.capabilities().supports(&Capability::Discussions));
    assert!(!descriptor.capabilities().supports(&Capability::Webhooks));
}

#[test]
fn bitbucket_provider_exposes_universal_contracts() {
    let provider = bitbucket();

    assert!(provider.capabilities().supports(&Capability::Repos));
    drop(Provider::repos(&provider));
    drop(Provider::authentication(&provider));
    drop(Provider::organizations(&provider));
    drop(Provider::issues(&provider));
    drop(Provider::code_reviews(&provider));
    drop(Provider::pipelines(&provider));
    drop(provider.releases());
}

#[test]
fn bitbucket_provider_uses_bearer_auth_for_oauth() {
    let style = bitbucket().auth_header_style(AuthKind::OAuth);

    assert_eq!(style, AuthHeaderStyle::AuthorizationBearer);
}

#[test]
fn bitbucket_provider_maps_oauth_header() {
    let credential = auth().oauth("test-token");
    let header = bitbucket().auth_header(&credential);

    assert_eq!(
        header.map(|header| (
            header.name().as_str().to_owned(),
            header.value().as_str().to_owned()
        )),
        Some(("authorization".into(), "Bearer test-token".into()))
    );
}

#[test]
fn bitbucket_client_routes_auth_and_middleware_through_transport() -> CognitionResult<()> {
    let transport = bitbucket()
        .body(r#"{"full_name":"akira-io/git-cognition-rs","is_private":true}"#)
        .record();
    run_async_test(async {
        let repository = cognition()
            .provider(bitbucket())
            .middleware(transport.clone())
            .header("x-cognition-trace", "trace-1")
            .auth(auth().oauth("test-token"))
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
                .map(|header| header.value().as_str()),
            Some("Bearer test-token")
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
fn bitbucket_releases_report_unsupported_operation() {
    let repo = repo().owner("akira-io").name("git-cognition-rs").get();
    let result = run_async_test(async {
        bitbucket()
            .releases()
            .delete()
            .location(repo)
            .id("v1.0.0")
            .delete()
            .await
    });

    assert!(matches!(
        result,
        Err(CognitionError::UnsupportedOperation(operation)) if operation == "release delete"
    ));
}

#[test]
fn bitbucket_provider_registers_through_core_registry() -> CognitionResult<()> {
    let registry = provider().register(bitbucket())?.build();

    assert!(registry.contains_provider(&provider_id(PROVIDER_ID)));

    Ok(())
}

#[test]
fn bitbucket_provider_registry_rejects_duplicate_provider_ids() -> CognitionResult<()> {
    let result = provider().register(bitbucket())?.register(bitbucket());

    assert_eq!(
        result.err(),
        Some(CognitionError::ProviderAlreadyRegistered(
            PROVIDER_ID.into()
        ))
    );

    Ok(())
}

#[test]
fn bitbucket_provider_registry_filters_by_capability() -> CognitionResult<()> {
    let registry = provider().register(bitbucket())?.build();
    let providers = registry
        .providers_supporting(Capability::Repos)
        .collect::<Vec<_>>();

    assert_eq!(providers.len(), 1);

    Ok(())
}
