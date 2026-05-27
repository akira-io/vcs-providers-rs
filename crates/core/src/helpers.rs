use crate::repos::{MissingOwnerName, MissingRepositoryName};
use crate::{
    AuthBuilder, Branch, Capability, CapabilitySet, CodeReviewBuilder, Commit, ErrorBuilder,
    HttpBuilder, IssueBuilder, MissingCodeReviewId, MissingCodeReviewRepo, MissingIssueId,
    MissingIssueRepo, MissingPipelineId, MissingPipelineRepo, MissingReleaseId, MissingReleaseRepo,
    PaginationBuilder, PipelineBuilder, ProviderRegistry, ProviderRegistryBuilder,
    ProviderRuntimeBuilder, RateLimitBuilder, ReleaseBuilder, RepoBuilder, RequestBuilder,
    RequestUrlBuilder, ResponseBuilder, RetryBuilder, TelemetryBuilder, TransportPipeline,
    TransportPipelineBuilder, VcsManagerBuilder,
};

#[cfg(feature = "testing")]
use crate::ProviderConformanceBuilder;

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

pub fn issue() -> IssueBuilder<MissingIssueRepo, MissingIssueId> {
    crate::Issue::builder()
}

pub fn http() -> HttpBuilder {
    HttpBuilder
}

pub fn code_review() -> CodeReviewBuilder<MissingCodeReviewRepo, MissingCodeReviewId> {
    crate::CodeReview::builder()
}

pub fn pipeline() -> PipelineBuilder<MissingPipelineRepo, MissingPipelineId> {
    crate::Pipeline::builder()
}

pub fn release() -> ReleaseBuilder<MissingReleaseRepo, MissingReleaseId> {
    crate::Release::builder()
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

#[cfg(feature = "testing")]
pub fn conformance() -> ProviderConformanceBuilder {
    crate::testing::conformance()
}

pub fn vcs<Driver>(driver: Driver) -> crate::VcsManager<Driver>
where
    Driver: crate::ManagedProvider,
{
    VcsManagerBuilder.driver(driver).build()
}

pub fn runtime() -> ProviderRuntimeBuilder {
    ProviderRuntimeBuilder
}

pub fn request() -> RequestBuilder {
    RequestBuilder::default()
}

pub fn response() -> ResponseBuilder {
    ResponseBuilder::default()
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

pub fn retry() -> RetryBuilder {
    RetryBuilder
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
        CapabilitySet::make(capabilities)
    }
}
