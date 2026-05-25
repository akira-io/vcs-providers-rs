use vcs_provider_core::{
    AuthHeaderStyle, AuthKind, Capability, Provider, VcsError, VcsResult, auth, provider,
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
    drop(provider.issues());
    drop(provider.code_reviews());
    drop(provider.pipelines());
    drop(provider.releases());
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
