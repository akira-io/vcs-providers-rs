use std::sync::Arc;

use vcs_provider_core::{
    CodeReviews, Issues, Pipelines, Provider, ProviderDescriptor, Releases, Repos, Transport,
    TransportBackedRepos, TransportNotConfiguredCodeReviews, TransportNotConfiguredIssues,
    TransportNotConfiguredPipelines, TransportNotConfiguredReleases,
};

use crate::mappers::BitbucketRepositoryMapper;
use crate::{BitbucketProvider, DEFAULT_BASE_URL, bitbucket};

#[derive(Clone)]
pub struct BitbucketClient {
    transport: Arc<dyn Transport>,
}

impl BitbucketClient {
    pub fn make(transport: impl Transport + 'static) -> Self {
        Self {
            transport: Arc::new(transport),
        }
    }

    pub fn repos(&self) -> Box<dyn Repos> {
        Box::new(TransportBackedRepos::make(
            bitbucket(),
            Arc::clone(&self.transport),
            BitbucketRepositoryMapper,
        ))
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
