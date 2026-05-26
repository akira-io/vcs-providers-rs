use vcs_provider_core::{
    AuthHeaderStyle, AuthKind, Capability, Provider, RecordingTransport, VcsError, VcsResult, auth,
    provider, repo, response, run_async_test,
};
use vcs_provider_github::{DISPLAY_NAME, PROVIDER_ID, github};

#[test]
fn github_provider_exposes_provider_descriptor() {
    let descriptor = github().descriptor();

    assert_eq!(descriptor.id().as_str(), PROVIDER_ID);
    assert_eq!(descriptor.display_name(), DISPLAY_NAME);
    assert!(descriptor.capabilities().supports(&Capability::Repos));
}

#[test]
fn github_provider_exposes_universal_contracts() {
    let provider = github();

    assert!(provider.capabilities().supports(&Capability::Repos));
    drop(provider.repos());
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
fn github_client_sends_documented_headers_and_auth() -> VcsResult<()> {
    let transport = RecordingTransport::make(
        response()
            .body(
                r#"{"full_name":"akira-io/vcs-providers-rs","private":false,"archived":false,"disabled":false}"#,
            )
            .build(),
    );

    run_async_test(async {
        github()
            .client(transport.clone())
            .auth(auth().personal_access_token("test-token"))
            .repos()
            .get(repo().owner("akira-io").name("vcs-providers-rs").get())
            .await?;

        let requests = transport.requests();

        assert_eq!(
            requests.first().map(|request| request.headers().len()),
            Some(3)
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

        Ok(())
    })
}

#[test]
fn github_provider_registers_through_core_registry() -> VcsResult<()> {
    let registry = provider().register(github())?.build();

    assert!(registry.contains_provider(&vcs_provider_core::ProviderId::make(PROVIDER_ID)));

    Ok(())
}

#[test]
fn github_provider_registry_rejects_duplicate_provider_ids() -> VcsResult<()> {
    let result = provider().register(github())?.register(github());

    assert_eq!(
        result.err(),
        Some(VcsError::ProviderAlreadyRegistered(PROVIDER_ID.into()))
    );

    Ok(())
}

#[test]
fn github_provider_registry_filters_by_capability() -> VcsResult<()> {
    let registry = provider().register(github())?.build();
    let providers = registry
        .providers_supporting(Capability::Repos)
        .collect::<Vec<_>>();

    assert_eq!(providers.len(), 1);

    Ok(())
}
