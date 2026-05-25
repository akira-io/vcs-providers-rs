use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

mod auth;
mod helpers;
mod middleware;
mod rate_limit;
mod registry;
mod repos;
mod telemetry;
mod transport;

pub use auth::{
    AuthBuilder, AuthCredential, AuthHeader, AuthHeaderName, AuthHeaderStyle, AuthHeaderValue,
    AuthKind, AuthToken,
};
pub use helpers::{
    CapabilitySetBuilder, auth, capabilities, middleware, provider, rate_limit, repo, request,
    telemetry,
};
pub use middleware::{
    HeaderMiddleware, Middleware, MissingTransport, ProvidedTransport, TransportPipeline,
    TransportPipelineBuilder,
};
pub use rate_limit::{
    RateLimitBuilder, RateLimitCost, RateLimitHeaderName, RateLimitHeaderProfile,
    RateLimitHeaderProfileBuilder, RateLimitObservation, RateLimitQuota, RateLimitReset,
    RetryAfter,
};
pub use registry::{ProviderRegistry, ProviderRegistryBuilder};
pub use repos::{
    BoxFuture, Branch, Commit, LifecycleState, OwnerName, Page, Repo, RepoBuilder, Repos,
    Repository, RepositoryBuilder, RepositoryListQuery, RepositoryName, RepositorySearchQuery,
    TransportNotConfiguredRepos, Visibility,
};
pub use telemetry::{
    MissingTelemetrySink, ProvidedTelemetrySink, ProvidedTelemetryTransport, RequestTelemetry,
    RequestTelemetryBuilder, ResponseTelemetry, ResponseTelemetryBuilder, TelemetryBuilder,
    TelemetryEvent, TelemetryRecorder, TelemetrySink, TelemetryTransport,
    TelemetryTransportBuilder,
};
pub use transport::{
    Request, RequestBuilder, RequestHeader, RequestHeaderName, RequestHeaderValue, RequestMethod,
    RequestUrl, Response, ResponseStatus, Transport,
};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ProviderId(String);

impl ProviderId {
    pub fn make(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ProviderId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProviderDescriptor {
    id: ProviderId,
    display_name: String,
    capabilities: CapabilitySet,
}

impl ProviderDescriptor {
    pub fn make(
        id: ProviderId,
        display_name: impl Into<String>,
        capabilities: CapabilitySet,
    ) -> Self {
        Self {
            id,
            display_name: display_name.into(),
            capabilities,
        }
    }

    pub fn id(&self) -> &ProviderId {
        &self.id
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn capabilities(&self) -> &CapabilitySet {
        &self.capabilities
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Capability {
    Repos,
    Issues,
    CodeReviews,
    Pipelines,
    Releases,
    Organizations,
    Discussions,
    Webhooks,
    SelfHosted,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct CapabilitySet {
    capabilities: BTreeSet<Capability>,
}

impl CapabilitySet {
    pub fn make(capabilities: impl IntoIterator<Item = Capability>) -> Self {
        Self {
            capabilities: capabilities.into_iter().collect(),
        }
    }

    pub fn supports(&self, capability: &Capability) -> bool {
        self.capabilities.contains(capability)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Capability> {
        self.capabilities.iter()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum VcsError {
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict,
    RateLimited,
    ProviderUnavailable,
    TransportNotConfigured,
    ProviderAlreadyRegistered(String),
    ProviderNotRegistered(String),
    InvalidInput(String),
}

pub type VcsResult<T> = Result<T, VcsError>;

pub trait Provider: Send + Sync {
    fn descriptor(&self) -> ProviderDescriptor;

    fn repos(&self) -> Box<dyn Repos>;

    fn default_base_url(&self) -> &str;

    fn auth_header_style(&self, auth_kind: AuthKind) -> AuthHeaderStyle;

    fn auth_header(&self, credential: &AuthCredential) -> Option<AuthHeader> {
        credential.header(self.auth_header_style(credential.kind()))
    }
}
