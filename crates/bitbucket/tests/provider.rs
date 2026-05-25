use vcs_provider_bitbucket::{DISPLAY_NAME, PROVIDER_ID, bitbucket};
use vcs_provider_core::{
    AuthHeaderStyle, AuthKind, Capability, Provider, ProviderId, VcsError, VcsResult, auth,
    provider,
};

#[test]
fn bitbucket_provider_exposes_provider_descriptor() {
    let descriptor = bitbucket().descriptor();

    assert_eq!(descriptor.id().as_str(), PROVIDER_ID);
    assert_eq!(descriptor.display_name(), DISPLAY_NAME);
    assert!(descriptor.capabilities().supports(&Capability::Pipelines));
    assert!(!descriptor.capabilities().supports(&Capability::Issues));
}

#[test]
fn bitbucket_provider_exposes_universal_contracts() {
    let provider = bitbucket();

    assert!(provider.capabilities().supports(&Capability::Repos));
    drop(provider.repos());
    drop(provider.issues());
    drop(provider.code_reviews());
    drop(provider.pipelines());
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
fn bitbucket_provider_registers_through_core_registry() -> VcsResult<()> {
    let registry = provider().register(bitbucket())?.build();

    assert!(registry.contains_provider(&ProviderId::make(PROVIDER_ID)));

    Ok(())
}

#[test]
fn bitbucket_provider_registry_rejects_duplicate_provider_ids() -> VcsResult<()> {
    let result = provider().register(bitbucket())?.register(bitbucket());

    assert_eq!(
        result.err(),
        Some(VcsError::ProviderAlreadyRegistered(PROVIDER_ID.into()))
    );

    Ok(())
}

#[test]
fn bitbucket_provider_registry_filters_by_capability() -> VcsResult<()> {
    let registry = provider().register(bitbucket())?.build();
    let providers = registry
        .providers_supporting(Capability::Repos)
        .collect::<Vec<_>>();

    assert_eq!(providers.len(), 1);

    Ok(())
}
