use crate::repos::{MissingOwnerName, MissingRepositoryName};
use crate::{
    AuthBuilder, Branch, Capability, CapabilitySet, Commit, ErrorBuilder, PaginationBuilder,
    ProviderRegistry, ProviderRegistryBuilder, RateLimitBuilder, RepoBuilder, RequestBuilder,
    RequestUrlBuilder, TelemetryBuilder, TransportPipeline, TransportPipelineBuilder,
};

pub fn auth() -> AuthBuilder {
    AuthBuilder
}

pub fn capabilities() -> CapabilitySetBuilder {
    CapabilitySetBuilder
}

pub fn error() -> ErrorBuilder {
    ErrorBuilder
}

pub fn repo() -> RepoBuilder<MissingOwnerName, MissingRepositoryName> {
    RepoBuilder {
        owner_name: MissingOwnerName,
        repository_name: MissingRepositoryName,
    }
}

pub fn branch(name: impl Into<String>) -> Branch {
    Branch::make(name)
}

pub fn commit(id: impl Into<String>) -> Commit {
    Commit::make(id)
}

pub fn provider() -> ProviderRegistryBuilder {
    ProviderRegistry::builder()
}

pub fn request() -> RequestBuilder {
    RequestBuilder::default()
}

pub fn url(base_url: impl Into<String>) -> RequestUrlBuilder {
    RequestUrlBuilder::make(base_url)
}

pub fn pagination() -> PaginationBuilder {
    PaginationBuilder
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
