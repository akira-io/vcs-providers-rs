use vcs_provider_core::{
    AuthHeaderStyle, AuthKind, Capability, Provider, ProviderRegistry, VcsError, VcsResult, repo,
};
use vcs_provider_github::{DISPLAY_NAME, GitHubProvider, PROVIDER_ID, provider};

#[test]
fn github_provider_exposes_provider_descriptor() {
    let descriptor = GitHubProvider.descriptor();

    assert_eq!(descriptor.id().as_str(), PROVIDER_ID);
    assert_eq!(descriptor.display_name(), DISPLAY_NAME);
    assert!(descriptor.capabilities().supports(&Capability::Repos));
}

#[test]
fn github_provider_uses_bearer_auth_for_tokens() {
    let style = GitHubProvider.auth_header_style(AuthKind::PersonalAccessToken);

    assert_eq!(style, AuthHeaderStyle::AuthorizationBearer);
}

#[test]
fn github_provider_registers_through_core_registry() -> VcsResult<()> {
    let registry = ProviderRegistry::builder().register(provider())?.build();

    assert!(registry.contains_provider(&vcs_provider_core::ProviderId::make(PROVIDER_ID)));

    Ok(())
}

#[test]
fn github_provider_registry_rejects_duplicate_provider_ids() -> VcsResult<()> {
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
fn github_provider_registry_filters_by_capability() -> VcsResult<()> {
    let registry = ProviderRegistry::builder().register(provider())?.build();
    let providers = registry
        .providers_supporting(Capability::Repos)
        .collect::<Vec<_>>();

    assert_eq!(providers.len(), 1);

    Ok(())
}

#[test]
fn github_provider_exposes_repos_contract() -> VcsResult<()> {
    let repo = repo().name("vcs-providers-rs").owner("akira-io").build();
    let result = futures::executor::block_on(GitHubProvider.repos().get(repo));

    assert_eq!(result, Err(VcsError::TransportNotConfigured));

    Ok(())
}
