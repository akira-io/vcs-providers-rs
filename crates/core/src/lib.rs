use std::fmt;

use serde::{Deserialize, Serialize};

mod auth;
mod capability;
mod code_reviews;
mod errors;
mod helpers;
mod http;
mod issues;
mod manager;
mod middleware;
mod pagination;
mod pipelines;
mod rate_limit;
mod registry;
mod releases;
mod repos;
mod retry;
mod runtime;
mod telemetry;
mod testing;
mod transport;
mod url;

pub use auth::{
    AuthBuilder, AuthCredential, AuthHeader, AuthHeaderName, AuthHeaderStyle, AuthHeaderValue,
    AuthKind, AuthToken,
};
pub use capability::{Capability, CapabilitySet};
pub use code_reviews::{
    CodeReview, CodeReviewBuilder, CodeReviewDeleteOperation, CodeReviewDraft,
    CodeReviewDraftBuilder, CodeReviewId, CodeReviewListOperation,
    CodeReviewListPaginationOperation, CodeReviewListQuery, CodeReviewPatch,
    CodeReviewPatchBuilder, CodeReviewQueryBuilder, CodeReviewResponseMapper, CodeReviews,
    CodeReviewsFluent, MissingCodeReviewDraftRepo, MissingCodeReviewId, MissingCodeReviewRepo,
    MissingCodeReviewTitle, ProvidedCodeReviewDraftRepo, ProvidedCodeReviewId,
    ProvidedCodeReviewRepo, ProvidedCodeReviewTitle, ScopedCodeReviewOperation,
    TransportBackedCodeReviews, TransportNotConfiguredCodeReviews,
};
pub(crate) use errors::transport_not_configured;
pub use errors::{ErrorBuilder, ErrorKind, VcsError, VcsResult};
#[cfg(feature = "testing")]
pub use helpers::conformance;
pub use helpers::{
    CapabilitySetBuilder, auth, branch, capabilities, code_review, commit, error, http, issue,
    issue_id, middleware, pagination, pipeline, provider, provider_id, rate_limit, release,
    release_id, repo, request, request_body, response, retry, runtime, telemetry, url, vcs,
};
pub use http::{HttpBuilder, HttpTransport, HttpTransportBuilder};
pub use issues::{
    Issue, IssueBuilder, IssueDeleteOperation, IssueDraft, IssueDraftBuilder, IssueId,
    IssueListOperation, IssueListPaginationOperation, IssueListQuery, IssuePatch,
    IssuePatchBuilder, IssueQueryBuilder, IssueResponseMapper, Issues, IssuesFluent,
    MissingIssueId, MissingIssueRepo, MissingIssueTitle, ProvidedIssueId, ProvidedIssueRepo,
    ProvidedIssueTitle, ScopedIssueOperation, TransportBackedIssues, TransportNotConfiguredIssues,
    UnsupportedIssues,
};
pub use manager::{
    ManagedClientProvider, ManagedCodeReview, ManagedCodeReviewBuilder,
    ManagedCodeReviewCollection, ManagedCodeReviewDraftBuilder, ManagedCodeReviewProvider,
    ManagedIssue, ManagedIssueBuilder, ManagedIssueCollection, ManagedIssueDraftBuilder,
    ManagedIssueProvider, ManagedIssueUpdateBuilder, ManagedMiddlewareTransportBuilder,
    ManagedPipeline, ManagedPipelineBuilder, ManagedPipelineCollection, ManagedProvider,
    ManagedRateLimitTransportBuilder, ManagedRelease, ManagedReleaseBuilder,
    ManagedReleaseCollection, ManagedReleaseDraftBuilder, ManagedReleaseProvider,
    ManagedReleaseUpdateBuilder, ManagedRepo, ManagedRepoBuilder, ManagedRepoCodeReviews,
    ManagedRepoCodeReviewsPagination, ManagedRepoCollection, ManagedRepoIssues,
    ManagedRepoIssuesPagination, ManagedRepoPipelines, ManagedRepoPipelinesPagination,
    ManagedRepoReleases, ManagedRepoReleasesPagination, ManagedRepositoryDraftBuilder,
    ManagedRepositoryUpdateBuilder, ManagedRetryTransportBuilder, ProviderClient, VcsManager,
    VcsManagerBuilder, VcsManagerWithDriverBuilder,
};
pub use middleware::{
    HeaderMiddleware, Middleware, MissingTransport, ProvidedTransport, TransportPipeline,
    TransportPipelineBuilder,
};
pub use pagination::{
    Page, PageBuilder, PageCursor, PageLimit, PageRequest, PageRequestBuilder, PaginationBuilder,
};
pub use pipelines::{
    ManagedPipelineProvider, MissingPipelineId, MissingPipelineRepo, Pipeline, PipelineBuilder,
    PipelineId, PipelineListQuery, PipelinePaginationOperation, PipelineQueryBuilder,
    PipelineResponseMapper, Pipelines, PipelinesFluent, ProvidedPipelineId, ProvidedPipelineRepo,
    ScopedPipelineOperation, TransportBackedPipelines, TransportNotConfiguredPipelines,
};
pub use rate_limit::{
    RateLimitBuilder, RateLimitCost, RateLimitHeaderName, RateLimitHeaderProfile,
    RateLimitHeaderProfileBuilder, RateLimitObservation, RateLimitQuota, RateLimitRecorder,
    RateLimitReset, RateLimitSink, RateLimitTransport, RateLimitTransportBuilder, RetryAfter,
};
pub use registry::{ProviderRegistry, ProviderRegistryBuilder};
pub use releases::{
    MissingReleaseId, MissingReleaseRepo, MissingReleaseTag, ProvidedReleaseId,
    ProvidedReleaseRepo, ProvidedReleaseTag, Release, ReleaseBuilder, ReleaseDraft,
    ReleaseDraftBuilder, ReleaseId, ReleaseListOperation, ReleaseListPaginationOperation,
    ReleaseListQuery, ReleasePatch, ReleasePatchBuilder, ReleaseQueryBuilder,
    ReleaseResponseMapper, Releases, ReleasesFluent, ScopedReleaseOperation,
    TransportBackedReleases, TransportNotConfiguredReleases, UnsupportedReleases,
};
pub use repos::{
    BoxFuture, Branch, Commit, LifecycleState, MissingLifecycleState, MissingOwnerName,
    MissingRepositoryName, MissingVisibility, OwnerName, ProvidedLifecycleState, ProvidedOwnerName,
    ProvidedProviderId, ProvidedRepositoryName, ProvidedVisibility, Repo, RepoBuilder,
    RepoCreateOperation, RepoQueryBuilder, RepoUpdateOperation, Repos, ReposFluent, Repository,
    RepositoryBuilder, RepositoryDraft, RepositoryDraftBuilder, RepositoryListQuery,
    RepositoryName, RepositoryPatch, RepositoryPatchBuilder, RepositoryResponseMapper,
    RepositorySearchQuery, TransportBackedRepos, TransportNotConfiguredRepos, Visibility,
};
pub use retry::{
    ProvidedRetryTransport, RetryBuilder, RetryPolicy, RetryTransport, RetryTransportBuilder,
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
    EchoTransport, RecordingTransport, ResponseSequenceTransport, SingleResponseTransport,
    TestTransportBuilder, TestTransportResponseBuilder, TestTransportSequenceBuilder,
    TestTransportSequenceResponseBuilder, run_async_test, test_transport,
};
#[cfg(feature = "testing")]
pub use testing::{ProviderConformance, ProviderConformanceBuilder};
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
