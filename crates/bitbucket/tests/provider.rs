use vcs_provider_bitbucket::{BitbucketProvider, DISPLAY_NAME, PROVIDER_ID, provider};
use vcs_provider_core::{
    AuthHeaderStyle, AuthKind, Capability, Provider, ProviderId, ProviderRegistry, VcsError,
    VcsResult, repo,
};

#[test]
fn bitbucket_provider_exposes_provider_descriptor() {
    let descriptor = BitbucketProvider.descriptor();

    assert_eq!(descriptor.id().as_str(), PROVIDER_ID);
    assert_eq!(descriptor.display_name(), DISPLAY_NAME);
    assert!(descriptor.capabilities().supports(&Capability::Pipelines));
}

#[test]
fn bitbucket_provider_uses_bearer_auth_for_oauth() {
    let style = BitbucketProvider.auth_header_style(AuthKind::OAuth);

    assert_eq!(style, AuthHeaderStyle::AuthorizationBearer);
}

#[test]
fn bitbucket_provider_registers_through_core_registry() -> VcsResult<()> {
    let registry = ProviderRegistry::builder().register(provider())?.build();

    assert!(registry.contains_provider(&ProviderId::make(PROVIDER_ID)));

    Ok(())
}

#[test]
fn bitbucket_provider_registry_rejects_duplicate_provider_ids() -> VcsResult<()> {
    let result = ProviderRegistry::builder()
        .register(provider())?
        .register(provider());

    assert_eq!(
        result.err(),
        Some(VcsError::ProviderAlreadyRegistered(PROVIDER_ID.into()))
    );

    Ok(())
}

#[test]
fn bitbucket_provider_registry_filters_by_capability() -> VcsResult<()> {
    let registry = ProviderRegistry::builder().register(provider())?.build();
    let providers = registry
        .providers_supporting(Capability::Repos)
        .collect::<Vec<_>>();

    assert_eq!(providers.len(), 1);

    Ok(())
}

#[test]
fn bitbucket_provider_exposes_repos_contract() -> VcsResult<()> {
    let repo = repo().name("vcs-providers-rs").owner("akira-io").build();
    let result = futures::executor::block_on(BitbucketProvider.repos().get(repo));

    assert_eq!(result, Err(VcsError::TransportNotConfigured));

    Ok(())
}
