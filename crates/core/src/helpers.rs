use crate::repos::{MissingOwnerName, MissingRepositoryName};
use crate::{
    AuthBuilder, Capability, CapabilitySet, ProviderRegistry, ProviderRegistryBuilder,
    RateLimitBuilder, RepoBuilder, RequestBuilder, TelemetryBuilder, TransportPipeline,
    TransportPipelineBuilder,
};

pub fn auth() -> AuthBuilder {
    AuthBuilder
}

pub fn capabilities() -> CapabilitySetBuilder {
    CapabilitySetBuilder
}

pub fn repo() -> RepoBuilder<MissingOwnerName, MissingRepositoryName> {
    RepoBuilder {
        owner_name: MissingOwnerName,
        repository_name: MissingRepositoryName,
    }
}

pub fn provider() -> ProviderRegistryBuilder {
    ProviderRegistry::builder()
}

pub fn request() -> RequestBuilder {
    RequestBuilder::default()
}

pub fn rate_limit() -> RateLimitBuilder {
    RateLimitBuilder
}

pub fn middleware() -> TransportPipelineBuilder<crate::middleware::MissingTransport> {
    TransportPipeline::builder()
}

pub fn telemetry() -> TelemetryBuilder {
    TelemetryBuilder
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CapabilitySetBuilder;

impl CapabilitySetBuilder {
    pub fn make(self, capabilities: impl IntoIterator<Item = Capability>) -> CapabilitySet {
        CapabilitySet {
            capabilities: capabilities.into_iter().collect(),
        }
    }
}
