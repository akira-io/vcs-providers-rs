use vcs_provider_core::{
    AuthHeaderStyle, AuthKind, CodeReviews, Issues, ManagedCodeReviewProvider,
    ManagedIssueProvider, ManagedProvider, MissingCodeReviewId, MissingCodeReviewRepo,
    MissingOwnerName, MissingReleaseId, MissingReleaseRepo, MissingRepositoryName, Pipelines,
    Provider, ProviderDescriptor, ProviderId, Releases, Repos, TransportNotConfiguredCodeReviews,
    TransportNotConfiguredIssues, TransportNotConfiguredPipelines, TransportNotConfiguredReleases,
    TransportNotConfiguredRepos,
};

mod capabilities;
mod client;
mod code_reviews;
mod issues;
mod mappers;
mod pagination;
mod pipelines;
mod provider_pipelines;
mod releases;
mod repos;
mod request_pagination;
mod response_fixture;

pub use client::GitHubClient;
pub use code_reviews::{GitHubCodeReview, GitHubCodeReviewCollection};
pub use issues::{GitHubIssue, GitHubIssueCollection};
pub use pipelines::{GitHubPipeline, GitHubPipelineCollection};
pub use releases::{GitHubRelease, GitHubReleaseCollection};
pub use repos::{GitHubRepo, GitHubRepoCollection};
pub use response_fixture::GitHubResponseBuilder;

use capabilities::github_capabilities;

pub const PROVIDER_ID: &str = "github";
pub const DISPLAY_NAME: &str = "GitHub";
pub const DEFAULT_BASE_URL: &str = "https://api.github.com";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitHubProvider {
    base_url: String,
}

impl GitHubProvider {
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

impl ManagedProvider for GitHubProvider {
    fn repo_url(&self, repo: &vcs_provider_core::Repo) -> vcs_provider_core::RequestUrl {
        GitHubRepo::make(self.api_base_url(), repo.clone()).url()
    }

    fn repo_branches_url(
        &self,
        repo: &vcs_provider_core::Repo,
        page: Option<&vcs_provider_core::PageRequest>,
    ) -> vcs_provider_core::RequestUrl {
        GitHubRepo::make(self.api_base_url(), repo.clone()).branches(page)
    }

    fn repo_commits_url(
        &self,
        repo: &vcs_provider_core::Repo,
        page: Option<&vcs_provider_core::PageRequest>,
    ) -> vcs_provider_core::RequestUrl {
        GitHubRepo::make(self.api_base_url(), repo.clone()).commits(page)
    }

    fn repo_list_url(
        &self,
        query: &vcs_provider_core::RepositoryListQuery,
    ) -> vcs_provider_core::RequestUrl {
        GitHubRepoCollection::make(self.api_base_url()).list(query)
    }

    fn repo_search_url(
        &self,
        query: &vcs_provider_core::RepositorySearchQuery,
    ) -> vcs_provider_core::RequestUrl {
        GitHubRepoCollection::make(self.api_base_url()).search(query)
    }

    fn repo_create_request(
        &self,
        draft: &vcs_provider_core::RepositoryDraft,
    ) -> vcs_provider_core::Request {
        GitHubRepoCollection::make(self.api_base_url()).create(draft)
    }

    fn repo_update_request(
        &self,
        patch: &vcs_provider_core::RepositoryPatch,
    ) -> vcs_provider_core::Request {
        GitHubRepo::make(self.api_base_url(), patch.repo().clone()).update(patch)
    }

    fn repo_delete_request(&self, repo: &vcs_provider_core::Repo) -> vcs_provider_core::Request {
        GitHubRepo::make(self.api_base_url(), repo.clone()).delete()
    }
}

impl ManagedIssueProvider for GitHubProvider {
    fn issue_url(&self, issue: &vcs_provider_core::Issue) -> vcs_provider_core::RequestUrl {
        GitHubIssue::make(self.api_base_url(), issue.clone()).url()
    }

    fn issue_list_url(
        &self,
        query: &vcs_provider_core::IssueListQuery,
    ) -> vcs_provider_core::RequestUrl {
        GitHubIssueCollection::make(self.api_base_url()).list(query)
    }

    fn issue_create_request(
        &self,
        draft: &vcs_provider_core::IssueDraft,
    ) -> vcs_provider_core::Request {
        GitHubIssueCollection::make(self.api_base_url()).create(draft)
    }

    fn issue_update_request(
        &self,
        patch: &vcs_provider_core::IssuePatch,
    ) -> vcs_provider_core::Request {
        GitHubIssue::make(self.api_base_url(), patch.issue().clone()).update(patch)
    }
}

impl ManagedCodeReviewProvider for GitHubProvider {
    fn code_review_url(
        &self,
        code_review: &vcs_provider_core::CodeReview,
    ) -> vcs_provider_core::RequestUrl {
        GitHubCodeReview::make(self.api_base_url(), code_review.clone()).url()
    }

    fn code_review_list_url(
        &self,
        query: &vcs_provider_core::CodeReviewListQuery,
    ) -> vcs_provider_core::RequestUrl {
        GitHubCodeReviewCollection::make(self.api_base_url()).list(query)
    }

    fn code_review_create_request(
        &self,
        draft: &vcs_provider_core::CodeReviewDraft,
    ) -> vcs_provider_core::Request {
        GitHubCodeReviewCollection::make(self.api_base_url()).create(draft)
    }

    fn code_review_update_request(
        &self,
        patch: &vcs_provider_core::CodeReviewPatch,
    ) -> vcs_provider_core::Request {
        GitHubCodeReview::make(self.api_base_url(), patch.code_review().clone()).update(patch)
    }

    fn code_review_merge_request(
        &self,
        code_review: &vcs_provider_core::CodeReview,
    ) -> vcs_provider_core::Request {
        GitHubCodeReview::make(self.api_base_url(), code_review.clone()).merge()
    }

    fn code_review_close_request(
        &self,
        code_review: &vcs_provider_core::CodeReview,
    ) -> vcs_provider_core::Request {
        GitHubCodeReview::make(self.api_base_url(), code_review.clone()).close()
    }
}

impl vcs_provider_core::ManagedReleaseProvider for GitHubProvider {
    fn release_url(&self, release: &vcs_provider_core::Release) -> vcs_provider_core::RequestUrl {
        GitHubRelease::make(self.api_base_url(), release.clone()).url()
    }

    fn release_list_url(
        &self,
        query: &vcs_provider_core::ReleaseListQuery,
    ) -> vcs_provider_core::RequestUrl {
        GitHubReleaseCollection::make(self.api_base_url()).list(query)
    }

    fn release_create_request(
        &self,
        draft: &vcs_provider_core::ReleaseDraft,
    ) -> vcs_provider_core::Request {
        GitHubReleaseCollection::make(self.api_base_url()).create(draft)
    }

    fn release_update_request(
        &self,
        patch: &vcs_provider_core::ReleasePatch,
    ) -> vcs_provider_core::Request {
        GitHubRelease::make(self.api_base_url(), patch.release().clone()).update(patch)
    }

    fn release_delete_request(
        &self,
        release: &vcs_provider_core::Release,
    ) -> vcs_provider_core::Request {
        GitHubRelease::make(self.api_base_url(), release.clone()).delete()
    }
}

impl Provider for GitHubProvider {
    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptor::make(
            ProviderId::make(PROVIDER_ID),
            DISPLAY_NAME,
            github_capabilities(),
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
            AuthKind::PersonalAccessToken => AuthHeaderStyle::AuthorizationBearer,
            AuthKind::OAuth => AuthHeaderStyle::AuthorizationBearer,
            AuthKind::AppInstallation => AuthHeaderStyle::AuthorizationBearer,
            AuthKind::Jwt => AuthHeaderStyle::AuthorizationBearer,
        }
    }
}

pub fn github() -> GitHubProvider {
    GitHubProvider::default()
}

impl Default for GitHubProvider {
    fn default() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.into(),
        }
    }
}
