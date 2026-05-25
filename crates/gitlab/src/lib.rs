use vcs_provider_core::{
    AuthHeaderStyle, AuthKind, Capability, CapabilitySet, ProviderDescriptor, ProviderDriver,
    ProviderId,
};

pub const PROVIDER_ID: &str = "gitlab";
pub const DISPLAY_NAME: &str = "GitLab";
pub const DEFAULT_BASE_URL: &str = "https://gitlab.com";

#[derive(Clone, Copy, Debug, Default)]
pub struct GitLabDriver;

impl ProviderDriver for GitLabDriver {
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
                Capability::Webhooks,
                Capability::SelfHosted,
            ]),
        )
    }

    fn default_base_url(&self) -> &str {
        DEFAULT_BASE_URL
    }

    fn auth_header_style(&self, auth_kind: AuthKind) -> AuthHeaderStyle {
        match auth_kind {
            AuthKind::Anonymous => AuthHeaderStyle::None,
            AuthKind::PersonalAccessToken => AuthHeaderStyle::CustomHeader("private-token".into()),
            AuthKind::OAuth => AuthHeaderStyle::AuthorizationBearer,
            AuthKind::AppInstallation => AuthHeaderStyle::AuthorizationBearer,
            AuthKind::Jwt => AuthHeaderStyle::AuthorizationBearer,
        }
    }
}

pub fn driver() -> GitLabDriver {
    GitLabDriver
}

#[cfg(test)]
mod tests {
    use super::{DISPLAY_NAME, GitLabDriver, PROVIDER_ID};
    use vcs_provider_core::{
        AuthHeaderStyle, AuthKind, Capability, ProviderDriver, ProviderId, ProviderRegistry,
        VcsResult,
    };

    #[test]
    fn gitlab_driver_exposes_provider_descriptor() {
        let descriptor = GitLabDriver.descriptor();

        assert_eq!(descriptor.id().as_str(), PROVIDER_ID);
        assert_eq!(descriptor.display_name(), DISPLAY_NAME);
        assert!(descriptor.capabilities().supports(&Capability::SelfHosted));
    }

    #[test]
    fn gitlab_driver_uses_private_token_header_for_personal_access_tokens() {
        let style = GitLabDriver.auth_header_style(AuthKind::PersonalAccessToken);

        assert_eq!(style, AuthHeaderStyle::CustomHeader("private-token".into()));
    }

    #[test]
    fn gitlab_driver_registers_through_core_registry() -> VcsResult<()> {
        let registry = ProviderRegistry::builder()
            .register(super::driver())?
            .build();

        assert!(registry.contains_provider(&ProviderId::make(PROVIDER_ID)));

        Ok(())
    }
}
