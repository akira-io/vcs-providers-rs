use std::sync::Arc;

use vcs_provider_core::{
    AuthCredential, CodeReviews, Issues, Pipelines, Provider, ProviderDescriptor, Releases, Repos,
    RequestHeader, Transport, TransportBackedCodeReviews, TransportBackedIssues,
    TransportBackedReleases, TransportBackedRepos, TransportNotConfiguredPipelines,
};

use crate::mappers::{
    GitLabCodeReviewMapper, GitLabIssueMapper, GitLabReleaseMapper, GitLabRepositoryMapper,
};
use crate::{DEFAULT_BASE_URL, GitLabProvider, gitlab};

#[derive(Clone)]
pub struct GitLabClient {
    transport: Arc<dyn Transport>,
    headers: Vec<RequestHeader>,
}

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
        Box::<TransportNotConfiguredPipelines>::default()
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

impl GitLabProvider {
    pub fn client(self, transport: impl Transport + 'static) -> GitLabClient {
        GitLabClient::make(transport)
    }

    pub fn transport(self, transport: impl Transport + 'static) -> GitLabClient {
        GitLabClient::make(transport)
    }

    pub fn body(self, body: impl Into<String>) -> GitLabClient {
        GitLabClient::make(vcs_provider_core::provider_response().body(body).get())
    }
}

fn default_headers() -> Vec<RequestHeader> {
    vec![RequestHeader::make("accept", "application/json")]
}
