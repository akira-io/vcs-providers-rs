use vcs_provider_core::{
    AuthHeaderStyle, AuthKind, CodeReviews, Issues, ManagedProvider, MissingCodeReviewId,
    MissingCodeReviewRepo, MissingOwnerName, MissingReleaseId, MissingReleaseRepo,
    MissingRepositoryName, Pipelines, Provider, ProviderDescriptor, ProviderId, Releases, Repos,
    TransportNotConfiguredCodeReviews, TransportNotConfiguredIssues,
    TransportNotConfiguredPipelines, TransportNotConfiguredReleases, TransportNotConfiguredRepos,
};

mod capabilities;
mod client;
mod code_reviews;
mod issues;
mod mappers;
mod pagination;
mod pipelines;
mod provider_collaboration;
mod provider_pipelines;
mod releases;
mod repos;
mod request_pagination;
mod response_fixture;

pub use client::GitLabClient;
pub use code_reviews::{GitLabCodeReview, GitLabCodeReviewCollection};
pub use issues::{GitLabIssue, GitLabIssueCollection};
pub use pipelines::{GitLabPipeline, GitLabPipelineCollection};
pub use releases::{GitLabRelease, GitLabReleaseCollection};
pub use repos::{GitLabRepo, GitLabRepoCollection};
pub use response_fixture::GitLabResponseBuilder;

use capabilities::gitlab_capabilities;

pub const PROVIDER_ID: &str = "gitlab";
pub const DISPLAY_NAME: &str = "GitLab";
pub const DEFAULT_BASE_URL: &str = "https://gitlab.com";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitLabProvider {
    base_url: String,
}

impl GitLabProvider {
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    pub fn api_base_url(&self) -> &str {
        &self.base_url
    }

    pub fn repo(
        &self,
    ) -> vcs_provider_core::ManagedRepoBuilder<Self, MissingOwnerName, MissingRepositoryName> {
        vcs_provider_core::vcs(self.clone()).repo()
    }

    pub fn issue(
        &self,
    ) -> vcs_provider_core::ManagedIssueBuilder<
        Self,
        vcs_provider_core::MissingIssueRepo,
        vcs_provider_core::MissingIssueId,
    > {
        vcs_provider_core::vcs(self.clone()).issue()
    }

    pub fn code_review(
        &self,
    ) -> vcs_provider_core::ManagedCodeReviewBuilder<Self, MissingCodeReviewRepo, MissingCodeReviewId>
    {
        vcs_provider_core::vcs(self.clone()).code_review()
    }

    pub fn release(
        &self,
    ) -> vcs_provider_core::ManagedReleaseBuilder<Self, MissingReleaseRepo, MissingReleaseId> {
        vcs_provider_core::vcs(self.clone()).release()
    }

    pub fn pagination(&self) -> vcs_provider_core::PaginationBuilder {
        vcs_provider_core::pagination()
    }
}

impl ManagedProvider for GitLabProvider {
    fn repo_url(&self, repo: &vcs_provider_core::Repo) -> vcs_provider_core::RequestUrl {
        GitLabRepo::make(self.api_base_url(), repo.clone()).url()
    }

    fn repo_branches_url(
        &self,
        repo: &vcs_provider_core::Repo,
        page: Option<&vcs_provider_core::PageRequest>,
    ) -> vcs_provider_core::RequestUrl {
        GitLabRepo::make(self.api_base_url(), repo.clone()).branches(page)
    }

    fn repo_commits_url(
        &self,
        repo: &vcs_provider_core::Repo,
        page: Option<&vcs_provider_core::PageRequest>,
    ) -> vcs_provider_core::RequestUrl {
        GitLabRepo::make(self.api_base_url(), repo.clone()).commits(page)
    }

    fn repo_list_url(
        &self,
        query: &vcs_provider_core::RepositoryListQuery,
    ) -> vcs_provider_core::RequestUrl {
        GitLabRepoCollection::make(self.api_base_url()).list(query)
    }

    fn repo_search_url(
        &self,
        query: &vcs_provider_core::RepositorySearchQuery,
    ) -> vcs_provider_core::RequestUrl {
        GitLabRepoCollection::make(self.api_base_url()).search(query)
    }

    fn repo_create_request(
        &self,
        draft: &vcs_provider_core::RepositoryDraft,
    ) -> vcs_provider_core::Request {
        GitLabRepoCollection::make(self.api_base_url()).create(draft)
    }

    fn repo_update_request(
        &self,
        patch: &vcs_provider_core::RepositoryPatch,
    ) -> vcs_provider_core::Request {
        GitLabRepo::make(self.api_base_url(), patch.repo().clone()).update(patch)
    }

    fn repo_delete_request(&self, repo: &vcs_provider_core::Repo) -> vcs_provider_core::Request {
        GitLabRepo::make(self.api_base_url(), repo.clone()).delete()
    }
}

impl Provider for GitLabProvider {
    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptor::make(
            ProviderId::make(PROVIDER_ID),
            DISPLAY_NAME,
            gitlab_capabilities(),
        )
    }

    fn repos(&self) -> Box<dyn Repos> {
        Box::<TransportNotConfiguredRepos>::default()
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
        self.api_base_url()
    }

    fn auth_header_style(&self, auth_kind: AuthKind) -> AuthHeaderStyle {
        match auth_kind {
            AuthKind::Anonymous => AuthHeaderStyle::None,
            AuthKind::PersonalAccessToken => AuthHeaderStyle::CustomHeader("private-token".into()),
            AuthKind::OAuth => AuthHeaderStyle::AuthorizationBearer,
            AuthKind::AppInstallation => AuthHeaderStyle::AuthorizationBearer,
            AuthKind::Jwt => AuthHeaderStyle::AuthorizationBearer,
        }
    }
}

pub fn gitlab() -> GitLabProvider {
    GitLabProvider::default()
}

impl Default for GitLabProvider {
    fn default() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.into(),
        }
    }
}
