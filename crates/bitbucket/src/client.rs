use std::sync::Arc;

use vcs_provider_core::{
    AuthCredential, CodeReviews, Issues, Pipelines, Provider, ProviderDescriptor, Releases, Repos,
    RequestHeader, Transport, TransportBackedRepos, TransportNotConfiguredCodeReviews,
    TransportNotConfiguredIssues, TransportNotConfiguredPipelines, TransportNotConfiguredReleases,
};

use crate::mappers::BitbucketRepositoryMapper;
use crate::{BitbucketProvider, DEFAULT_BASE_URL, bitbucket};

#[derive(Clone)]
pub struct BitbucketClient {
    transport: Arc<dyn Transport>,
    headers: Vec<RequestHeader>,
}

impl BitbucketClient {
    pub fn make(transport: impl Transport + 'static) -> Self {
        Self {
            transport: Arc::new(transport),
            headers: default_headers(),
        }
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
        Box::<TransportNotConfiguredCodeReviews>::default()
    }

    fn pipelines(&self) -> Box<dyn Pipelines> {
        Box::<TransportNotConfiguredPipelines>::default()
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
    pub fn client(self, transport: impl Transport + 'static) -> BitbucketClient {
        BitbucketClient::make(transport)
    }

    pub fn transport(self, transport: impl Transport + 'static) -> BitbucketClient {
        BitbucketClient::make(transport)
    }
}

fn default_headers() -> Vec<RequestHeader> {
    vec![RequestHeader::make("accept", "application/json")]
}
