use std::sync::Arc;

use vcs_provider_core::{
    AuthCredential, CodeReviews, Issues, Pipelines, Provider, ProviderDescriptor, Releases, Repos,
    RequestHeader, Transport, TransportBackedCodeReviews, TransportBackedPipelines,
    TransportBackedRepos, TransportNotConfiguredIssues, TransportNotConfiguredReleases,
};

use crate::mappers::{
    BitbucketCodeReviewMapper, BitbucketPipelineMapper, BitbucketRepositoryMapper,
};
use crate::{BitbucketProvider, DEFAULT_BASE_URL, bitbucket};

#[derive(Clone)]
pub struct BitbucketClient {
    transport: Arc<dyn Transport>,
    headers: Vec<RequestHeader>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BitbucketPipelinesTransportBuilder;

#[derive(Clone, Copy, Debug, Default)]
pub struct BitbucketCodeReviewsTransportBuilder;

impl BitbucketClient {
    pub fn make(transport: impl Transport + 'static) -> Self {
        Self {
            transport: Arc::new(transport),
            headers: default_headers(),
        }
    }

    pub fn code_reviews(&self) -> Box<dyn CodeReviews> {
        Box::new(
            TransportBackedCodeReviews::make(
                bitbucket(),
                Arc::clone(&self.transport),
                BitbucketCodeReviewMapper,
            )
            .with_headers(self.headers.clone()),
        )
    }

    pub fn repos(&self) -> Box<dyn Repos> {
        Box::new(
            TransportBackedRepos::make(
                bitbucket(),
                Arc::clone(&self.transport),
                BitbucketRepositoryMapper,
            )
            .with_headers(self.headers.clone()),
        )
    }

    pub fn pipelines(&self) -> Box<dyn Pipelines> {
        Box::new(
            TransportBackedPipelines::make(
                bitbucket(),
                Arc::clone(&self.transport),
                BitbucketPipelineMapper,
            )
            .with_headers(self.headers.clone()),
        )
    }

    pub fn auth(mut self, credential: AuthCredential) -> Self {
        if let Some(header) = bitbucket().auth_header(&credential) {
            self.headers.push(RequestHeader::make(
                header.name().as_str(),
                header.value().as_str(),
            ));
        }

        self
    }
}

impl Provider for BitbucketClient {
    fn descriptor(&self) -> ProviderDescriptor {
        bitbucket().descriptor()
    }

    fn repos(&self) -> Box<dyn Repos> {
        BitbucketClient::repos(self)
    }

    fn issues(&self) -> Box<dyn Issues> {
        Box::<TransportNotConfiguredIssues>::default()
    }

    fn code_reviews(&self) -> Box<dyn CodeReviews> {
        BitbucketClient::code_reviews(self)
    }

    fn pipelines(&self) -> Box<dyn Pipelines> {
        BitbucketClient::pipelines(self)
    }

    fn releases(&self) -> Box<dyn Releases> {
        Box::<TransportNotConfiguredReleases>::default()
    }

    fn default_base_url(&self) -> &str {
        DEFAULT_BASE_URL
    }

    fn auth_header_style(
        &self,
        auth_kind: vcs_provider_core::AuthKind,
    ) -> vcs_provider_core::AuthHeaderStyle {
        bitbucket().auth_header_style(auth_kind)
    }
}

impl BitbucketProvider {
    pub fn code_reviews(self) -> BitbucketCodeReviewsTransportBuilder {
        BitbucketCodeReviewsTransportBuilder
    }

    pub fn pipelines(self) -> BitbucketPipelinesTransportBuilder {
        BitbucketPipelinesTransportBuilder
    }

    pub fn client(self, transport: impl Transport + 'static) -> BitbucketClient {
        BitbucketClient::make(transport)
    }

    pub fn transport(self, transport: impl Transport + 'static) -> BitbucketClient {
        BitbucketClient::make(transport)
    }

    pub fn body(self, body: impl Into<String>) -> BitbucketClient {
        BitbucketClient::make(vcs_provider_core::provider_response().body(body).get())
    }
}

impl BitbucketCodeReviewsTransportBuilder {
    pub fn response_body(self, body: impl Into<String>) -> Box<dyn CodeReviews> {
        BitbucketClient::make(vcs_provider_core::provider_response().body(body).get())
            .code_reviews()
    }

    pub fn transport(self, transport: impl Transport + 'static) -> Box<dyn CodeReviews> {
        BitbucketClient::make(transport).code_reviews()
    }
}

impl BitbucketPipelinesTransportBuilder {
    pub fn response_body(self, body: impl Into<String>) -> Box<dyn Pipelines> {
        BitbucketClient::make(vcs_provider_core::provider_response().body(body).get()).pipelines()
    }

    pub fn transport(self, transport: impl Transport + 'static) -> Box<dyn Pipelines> {
        BitbucketClient::make(transport).pipelines()
    }
}

fn default_headers() -> Vec<RequestHeader> {
    vec![RequestHeader::make("accept", "application/json")]
}
