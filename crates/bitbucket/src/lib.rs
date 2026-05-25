use vcs_provider_core::{
    AuthHeaderStyle, AuthKind, Capability, CapabilitySet, ProviderDescriptor, ProviderDriver,
    ProviderId,
};

pub const PROVIDER_ID: &str = "bitbucket";
pub const DISPLAY_NAME: &str = "Bitbucket";
pub const DEFAULT_BASE_URL: &str = "https://api.bitbucket.org/2.0";

#[derive(Clone, Copy, Debug, Default)]
pub struct BitbucketDriver;

impl ProviderDriver for BitbucketDriver {
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

pub fn driver() -> BitbucketDriver {
    BitbucketDriver
}

#[cfg(test)]
mod tests {
    use super::{BitbucketDriver, DISPLAY_NAME, PROVIDER_ID};
    use vcs_provider_core::{
        AuthHeaderStyle, AuthKind, Capability, ProviderDriver, ProviderId, ProviderRegistry,
        VcsResult,
    };

    #[test]
    fn bitbucket_driver_exposes_provider_descriptor() {
        let descriptor = BitbucketDriver.descriptor();

        assert_eq!(descriptor.id().as_str(), PROVIDER_ID);
        assert_eq!(descriptor.display_name(), DISPLAY_NAME);
        assert!(descriptor.capabilities().supports(&Capability::Pipelines));
    }

    #[test]
    fn bitbucket_driver_uses_bearer_auth_for_oauth() {
        let style = BitbucketDriver.auth_header_style(AuthKind::OAuth);

        assert_eq!(style, AuthHeaderStyle::AuthorizationBearer);
    }

    #[test]
    fn bitbucket_driver_registers_through_core_registry() -> VcsResult<()> {
        let registry = ProviderRegistry::builder()
            .register(super::driver())?
            .build();

        assert!(registry.contains_provider(&ProviderId::make(PROVIDER_ID)));

        Ok(())
    }
}
