use std::collections::BTreeMap;
use std::sync::Arc;

use crate::{Capability, Provider, ProviderDescriptor, ProviderId, VcsError, VcsResult};

#[derive(Clone, Default)]
pub struct ProviderRegistry {
    providers: BTreeMap<ProviderId, Arc<dyn Provider>>,
}

impl ProviderRegistry {
    pub fn builder() -> ProviderRegistryBuilder {
        ProviderRegistryBuilder::default()
    }

    pub fn get_provider(&self, id: &ProviderId) -> VcsResult<Arc<dyn Provider>> {
        match self.providers.get(id) {
            Some(provider) => Ok(Arc::clone(provider)),
            None => Err(VcsError::ProviderNotRegistered(id.as_str().into())),
        }
    }

    pub fn contains_provider(&self, id: &ProviderId) -> bool {
        self.providers.contains_key(id)
    }

    pub fn descriptors(&self) -> impl Iterator<Item = ProviderDescriptor> + '_ {
        self.providers
            .values()
            .map(|provider| provider.descriptor())
    }

    pub fn providers_supporting(
        &self,
        capability: Capability,
    ) -> impl Iterator<Item = Arc<dyn Provider>> + '_ {
        self.providers.values().filter_map(move |provider| {
            let descriptor = provider.descriptor();

            descriptor
                .capabilities()
                .supports(&capability)
                .then(|| Arc::clone(provider))
        })
    }
}

#[derive(Default)]
pub struct ProviderRegistryBuilder {
    providers: BTreeMap<ProviderId, Arc<dyn Provider>>,
}

impl ProviderRegistryBuilder {
    pub fn register(mut self, provider: impl Provider + 'static) -> VcsResult<Self> {
        let descriptor = provider.descriptor();
        let id = descriptor.id().clone();

        match self.providers.contains_key(&id) {
            true => Err(VcsError::ProviderAlreadyRegistered(id.as_str().into())),
            false => {
                self.providers.insert(id, Arc::new(provider));
                Ok(self)
            }
        }
    }

    pub fn build(self) -> ProviderRegistry {
        ProviderRegistry {
            providers: self.providers,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        AuthHeaderStyle, AuthKind, BoxFuture, Branch, Capability, CapabilitySet, Commit, Page,
        Provider, ProviderDescriptor, ProviderId, ProviderRegistry, Repositories, Repository,
        RepositoryCoordinates, RepositoryListQuery, RepositorySearchQuery, VcsError, VcsResult,
    };

    #[derive(Clone, Copy)]
    struct TestProvider;

    #[derive(Clone, Copy)]
    struct TestRepositories;

    impl Repositories for TestRepositories {
        fn get(&self, _coordinates: RepositoryCoordinates) -> BoxFuture<'_, VcsResult<Repository>> {
            Box::pin(async { Err(VcsError::TransportNotConfigured) })
        }

        fn list(&self, _query: RepositoryListQuery) -> BoxFuture<'_, VcsResult<Page<Repository>>> {
            Box::pin(async { Err(VcsError::TransportNotConfigured) })
        }

        fn search(
            &self,
            _query: RepositorySearchQuery,
        ) -> BoxFuture<'_, VcsResult<Page<Repository>>> {
            Box::pin(async { Err(VcsError::TransportNotConfigured) })
        }

        fn branches(
            &self,
            _coordinates: RepositoryCoordinates,
        ) -> BoxFuture<'_, VcsResult<Page<Branch>>> {
            Box::pin(async { Err(VcsError::TransportNotConfigured) })
        }

        fn commits(
            &self,
            _coordinates: RepositoryCoordinates,
        ) -> BoxFuture<'_, VcsResult<Page<Commit>>> {
            Box::pin(async { Err(VcsError::TransportNotConfigured) })
        }
    }

    impl Provider for TestProvider {
        fn descriptor(&self) -> ProviderDescriptor {
            ProviderDescriptor::make(
                ProviderId::make("test"),
                "Test",
                CapabilitySet::make([Capability::Repositories]),
            )
        }

        fn repositories(&self) -> Box<dyn Repositories> {
            Box::new(TestRepositories)
        }

        fn default_base_url(&self) -> &str {
            "https://vcs.example.test"
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

    #[test]
    fn registry_returns_registered_provider() -> VcsResult<()> {
        let registry = ProviderRegistry::builder().register(TestProvider)?.build();
        let provider = registry.get_provider(&ProviderId::make("test"))?;

        assert_eq!(provider.descriptor().display_name(), "Test");

        Ok(())
    }

    #[test]
    fn registry_rejects_duplicate_provider_ids() -> VcsResult<()> {
        let result = ProviderRegistry::builder()
            .register(TestProvider)?
            .register(TestProvider);

        assert_eq!(
            result.err(),
            Some(VcsError::ProviderAlreadyRegistered("test".into()))
        );

        Ok(())
    }

    #[test]
    fn registry_filters_providers_by_capability() -> VcsResult<()> {
        let registry = ProviderRegistry::builder().register(TestProvider)?.build();
        let providers = registry
            .providers_supporting(Capability::Repositories)
            .collect::<Vec<_>>();

        assert_eq!(providers.len(), 1);

        Ok(())
    }
}
