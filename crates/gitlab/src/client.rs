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
use crate::{DEFAULT_BASE_URL, GitLabProvider, gitlab};

#[derive(Clone)]
pub struct GitLabClient {
    transport: Arc<dyn Transport>,
    headers: Vec<RequestHeader>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct GitLabPipelinesTransportBuilder;

#[derive(Clone, Copy, Debug, Default)]
pub struct GitLabReposTransportBuilder;

#[derive(Clone, Copy, Debug, Default)]
pub struct GitLabIssuesTransportBuilder;

#[derive(Clone, Copy, Debug, Default)]
pub struct GitLabCodeReviewsTransportBuilder;

#[derive(Clone, Copy, Debug, Default)]
pub struct GitLabReleasesTransportBuilder;

impl GitLabClient {
    pub fn make(transport: impl Transport + 'static) -> Self {
        Self {
            transport: Arc::new(transport),
            headers: default_headers(),
        }
    }

    pub fn issues(&self) -> Box<dyn Issues> {
        Box::new(
            TransportBackedIssues::make(gitlab(), Arc::clone(&self.transport), GitLabIssueMapper)
                .with_headers(self.headers.clone()),
        )
    }

    pub fn code_reviews(&self) -> Box<dyn CodeReviews> {
        Box::new(
            TransportBackedCodeReviews::make(
                gitlab(),
                Arc::clone(&self.transport),
                GitLabCodeReviewMapper,
            )
            .with_headers(self.headers.clone()),
        )
    }

    pub fn releases(&self) -> Box<dyn Releases> {
        Box::new(
            TransportBackedReleases::make(
                gitlab(),
                Arc::clone(&self.transport),
                GitLabReleaseMapper,
            )
            .with_headers(self.headers.clone()),
        )
    }

    pub fn pipelines(&self) -> Box<dyn Pipelines> {
        Box::new(
            TransportBackedPipelines::make(
                gitlab(),
                Arc::clone(&self.transport),
                GitLabPipelineMapper,
            )
            .with_headers(self.headers.clone()),
        )
    }

    pub fn repos(&self) -> Box<dyn Repos> {
        Box::new(
            TransportBackedRepos::make(
                gitlab(),
                Arc::clone(&self.transport),
                GitLabRepositoryMapper,
            )
            .with_headers(self.headers.clone()),
        )
    }

    pub fn auth(mut self, credential: AuthCredential) -> Self {
        if let Some(header) = gitlab().auth_header(&credential) {
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
        gitlab().descriptor()
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
        DEFAULT_BASE_URL
    }

    fn auth_header_style(
        &self,
        auth_kind: vcs_provider_core::AuthKind,
    ) -> vcs_provider_core::AuthHeaderStyle {
        gitlab().auth_header_style(auth_kind)
    }
}

impl ProviderClient for GitLabClient {
    fn auth(self, credential: AuthCredential) -> Self {
        GitLabClient::auth(self, credential)
    }
}

impl GitLabProvider {
    pub fn repos(self) -> GitLabReposTransportBuilder {
        GitLabReposTransportBuilder
    }

    pub fn issues(self) -> GitLabIssuesTransportBuilder {
        GitLabIssuesTransportBuilder
    }

    pub fn code_reviews(self) -> GitLabCodeReviewsTransportBuilder {
        GitLabCodeReviewsTransportBuilder
    }

    pub fn pipelines(self) -> GitLabPipelinesTransportBuilder {
        GitLabPipelinesTransportBuilder
    }

    pub fn releases(self) -> GitLabReleasesTransportBuilder {
        GitLabReleasesTransportBuilder
    }

    pub fn client(self, transport: impl Transport + 'static) -> GitLabClient {
        GitLabClient::make(transport)
    }

    pub fn transport(self, transport: impl Transport + 'static) -> GitLabClient {
        GitLabClient::make(transport)
    }
}

impl ManagedClientProvider for GitLabProvider {
    type Client = GitLabClient;

    fn client(&self, transport: impl Transport + 'static) -> Self::Client {
        GitLabClient::make(transport)
    }
}

impl GitLabReposTransportBuilder {
    pub fn response_body(self, body: impl Into<String>) -> Box<dyn Repos> {
        GitLabClient::make(vcs_provider_core::provider_response().body(body).get()).repos()
    }

    pub fn transport(self, transport: impl Transport + 'static) -> Box<dyn Repos> {
        GitLabClient::make(transport).repos()
    }
}

impl GitLabIssuesTransportBuilder {
    pub fn response_body(self, body: impl Into<String>) -> Box<dyn Issues> {
        GitLabClient::make(vcs_provider_core::provider_response().body(body).get()).issues()
    }

    pub fn transport(self, transport: impl Transport + 'static) -> Box<dyn Issues> {
        GitLabClient::make(transport).issues()
    }
}

impl GitLabCodeReviewsTransportBuilder {
    pub fn response_body(self, body: impl Into<String>) -> Box<dyn CodeReviews> {
        GitLabClient::make(vcs_provider_core::provider_response().body(body).get()).code_reviews()
    }

    pub fn transport(self, transport: impl Transport + 'static) -> Box<dyn CodeReviews> {
        GitLabClient::make(transport).code_reviews()
    }
}

impl GitLabPipelinesTransportBuilder {
    pub fn response_body(self, body: impl Into<String>) -> Box<dyn Pipelines> {
        GitLabClient::make(vcs_provider_core::provider_response().body(body).get()).pipelines()
    }

    pub fn transport(self, transport: impl Transport + 'static) -> Box<dyn Pipelines> {
        GitLabClient::make(transport).pipelines()
    }
}

impl GitLabReleasesTransportBuilder {
    pub fn response_body(self, body: impl Into<String>) -> Box<dyn Releases> {
        GitLabClient::make(vcs_provider_core::provider_response().body(body).get()).releases()
    }

    pub fn transport(self, transport: impl Transport + 'static) -> Box<dyn Releases> {
        GitLabClient::make(transport).releases()
    }
}

fn default_headers() -> Vec<RequestHeader> {
    vec![RequestHeader::make("accept", "application/json")]
}
