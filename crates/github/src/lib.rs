use vcs_provider_core::{
    AuthHeaderStyle, AuthKind, Capability, CapabilitySet, Provider, ProviderDescriptor, ProviderId,
    Repositories,
};

mod repositories;

pub use repositories::GitHubRepositories;

pub const PROVIDER_ID: &str = "github";
pub const DISPLAY_NAME: &str = "GitHub";
pub const DEFAULT_BASE_URL: &str = "https://api.github.com";

#[derive(Clone, Copy, Debug, Default)]
pub struct GitHubProvider;

impl Provider for GitHubProvider {
    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptor::make(
            ProviderId::make(PROVIDER_ID),
            DISPLAY_NAME,
            CapabilitySet::make([
                Capability::Repositories,
                Capability::Issues,
                Capability::CodeReviews,
                Capability::Pipelines,
                Capability::Releases,
                Capability::Organizations,
                Capability::Discussions,
                Capability::Webhooks,
            ]),
        )
    }

    fn repositories(&self) -> Box<dyn Repositories> {
        Box::<GitHubRepositories>::default()
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

pub fn provider() -> GitHubProvider {
    GitHubProvider
}

#[cfg(test)]
mod tests {
    use super::{DISPLAY_NAME, GitHubProvider, PROVIDER_ID};
    use vcs_provider_core::{
        AuthHeaderStyle, AuthKind, Capability, Provider, ProviderRegistry, RepositoryCoordinates,
        VcsError, VcsResult,
    };

    #[test]
    fn github_provider_exposes_provider_descriptor() {
        let descriptor = GitHubProvider.descriptor();

        assert_eq!(descriptor.id().as_str(), PROVIDER_ID);
        assert_eq!(descriptor.display_name(), DISPLAY_NAME);
        assert!(
            descriptor
                .capabilities()
                .supports(&Capability::Repositories)
        );
    }

    #[test]
    fn github_provider_uses_bearer_auth_for_tokens() {
        let style = GitHubProvider.auth_header_style(AuthKind::PersonalAccessToken);

        assert_eq!(style, AuthHeaderStyle::AuthorizationBearer);
    }

    #[test]
    fn github_provider_registers_through_core_registry() -> VcsResult<()> {
        let registry = ProviderRegistry::builder()
            .register(super::provider())?
            .build();

        assert!(registry.contains_provider(&vcs_provider_core::ProviderId::make(PROVIDER_ID)));

        Ok(())
    }

    #[test]
    fn github_provider_exposes_repositories_contract() -> VcsResult<()> {
        let repositories = GitHubProvider.repositories();
        let coordinates = RepositoryCoordinates::make()
            .owner_name("akira-io")
            .name("vcs-providers-rs")
            .build()?;
        let result = futures::executor::block_on(repositories.get(coordinates));

        assert_eq!(result, Err(VcsError::TransportNotConfigured));

        Ok(())
    }
}
