use std::sync::Arc;

use vcs_provider_core::{
    AuthCredential, CodeReviews, Issues, ManagedClientProvider, Pipelines, Provider,
    ProviderClient, ProviderDescriptor, Releases, Repos, RequestHeader, Transport,
    TransportBackedCodeReviews, TransportBackedIssues, TransportBackedPipelines,
    TransportBackedReleases, TransportBackedRepos,
};

use crate::mappers::{
    GitLabCodeReviewMapper, GitLabIssueMapper, GitLabPipelineMapper, GitLabReleaseMapper,
    GitLabRepositoryMapper,
};
use crate::{GitLabProvider, gitlab};

#[derive(Clone)]
pub struct GitLabClient {
    provider: GitLabProvider,
    transport: Arc<dyn Transport>,
    headers: Vec<RequestHeader>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitLabPipelinesTransportBuilder {
    provider: GitLabProvider,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitLabReposTransportBuilder {
    provider: GitLabProvider,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitLabIssuesTransportBuilder {
    provider: GitLabProvider,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitLabCodeReviewsTransportBuilder {
    provider: GitLabProvider,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitLabReleasesTransportBuilder {
    provider: GitLabProvider,
}

impl GitLabClient {
    pub fn make(transport: impl Transport + 'static) -> Self {
        Self::with_provider(gitlab(), transport)
    }

    pub fn with_provider(provider: GitLabProvider, transport: impl Transport + 'static) -> Self {
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
                GitLabIssueMapper,
            )
            .with_headers(self.headers.clone()),
        )
    }

    pub fn code_reviews(&self) -> Box<dyn CodeReviews> {
        Box::new(
            TransportBackedCodeReviews::make(
                self.provider.clone(),
                Arc::clone(&self.transport),
                GitLabCodeReviewMapper,
            )
            .with_headers(self.headers.clone()),
        )
    }

    pub fn releases(&self) -> Box<dyn Releases> {
        Box::new(
            TransportBackedReleases::make(
                self.provider.clone(),
                Arc::clone(&self.transport),
                GitLabReleaseMapper,
            )
            .with_headers(self.headers.clone()),
        )
    }

    pub fn pipelines(&self) -> Box<dyn Pipelines> {
        Box::new(
            TransportBackedPipelines::make(
                self.provider.clone(),
                Arc::clone(&self.transport),
                GitLabPipelineMapper,
            )
            .with_headers(self.headers.clone()),
        )
    }

    pub fn repos(&self) -> Box<dyn Repos> {
        Box::new(
            TransportBackedRepos::make(
                self.provider.clone(),
                Arc::clone(&self.transport),
                GitLabRepositoryMapper,
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

impl Provider for GitLabClient {
    fn descriptor(&self) -> ProviderDescriptor {
        self.provider.descriptor()
    }

    fn repos(&self) -> Box<dyn Repos> {
        GitLabClient::repos(self)
    }

    fn issues(&self) -> Box<dyn Issues> {
        GitLabClient::issues(self)
    }

    fn code_reviews(&self) -> Box<dyn CodeReviews> {
        GitLabClient::code_reviews(self)
    }

    fn pipelines(&self) -> Box<dyn Pipelines> {
        GitLabClient::pipelines(self)
    }

    fn releases(&self) -> Box<dyn Releases> {
        GitLabClient::releases(self)
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

impl ProviderClient for GitLabClient {
    fn auth(self, credential: AuthCredential) -> Self {
        GitLabClient::auth(self, credential)
    }
}

impl GitLabProvider {
    pub fn repos(self) -> GitLabReposTransportBuilder {
        GitLabReposTransportBuilder { provider: self }
    }

    pub fn issues(self) -> GitLabIssuesTransportBuilder {
        GitLabIssuesTransportBuilder { provider: self }
    }

    pub fn code_reviews(self) -> GitLabCodeReviewsTransportBuilder {
        GitLabCodeReviewsTransportBuilder { provider: self }
    }

    pub fn pipelines(self) -> GitLabPipelinesTransportBuilder {
        GitLabPipelinesTransportBuilder { provider: self }
    }

    pub fn releases(self) -> GitLabReleasesTransportBuilder {
        GitLabReleasesTransportBuilder { provider: self }
    }

    pub fn client(self, transport: impl Transport + 'static) -> GitLabClient {
        GitLabClient::with_provider(self, transport)
    }

    pub fn transport(self, transport: impl Transport + 'static) -> GitLabClient {
        GitLabClient::with_provider(self, transport)
    }
}

impl ManagedClientProvider for GitLabProvider {
    type Client = GitLabClient;

    fn client(&self, transport: impl Transport + 'static) -> Self::Client {
        GitLabClient::with_provider(self.clone(), transport)
    }
}

impl GitLabReposTransportBuilder {
    pub fn response_body(self, body: impl Into<String>) -> Box<dyn Repos> {
        GitLabClient::with_provider(
            self.provider,
            vcs_provider_core::provider_response().body(body).get(),
        )
        .repos()
    }

    pub fn transport(self, transport: impl Transport + 'static) -> Box<dyn Repos> {
        GitLabClient::with_provider(self.provider, transport).repos()
    }
}

impl GitLabIssuesTransportBuilder {
    pub fn response_body(self, body: impl Into<String>) -> Box<dyn Issues> {
        GitLabClient::with_provider(
            self.provider,
            vcs_provider_core::provider_response().body(body).get(),
        )
        .issues()
    }

    pub fn transport(self, transport: impl Transport + 'static) -> Box<dyn Issues> {
        GitLabClient::with_provider(self.provider, transport).issues()
    }
}

impl GitLabCodeReviewsTransportBuilder {
    pub fn response_body(self, body: impl Into<String>) -> Box<dyn CodeReviews> {
        GitLabClient::with_provider(
            self.provider,
            vcs_provider_core::provider_response().body(body).get(),
        )
        .code_reviews()
    }

    pub fn transport(self, transport: impl Transport + 'static) -> Box<dyn CodeReviews> {
        GitLabClient::with_provider(self.provider, transport).code_reviews()
    }
}

impl GitLabPipelinesTransportBuilder {
    pub fn response_body(self, body: impl Into<String>) -> Box<dyn Pipelines> {
        GitLabClient::with_provider(
            self.provider,
            vcs_provider_core::provider_response().body(body).get(),
        )
        .pipelines()
    }

    pub fn transport(self, transport: impl Transport + 'static) -> Box<dyn Pipelines> {
        GitLabClient::with_provider(self.provider, transport).pipelines()
    }
}

impl GitLabReleasesTransportBuilder {
    pub fn response_body(self, body: impl Into<String>) -> Box<dyn Releases> {
        GitLabClient::with_provider(
            self.provider,
            vcs_provider_core::provider_response().body(body).get(),
        )
        .releases()
    }

    pub fn transport(self, transport: impl Transport + 'static) -> Box<dyn Releases> {
        GitLabClient::with_provider(self.provider, transport).releases()
    }
}

fn default_headers() -> Vec<RequestHeader> {
    vec![RequestHeader::make("accept", "application/json")]
}
