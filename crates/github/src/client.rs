use std::sync::Arc;

use vcs_provider_core::{
    AuthCredential, CodeReviews, Issues, ManagedClientProvider, Pipelines, Provider,
    ProviderClient, ProviderDescriptor, Releases, Repos, RequestHeader, Transport,
    TransportBackedCodeReviews, TransportBackedIssues, TransportBackedPipelines,
    TransportBackedReleases, TransportBackedRepos,
};

use crate::mappers::{
    GitHubCodeReviewMapper, GitHubIssueMapper, GitHubPipelineMapper, GitHubReleaseMapper,
    GitHubRepositoryMapper,
};
use crate::{GitHubProvider, github};

#[derive(Clone)]
pub struct GitHubClient {
    provider: GitHubProvider,
    transport: Arc<dyn Transport>,
    headers: Vec<RequestHeader>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitHubPipelinesTransportBuilder {
    provider: GitHubProvider,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitHubReposTransportBuilder {
    provider: GitHubProvider,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitHubIssuesTransportBuilder {
    provider: GitHubProvider,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitHubCodeReviewsTransportBuilder {
    provider: GitHubProvider,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitHubReleasesTransportBuilder {
    provider: GitHubProvider,
}

impl GitHubClient {
    pub fn make(transport: impl Transport + 'static) -> Self {
        Self::with_provider(github(), transport)
    }

    pub fn with_provider(provider: GitHubProvider, transport: impl Transport + 'static) -> Self {
        Self {
            provider,
            transport: Arc::new(transport),
            headers: default_headers(),
        }
    }

    pub fn issues(&self) -> Box<dyn Issues> {
        Box::new(
            TransportBackedIssues::make(
                self.provider.clone(),
                Arc::clone(&self.transport),
                GitHubIssueMapper,
            )
            .with_headers(self.headers.clone()),
        )
    }

    pub fn code_reviews(&self) -> Box<dyn CodeReviews> {
        Box::new(
            TransportBackedCodeReviews::make(
                self.provider.clone(),
                Arc::clone(&self.transport),
                GitHubCodeReviewMapper,
            )
            .with_headers(self.headers.clone()),
        )
    }

    pub fn releases(&self) -> Box<dyn Releases> {
        Box::new(
            TransportBackedReleases::make(
                self.provider.clone(),
                Arc::clone(&self.transport),
                GitHubReleaseMapper,
            )
            .with_headers(self.headers.clone()),
        )
    }

    pub fn pipelines(&self) -> Box<dyn Pipelines> {
        Box::new(
            TransportBackedPipelines::make(
                self.provider.clone(),
                Arc::clone(&self.transport),
                GitHubPipelineMapper,
            )
            .with_headers(self.headers.clone()),
        )
    }

    pub fn repos(&self) -> Box<dyn Repos> {
        Box::new(
            TransportBackedRepos::make(
                self.provider.clone(),
                Arc::clone(&self.transport),
                GitHubRepositoryMapper,
            )
            .with_headers(self.headers.clone()),
        )
    }

    pub fn auth(mut self, credential: AuthCredential) -> Self {
        if let Some(header) = self.provider.auth_header(&credential) {
            self.headers.push(RequestHeader::make(
                header.name().as_str(),
                header.value().as_str(),
            ));
        }

        self
    }
}

impl Provider for GitHubClient {
    fn descriptor(&self) -> ProviderDescriptor {
        self.provider.descriptor()
    }

    fn repos(&self) -> Box<dyn Repos> {
        GitHubClient::repos(self)
    }

    fn issues(&self) -> Box<dyn Issues> {
        GitHubClient::issues(self)
    }

    fn code_reviews(&self) -> Box<dyn CodeReviews> {
        GitHubClient::code_reviews(self)
    }

    fn pipelines(&self) -> Box<dyn Pipelines> {
        GitHubClient::pipelines(self)
    }

    fn releases(&self) -> Box<dyn Releases> {
        GitHubClient::releases(self)
    }

    fn default_base_url(&self) -> &str {
        self.provider.default_base_url()
    }

    fn auth_header_style(
        &self,
        auth_kind: vcs_provider_core::AuthKind,
    ) -> vcs_provider_core::AuthHeaderStyle {
        self.provider.auth_header_style(auth_kind)
    }
}

impl ProviderClient for GitHubClient {
    fn auth(self, credential: AuthCredential) -> Self {
        GitHubClient::auth(self, credential)
    }
}

impl GitHubProvider {
    pub fn repos(self) -> GitHubReposTransportBuilder {
        GitHubReposTransportBuilder { provider: self }
    }

    pub fn issues(self) -> GitHubIssuesTransportBuilder {
        GitHubIssuesTransportBuilder { provider: self }
    }

    pub fn code_reviews(self) -> GitHubCodeReviewsTransportBuilder {
        GitHubCodeReviewsTransportBuilder { provider: self }
    }

    pub fn pipelines(self) -> GitHubPipelinesTransportBuilder {
        GitHubPipelinesTransportBuilder { provider: self }
    }

    pub fn releases(self) -> GitHubReleasesTransportBuilder {
        GitHubReleasesTransportBuilder { provider: self }
    }

    pub fn client(self, transport: impl Transport + 'static) -> GitHubClient {
        GitHubClient::with_provider(self, transport)
    }

    pub fn transport(self, transport: impl Transport + 'static) -> GitHubClient {
        GitHubClient::with_provider(self, transport)
    }
}

impl ManagedClientProvider for GitHubProvider {
    type Client = GitHubClient;

    fn client(&self, transport: impl Transport + 'static) -> Self::Client {
        GitHubClient::with_provider(self.clone(), transport)
    }
}

impl GitHubReposTransportBuilder {
    pub fn response_body(self, body: impl Into<String>) -> Box<dyn Repos> {
        GitHubClient::with_provider(
            self.provider,
            vcs_provider_core::provider_response().body(body).get(),
        )
        .repos()
    }

    pub fn transport(self, transport: impl Transport + 'static) -> Box<dyn Repos> {
        GitHubClient::with_provider(self.provider, transport).repos()
    }
}

impl GitHubIssuesTransportBuilder {
    pub fn response_body(self, body: impl Into<String>) -> Box<dyn Issues> {
        GitHubClient::with_provider(
            self.provider,
            vcs_provider_core::provider_response().body(body).get(),
        )
        .issues()
    }

    pub fn transport(self, transport: impl Transport + 'static) -> Box<dyn Issues> {
        GitHubClient::with_provider(self.provider, transport).issues()
    }
}

impl GitHubCodeReviewsTransportBuilder {
    pub fn response_body(self, body: impl Into<String>) -> Box<dyn CodeReviews> {
        GitHubClient::with_provider(
            self.provider,
            vcs_provider_core::provider_response().body(body).get(),
        )
        .code_reviews()
    }

    pub fn transport(self, transport: impl Transport + 'static) -> Box<dyn CodeReviews> {
        GitHubClient::with_provider(self.provider, transport).code_reviews()
    }
}

impl GitHubPipelinesTransportBuilder {
    pub fn response_body(self, body: impl Into<String>) -> Box<dyn Pipelines> {
        GitHubClient::with_provider(
            self.provider,
            vcs_provider_core::provider_response().body(body).get(),
        )
        .pipelines()
    }

    pub fn transport(self, transport: impl Transport + 'static) -> Box<dyn Pipelines> {
        GitHubClient::with_provider(self.provider, transport).pipelines()
    }
}

impl GitHubReleasesTransportBuilder {
    pub fn response_body(self, body: impl Into<String>) -> Box<dyn Releases> {
        GitHubClient::with_provider(
            self.provider,
            vcs_provider_core::provider_response().body(body).get(),
        )
        .releases()
    }

    pub fn transport(self, transport: impl Transport + 'static) -> Box<dyn Releases> {
        GitHubClient::with_provider(self.provider, transport).releases()
    }
}

fn default_headers() -> Vec<RequestHeader> {
    vec![
        RequestHeader::make("accept", "application/vnd.github+json"),
        RequestHeader::make("x-github-api-version", "2022-11-28"),
    ]
}
