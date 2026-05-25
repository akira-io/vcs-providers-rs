use vcs_provider_core::{
    AuthHeaderStyle, AuthKind, Capability, CapabilitySet, Provider, ProviderDescriptor, ProviderId,
    Repositories,
};

mod repositories;

pub use repositories::BitbucketRepositories;

pub const PROVIDER_ID: &str = "bitbucket";
pub const DISPLAY_NAME: &str = "Bitbucket";
pub const DEFAULT_BASE_URL: &str = "https://api.bitbucket.org/2.0";

#[derive(Clone, Copy, Debug, Default)]
pub struct BitbucketProvider;

impl Provider for BitbucketProvider {
    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptor::make(
            ProviderId::make(PROVIDER_ID),
            DISPLAY_NAME,
            CapabilitySet::make([
                Capability::Repositories,
                Capability::Issues,
                Capability::CodeReviews,
                Capability::Pipelines,
                Capability::Webhooks,
            ]),
        )
    }

    fn repositories(&self) -> Box<dyn Repositories> {
        Box::<BitbucketRepositories>::default()
    }

    fn default_base_url(&self) -> &str {
        DEFAULT_BASE_URL
    }

    fn auth_header_style(&self, auth_kind: AuthKind) -> AuthHeaderStyle {
        match auth_kind {
            AuthKind::Anonymous => AuthHeaderStyle::None,
            AuthKind::PersonalAccessToken => AuthHeaderStyle::AuthorizationBearer,
            AuthKind::OAuth => AuthHeaderStyle::AuthorizationBearer,
            AuthKind::AppInstallation => AuthHeaderStyle::AuthorizationBearer,
            AuthKind::Jwt => AuthHeaderStyle::AuthorizationBearer,
        }
    }
}

pub fn provider() -> BitbucketProvider {
    BitbucketProvider
}

#[cfg(test)]
mod tests {
    use super::{BitbucketProvider, DISPLAY_NAME, PROVIDER_ID};
    use vcs_provider_core::{
        AuthHeaderStyle, AuthKind, Capability, Provider, ProviderId, ProviderRegistry,
        RepositoryCoordinates, VcsError, VcsResult,
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
        let registry = ProviderRegistry::builder()
            .register(super::provider())?
            .build();

        assert!(registry.contains_provider(&ProviderId::make(PROVIDER_ID)));

        Ok(())
    }

    #[test]
    fn bitbucket_provider_exposes_repositories_contract() -> VcsResult<()> {
        let repositories = BitbucketProvider.repositories();
        let coordinates = RepositoryCoordinates::make()
            .owner_name("akira-io")
            .name("vcs-providers-rs")
            .build()?;
        let result = futures::executor::block_on(repositories.get(coordinates));

        assert_eq!(result, Err(VcsError::TransportNotConfigured));

        Ok(())
    }
}
