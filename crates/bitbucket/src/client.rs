use std::sync::Arc;

use vcs_provider_core::{
    AuthCredential, CodeReviews, Issues, ManagedClientProvider, Pipelines, Provider,
    ProviderClient, ProviderDescriptor, Releases, Repos, RequestHeader, Transport,
    TransportBackedCodeReviews, TransportBackedPipelines, TransportBackedRepos, UnsupportedIssues,
    UnsupportedReleases,
};

use crate::mappers::{
    BitbucketCodeReviewMapper, BitbucketPipelineMapper, BitbucketRepositoryMapper,
};
use crate::{BitbucketProvider, bitbucket};

#[derive(Clone)]
pub struct BitbucketClient {
    provider: BitbucketProvider,
    transport: Arc<dyn Transport>,
    headers: Vec<RequestHeader>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitbucketPipelinesTransportBuilder {
    provider: BitbucketProvider,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitbucketReposTransportBuilder {
    provider: BitbucketProvider,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitbucketCodeReviewsTransportBuilder {
    provider: BitbucketProvider,
}

impl BitbucketClient {
    pub fn make(transport: impl Transport + 'static) -> Self {
        Self::with_provider(bitbucket(), transport)
    }

    pub fn with_provider(provider: BitbucketProvider, transport: impl Transport + 'static) -> Self {
        Self {
            provider,
            transport: Arc::new(transport),
            headers: default_headers(),
        }
    }

    pub fn code_reviews(&self) -> Box<dyn CodeReviews> {
        Box::new(
            TransportBackedCodeReviews::make(
                self.provider.clone(),
                Arc::clone(&self.transport),
                BitbucketCodeReviewMapper,
            )
            .with_headers(self.headers.clone()),
        )
    }

    pub fn repos(&self) -> Box<dyn Repos> {
        Box::new(
            TransportBackedRepos::make(
                self.provider.clone(),
                Arc::clone(&self.transport),
                BitbucketRepositoryMapper,
            )
            .with_headers(self.headers.clone()),
        )
    }

    pub fn pipelines(&self) -> Box<dyn Pipelines> {
        Box::new(
            TransportBackedPipelines::make(
                self.provider.clone(),
                Arc::clone(&self.transport),
                BitbucketPipelineMapper,
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

impl Provider for BitbucketClient {
    fn descriptor(&self) -> ProviderDescriptor {
        self.provider.descriptor()
    }

    fn repos(&self) -> Box<dyn Repos> {
        BitbucketClient::repos(self)
    }

    fn issues(&self) -> Box<dyn Issues> {
        Box::<UnsupportedIssues>::default()
    }

    fn code_reviews(&self) -> Box<dyn CodeReviews> {
        BitbucketClient::code_reviews(self)
    }

    fn pipelines(&self) -> Box<dyn Pipelines> {
        BitbucketClient::pipelines(self)
    }

    fn releases(&self) -> Box<dyn Releases> {
        Box::<UnsupportedReleases>::default()
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

impl ProviderClient for BitbucketClient {
    fn auth(self, credential: AuthCredential) -> Self {
        BitbucketClient::auth(self, credential)
    }
}

impl BitbucketProvider {
    pub fn repos(self) -> BitbucketReposTransportBuilder {
        BitbucketReposTransportBuilder { provider: self }
    }

    pub fn code_reviews(self) -> BitbucketCodeReviewsTransportBuilder {
        BitbucketCodeReviewsTransportBuilder { provider: self }
    }

    pub fn pipelines(self) -> BitbucketPipelinesTransportBuilder {
        BitbucketPipelinesTransportBuilder { provider: self }
    }

    pub fn client(self, transport: impl Transport + 'static) -> BitbucketClient {
        BitbucketClient::with_provider(self, transport)
    }

    pub fn transport(self, transport: impl Transport + 'static) -> BitbucketClient {
        BitbucketClient::with_provider(self, transport)
    }
}

impl ManagedClientProvider for BitbucketProvider {
    type Client = BitbucketClient;

    fn client(&self, transport: impl Transport + 'static) -> Self::Client {
        BitbucketClient::with_provider(self.clone(), transport)
    }
}

impl BitbucketReposTransportBuilder {
    pub fn response_body(self, body: impl Into<String>) -> Box<dyn Repos> {
        BitbucketClient::with_provider(
            self.provider,
            vcs_provider_core::provider_response().body(body).get(),
        )
        .repos()
    }

    pub fn transport(self, transport: impl Transport + 'static) -> Box<dyn Repos> {
        BitbucketClient::with_provider(self.provider, transport).repos()
    }
}

impl BitbucketCodeReviewsTransportBuilder {
    pub fn response_body(self, body: impl Into<String>) -> Box<dyn CodeReviews> {
        BitbucketClient::with_provider(
            self.provider,
            vcs_provider_core::provider_response().body(body).get(),
        )
        .code_reviews()
    }

    pub fn transport(self, transport: impl Transport + 'static) -> Box<dyn CodeReviews> {
        BitbucketClient::with_provider(self.provider, transport).code_reviews()
    }
}

impl BitbucketPipelinesTransportBuilder {
    pub fn response_body(self, body: impl Into<String>) -> Box<dyn Pipelines> {
        BitbucketClient::with_provider(
            self.provider,
            vcs_provider_core::provider_response().body(body).get(),
        )
        .pipelines()
    }

    pub fn transport(self, transport: impl Transport + 'static) -> Box<dyn Pipelines> {
        BitbucketClient::with_provider(self.provider, transport).pipelines()
    }
}

fn default_headers() -> Vec<RequestHeader> {
    vec![RequestHeader::make("accept", "application/json")]
}
