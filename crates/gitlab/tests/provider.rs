use vcs_provider_core::{
    AuthHeaderStyle, AuthKind, Capability, Provider, ProviderId, VcsError, VcsResult, auth,
    provider,
};
use vcs_provider_gitlab::{DISPLAY_NAME, PROVIDER_ID, gitlab};

#[test]
fn gitlab_provider_exposes_provider_descriptor() {
    let descriptor = gitlab().descriptor();

    assert_eq!(descriptor.id().as_str(), PROVIDER_ID);
    assert_eq!(descriptor.display_name(), DISPLAY_NAME);
    assert!(descriptor.capabilities().supports(&Capability::SelfHosted));
}

#[test]
fn gitlab_provider_exposes_universal_contracts() {
    let provider = gitlab();

    assert!(provider.capabilities().supports(&Capability::Repos));
    drop(provider.repos());
    drop(provider.issues());
    drop(provider.code_reviews());
    drop(provider.pipelines());
    drop(provider.releases());
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
fn gitlab_provider_registers_through_core_registry() -> VcsResult<()> {
    let registry = provider().register(gitlab())?.build();

    assert!(registry.contains_provider(&ProviderId::make(PROVIDER_ID)));

    Ok(())
}

#[test]
fn gitlab_provider_registry_rejects_duplicate_provider_ids() -> VcsResult<()> {
    let result = provider().register(gitlab())?.register(gitlab());

    assert_eq!(
        result.err(),
        Some(VcsError::ProviderAlreadyRegistered(PROVIDER_ID.into()))
    );

    Ok(())
}

#[test]
fn gitlab_provider_registry_filters_by_capability() -> VcsResult<()> {
    let registry = provider().register(gitlab())?.build();
    let providers = registry
        .providers_supporting(Capability::Repos)
        .collect::<Vec<_>>();

    assert_eq!(providers.len(), 1);

    Ok(())
}
