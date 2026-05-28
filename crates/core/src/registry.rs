use std::collections::BTreeMap;
use std::sync::Arc;

use crate::{
    Capability, CognitionResult, Provider, ProviderDescriptor, ProviderDescriptorBuilder,
    ProviderId, error,
};

#[derive(Clone, Default)]
pub struct ProviderRegistry {
    providers: BTreeMap<ProviderId, Arc<dyn Provider>>,
}

impl ProviderRegistry {
    pub fn builder() -> ProviderRegistryBuilder {
        ProviderRegistryBuilder::default()
    }

    pub fn get_provider(&self, id: &ProviderId) -> CognitionResult<Arc<dyn Provider>> {
        match self.providers.get(id) {
            Some(provider) => Ok(Arc::clone(provider)),
            None => Err(error().provider_not_registered(id.as_str())),
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
    pub fn descriptor(&self, id: impl Into<String>) -> ProviderDescriptorBuilder {
        ProviderDescriptorBuilder::make(id)
    }

    pub fn register(mut self, provider: impl Provider + 'static) -> CognitionResult<Self> {
        let descriptor = provider.descriptor();
        let id = descriptor.id().clone();

        match self.providers.contains_key(&id) {
            true => Err(error().provider_already_registered(id.as_str())),
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
