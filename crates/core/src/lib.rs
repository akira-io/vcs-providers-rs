use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

mod auth;
mod code_reviews;
mod errors;
mod helpers;
mod issues;
mod manager;
mod middleware;
mod pagination;
mod pipelines;
mod rate_limit;
mod registry;
mod releases;
mod repos;
mod runtime;
mod telemetry;
mod testing;
mod transport;
mod url;

pub use auth::{
    AuthBuilder, AuthCredential, AuthHeader, AuthHeaderName, AuthHeaderStyle, AuthHeaderValue,
    AuthKind, AuthToken,
};
pub use code_reviews::{
    CodeReview, CodeReviewBuilder, CodeReviewDraft, CodeReviewDraftBuilder, CodeReviewId,
    CodeReviewListQuery, CodeReviewPatch, CodeReviewPatchBuilder, CodeReviewQueryBuilder,
    CodeReviews, MissingCodeReviewDraftRepo, MissingCodeReviewId, MissingCodeReviewRepo,
    MissingCodeReviewTitle, ProvidedCodeReviewDraftRepo, ProvidedCodeReviewId,
    ProvidedCodeReviewRepo, ProvidedCodeReviewTitle, TransportNotConfiguredCodeReviews,
};
pub(crate) use errors::transport_not_configured;
pub use errors::{ErrorBuilder, ErrorKind, VcsError, VcsResult};
pub use helpers::{
    CapabilitySetBuilder, auth, branch, capabilities, code_review, commit, error, issue,
    middleware, pagination, pipeline, provider, rate_limit, release, repo, request, response,
    runtime, telemetry, url, vcs,
};
pub use issues::{
    Issue, IssueBuilder, IssueDraft, IssueDraftBuilder, IssueId, IssueListQuery, IssuePatch,
    IssuePatchBuilder, IssueQueryBuilder, Issues, MissingIssueId, MissingIssueRepo,
    MissingIssueTitle, ProvidedIssueId, ProvidedIssueRepo, ProvidedIssueTitle,
    TransportNotConfiguredIssues,
};
pub use manager::{
    ManagedCodeReview, ManagedCodeReviewBuilder, ManagedCodeReviewCollection,
    ManagedCodeReviewDeleteProvider, ManagedCodeReviewDraftBuilder, ManagedCodeReviewProvider,
    ManagedIssue, ManagedIssueBuilder, ManagedIssueCollection, ManagedIssueDeleteProvider,
    ManagedIssueDraftBuilder, ManagedIssueProvider, ManagedProvider, ManagedRelease,
    ManagedReleaseBuilder, ManagedReleaseCollection, ManagedReleaseDraftBuilder,
    ManagedReleaseProvider, ManagedRepo, ManagedRepoBuilder, ManagedRepoCodeReviews,
    ManagedRepoCodeReviewsPagination, ManagedRepoCollection, ManagedRepoIssues,
    ManagedRepoIssuesPagination, ManagedRepoReleases, ManagedRepoReleasesPagination,
    ManagedRepositoryDraftBuilder, VcsManager, VcsManagerBuilder, VcsManagerWithDriverBuilder,
};
pub use middleware::{
    HeaderMiddleware, Middleware, MissingTransport, ProvidedTransport, TransportPipeline,
    TransportPipelineBuilder,
};
pub use pagination::{
    Page, PageBuilder, PageCursor, PageLimit, PageRequest, PageRequestBuilder, PaginationBuilder,
};
pub use pipelines::{
    MissingPipelineId, MissingPipelineRepo, Pipeline, PipelineBuilder, PipelineId,
    PipelineListQuery, PipelineQueryBuilder, Pipelines, ProvidedPipelineId, ProvidedPipelineRepo,
    TransportNotConfiguredPipelines,
};
pub use rate_limit::{
    RateLimitBuilder, RateLimitCost, RateLimitHeaderName, RateLimitHeaderProfile,
    RateLimitHeaderProfileBuilder, RateLimitObservation, RateLimitQuota, RateLimitReset,
    RetryAfter,
};
pub use registry::{ProviderRegistry, ProviderRegistryBuilder};
pub use releases::{
    MissingReleaseId, MissingReleaseRepo, MissingReleaseTag, ProvidedReleaseId,
    ProvidedReleaseRepo, ProvidedReleaseTag, Release, ReleaseBuilder, ReleaseDraft,
    ReleaseDraftBuilder, ReleaseId, ReleaseListQuery, ReleasePatch, ReleasePatchBuilder,
    ReleaseQueryBuilder, Releases, TransportNotConfiguredReleases,
};
pub use repos::{
    BoxFuture, Branch, Commit, LifecycleState, MissingLifecycleState, MissingOwnerName,
    MissingRepositoryName, MissingVisibility, OwnerName, ProvidedLifecycleState, ProvidedOwnerName,
    ProvidedProviderId, ProvidedRepositoryName, ProvidedVisibility, Repo, RepoBuilder,
    RepoQueryBuilder, Repos, Repository, RepositoryBuilder, RepositoryDraft,
    RepositoryDraftBuilder, RepositoryListQuery, RepositoryName, RepositoryPatch,
    RepositoryPatchBuilder, RepositoryResponseMapper, RepositorySearchQuery, TransportBackedRepos,
    TransportNotConfiguredRepos, Visibility,
};
pub use runtime::{
    IntoProvider, MissingProviderTransport, ProvidedProviderTransport, ProviderRequestBuilder,
    ProviderRuntime, ProviderRuntimeBuilder, ProviderRuntimeWithProviderBuilder,
    RuntimeConfiguredProvider, RuntimeProviderConfigurationBuilder, transport_status_error,
};
pub use telemetry::{
    MissingTelemetrySink, ProvidedTelemetrySink, ProvidedTelemetryTransport, RequestTelemetry,
    RequestTelemetryBuilder, ResponseTelemetry, ResponseTelemetryBuilder, TelemetryBuilder,
    TelemetryEvent, TelemetryRecorder, TelemetrySink, TelemetryTransport,
    TelemetryTransportBuilder,
};
pub use testing::{
    EchoTransport, ProviderResponseBuilder, ProviderResponseTransportBuilder,
    SingleResponseTransport, provider_response, run_async_test,
};
pub use transport::{
    Request, RequestBody, RequestBuilder, RequestHeader, RequestHeaderName, RequestHeaderValue,
    RequestMethod, Response, ResponseBody, ResponseBuilder, ResponseStatus, Transport,
};
pub use url::{RequestUrl, RequestUrlBuilder};

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderDescriptorBuilder {
    id: ProviderId,
    display_name: String,
    capabilities: CapabilitySet,
}

impl ProviderDescriptorBuilder {
    pub fn make(id: impl Into<String>) -> Self {
        Self {
            id: ProviderId::make(id),
            display_name: String::new(),
            capabilities: CapabilitySet::default(),
        }
    }

    pub fn display_name(mut self, display_name: impl Into<String>) -> Self {
        self.display_name = display_name.into();
        self
    }

    pub fn capabilities(mut self, capabilities: CapabilitySet) -> Self {
        self.capabilities = capabilities;
        self
    }

    pub fn build(self) -> ProviderDescriptor {
        ProviderDescriptor {
            id: self.id,
            display_name: self.display_name,
            capabilities: self.capabilities,
        }
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

pub trait Provider: Send + Sync {
    fn descriptor(&self) -> ProviderDescriptor;

    fn repos(&self) -> Box<dyn Repos>;

    fn issues(&self) -> Box<dyn Issues>;

    fn code_reviews(&self) -> Box<dyn CodeReviews>;

    fn pipelines(&self) -> Box<dyn Pipelines>;

    fn releases(&self) -> Box<dyn Releases>;

    fn capabilities(&self) -> CapabilitySet {
        self.descriptor().capabilities().clone()
    }

    fn default_base_url(&self) -> &str;

    fn auth_header_style(&self, auth_kind: AuthKind) -> AuthHeaderStyle;

    fn auth_header(&self, credential: &AuthCredential) -> Option<AuthHeader> {
        credential.header(self.auth_header_style(credential.kind()))
    }
}
