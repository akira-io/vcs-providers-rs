use std::sync::Arc;

use vcs_provider_core::{
    CodeReviews, Issues, Pipelines, Provider, ProviderDescriptor, Releases, Repos, Transport,
    TransportBackedRepos, TransportNotConfiguredCodeReviews, TransportNotConfiguredIssues,
    TransportNotConfiguredPipelines, TransportNotConfiguredReleases,
};

use crate::mappers::GitHubRepositoryMapper;
use crate::{DEFAULT_BASE_URL, GitHubProvider, github};

#[derive(Clone)]
pub struct GitHubClient {
    transport: Arc<dyn Transport>,
}

impl GitHubClient {
    pub fn make(transport: impl Transport + 'static) -> Self {
        Self {
            transport: Arc::new(transport),
        }
    }

    pub fn repos(&self) -> Box<dyn Repos> {
        Box::new(TransportBackedRepos::make(
            github(),
            Arc::clone(&self.transport),
            GitHubRepositoryMapper,
        ))
    }
}

impl Provider for GitHubClient {
    fn descriptor(&self) -> ProviderDescriptor {
        github().descriptor()
    }

    fn repos(&self) -> Box<dyn Repos> {
        GitHubClient::repos(self)
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
        github().auth_header_style(auth_kind)
    }
}

impl GitHubProvider {
    pub fn client(self, transport: impl Transport + 'static) -> GitHubClient {
        GitHubClient::make(transport)
    }

    pub fn transport(self, transport: impl Transport + 'static) -> GitHubClient {
        GitHubClient::make(transport)
    }
}
