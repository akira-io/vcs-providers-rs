use std::collections::BTreeMap;
use std::sync::Arc;

use crate::{Capability, ProviderDescriptor, ProviderDriver, ProviderId, VcsError, VcsResult};

#[derive(Clone, Default)]
pub struct ProviderRegistry {
    drivers: BTreeMap<ProviderId, Arc<dyn ProviderDriver>>,
}

impl ProviderRegistry {
    pub fn builder() -> ProviderRegistryBuilder {
        ProviderRegistryBuilder::default()
    }

    pub fn get_driver(&self, id: &ProviderId) -> VcsResult<Arc<dyn ProviderDriver>> {
        match self.drivers.get(id) {
            Some(driver) => Ok(Arc::clone(driver)),
            None => Err(VcsError::ProviderNotRegistered(id.as_str().into())),
        }
    }

    pub fn contains_provider(&self, id: &ProviderId) -> bool {
        self.drivers.contains_key(id)
    }

    pub fn descriptors(&self) -> impl Iterator<Item = ProviderDescriptor> + '_ {
        self.drivers.values().map(|driver| driver.descriptor())
    }

    pub fn drivers_supporting(
        &self,
        capability: Capability,
    ) -> impl Iterator<Item = Arc<dyn ProviderDriver>> + '_ {
        self.drivers.values().filter_map(move |driver| {
            let descriptor = driver.descriptor();

            descriptor
                .capabilities()
                .supports(&capability)
                .then(|| Arc::clone(driver))
        })
    }
}

#[derive(Default)]
pub struct ProviderRegistryBuilder {
    drivers: BTreeMap<ProviderId, Arc<dyn ProviderDriver>>,
}

impl ProviderRegistryBuilder {
    pub fn register(mut self, driver: impl ProviderDriver + 'static) -> VcsResult<Self> {
        let descriptor = driver.descriptor();
        let id = descriptor.id().clone();

        match self.drivers.contains_key(&id) {
            true => Err(VcsError::ProviderAlreadyRegistered(id.as_str().into())),
            false => {
                self.drivers.insert(id, Arc::new(driver));
                Ok(self)
            }
        }
    }

    pub fn build(self) -> ProviderRegistry {
        ProviderRegistry {
            drivers: self.drivers,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        AuthHeaderStyle, AuthKind, Capability, CapabilitySet, ProviderDescriptor, ProviderDriver,
        ProviderId, ProviderRegistry, VcsError, VcsResult,
    };

    #[derive(Clone, Copy)]
    struct TestDriver;

    impl ProviderDriver for TestDriver {
        fn descriptor(&self) -> ProviderDescriptor {
            ProviderDescriptor::make(
                ProviderId::make("test"),
                "Test",
                CapabilitySet::make([Capability::Repositories]),
            )
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
    fn registry_returns_registered_driver() -> VcsResult<()> {
        let registry = ProviderRegistry::builder().register(TestDriver)?.build();
        let driver = registry.get_driver(&ProviderId::make("test"))?;

        assert_eq!(driver.descriptor().display_name(), "Test");

        Ok(())
    }

    #[test]
    fn registry_rejects_duplicate_provider_ids() -> VcsResult<()> {
        let result = ProviderRegistry::builder()
            .register(TestDriver)?
            .register(TestDriver);

        assert_eq!(
            result.err(),
            Some(VcsError::ProviderAlreadyRegistered("test".into()))
        );

        Ok(())
    }

    #[test]
    fn registry_filters_drivers_by_capability() -> VcsResult<()> {
        let registry = ProviderRegistry::builder().register(TestDriver)?.build();
        let drivers = registry
            .drivers_supporting(Capability::Repositories)
            .collect::<Vec<_>>();

        assert_eq!(drivers.len(), 1);

        Ok(())
    }
}
