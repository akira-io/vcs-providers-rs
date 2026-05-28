use git_cognition_core::{
    AuthHeaderStyle, AuthKind, BranchDraft, CodeReviews, Issues, ManagedAuthProvider,
    ManagedCodeReviewProvider, ManagedIssueProvider, ManagedProvider, Pipelines, Provider,
    ProviderDescriptor, ProviderId, Releases, Repos, TransportNotConfiguredAuthentication,
    TransportNotConfiguredCodeReviews, TransportNotConfiguredIssues,
    TransportNotConfiguredOrganizations, TransportNotConfiguredPipelines,
    TransportNotConfiguredReleases, TransportNotConfiguredRepos,
};

mod capabilities;
mod client;
mod code_reviews;
mod issues;
mod mappers;
mod pagination;
mod pipelines;
mod provider_fluent;
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

impl ManagedProvider for GitHubProvider {
    fn repo_url(&self, repo: &git_cognition_core::Repo) -> git_cognition_core::RequestUrl {
        GitHubRepo::make(self.api_base_url(), repo.clone()).url()
    }

    fn repo_branches_url(
        &self,
        repo: &git_cognition_core::Repo,
        page: Option<&git_cognition_core::PageRequest>,
    ) -> git_cognition_core::RequestUrl {
        GitHubRepo::make(self.api_base_url(), repo.clone()).branches(page)
    }

    fn repo_commits_url(
        &self,
        repo: &git_cognition_core::Repo,
        page: Option<&git_cognition_core::PageRequest>,
    ) -> git_cognition_core::RequestUrl {
        GitHubRepo::make(self.api_base_url(), repo.clone()).commits(page)
    }

    fn repo_list_url(
        &self,
        query: &git_cognition_core::RepositoryListQuery,
    ) -> git_cognition_core::RequestUrl {
        GitHubRepoCollection::make(self.api_base_url()).list(query)
    }

    fn repo_search_url(
        &self,
        query: &git_cognition_core::RepositorySearchQuery,
    ) -> git_cognition_core::RequestUrl {
        GitHubRepoCollection::make(self.api_base_url()).search(query)
    }

    fn repo_create_request(
        &self,
        draft: &git_cognition_core::RepositoryDraft,
    ) -> git_cognition_core::Request {
        GitHubRepoCollection::make(self.api_base_url()).create(draft)
    }

    fn repo_update_request(
        &self,
        patch: &git_cognition_core::RepositoryPatch,
    ) -> git_cognition_core::Request {
        GitHubRepo::make(self.api_base_url(), patch.repo().clone()).update(patch)
    }

    fn repo_delete_request(&self, repo: &git_cognition_core::Repo) -> git_cognition_core::Request {
        GitHubRepo::make(self.api_base_url(), repo.clone()).delete()
    }

    fn repo_branch_create_request(
        &self,
        draft: &BranchDraft,
    ) -> git_cognition_core::CognitionResult<git_cognition_core::Request> {
        Ok(GitHubRepo::make(self.api_base_url(), draft.repo().clone()).create_branch(draft))
    }

    fn repo_branch_delete_request(
        &self,
        repo: &git_cognition_core::Repo,
        branch_name: &str,
    ) -> git_cognition_core::CognitionResult<git_cognition_core::Request> {
        Ok(GitHubRepo::make(self.api_base_url(), repo.clone()).delete_branch(branch_name))
    }
}

impl ManagedAuthProvider for GitHubProvider {
    fn auth_validate_url(&self) -> git_cognition_core::RequestUrl {
        git_cognition_core::url(self.api_base_url())
            .path_segments(["user"])
            .build()
    }
}

impl git_cognition_core::ManagedOrganizationProvider for GitHubProvider {
    fn organization_list_url(
        &self,
        query: Option<&git_cognition_core::OrganizationListQuery>,
    ) -> git_cognition_core::RequestUrl {
        let url = git_cognition_core::url(self.api_base_url()).path_segments(["user", "orgs"]);

        match query.and_then(git_cognition_core::OrganizationListQuery::page) {
            Some(page) => crate::request_pagination::apply_page(url, Some(page)).build(),
            None => url.build(),
        }
    }
}

impl ManagedIssueProvider for GitHubProvider {
    fn issue_url(&self, issue: &git_cognition_core::Issue) -> git_cognition_core::RequestUrl {
        GitHubIssue::make(self.api_base_url(), issue.clone()).url()
    }

    fn issue_list_url(
        &self,
        query: &git_cognition_core::IssueListQuery,
    ) -> git_cognition_core::RequestUrl {
        GitHubIssueCollection::make(self.api_base_url()).list(query)
    }

    fn issue_create_request(
        &self,
        draft: &git_cognition_core::IssueDraft,
    ) -> git_cognition_core::Request {
        GitHubIssueCollection::make(self.api_base_url()).create(draft)
    }

    fn issue_update_request(
        &self,
        patch: &git_cognition_core::IssuePatch,
    ) -> git_cognition_core::Request {
        GitHubIssue::make(self.api_base_url(), patch.issue().clone()).update(patch)
    }
}

impl ManagedCodeReviewProvider for GitHubProvider {
    fn code_review_url(
        &self,
        code_review: &git_cognition_core::CodeReview,
    ) -> git_cognition_core::RequestUrl {
        GitHubCodeReview::make(self.api_base_url(), code_review.clone()).url()
    }

    fn code_review_list_url(
        &self,
        query: &git_cognition_core::CodeReviewListQuery,
    ) -> git_cognition_core::RequestUrl {
        GitHubCodeReviewCollection::make(self.api_base_url()).list(query)
    }

    fn code_review_create_request(
        &self,
        draft: &git_cognition_core::CodeReviewDraft,
    ) -> git_cognition_core::Request {
        GitHubCodeReviewCollection::make(self.api_base_url()).create(draft)
    }

    fn code_review_update_request(
        &self,
        patch: &git_cognition_core::CodeReviewPatch,
    ) -> git_cognition_core::Request {
        GitHubCodeReview::make(self.api_base_url(), patch.code_review().clone()).update(patch)
    }

    fn code_review_merge_request(
        &self,
        code_review: &git_cognition_core::CodeReview,
    ) -> git_cognition_core::Request {
        GitHubCodeReview::make(self.api_base_url(), code_review.clone()).merge()
    }

    fn code_review_close_request(
        &self,
        code_review: &git_cognition_core::CodeReview,
    ) -> git_cognition_core::Request {
        GitHubCodeReview::make(self.api_base_url(), code_review.clone()).close()
    }
}

impl git_cognition_core::ManagedReleaseProvider for GitHubProvider {
    fn release_url(&self, release: &git_cognition_core::Release) -> git_cognition_core::RequestUrl {
        GitHubRelease::make(self.api_base_url(), release.clone()).url()
    }

    fn release_list_url(
        &self,
        query: &git_cognition_core::ReleaseListQuery,
    ) -> git_cognition_core::RequestUrl {
        GitHubReleaseCollection::make(self.api_base_url()).list(query)
    }

    fn release_create_request(
        &self,
        draft: &git_cognition_core::ReleaseDraft,
    ) -> git_cognition_core::Request {
        GitHubReleaseCollection::make(self.api_base_url()).create(draft)
    }

    fn release_update_request(
        &self,
        patch: &git_cognition_core::ReleasePatch,
    ) -> git_cognition_core::Request {
        GitHubRelease::make(self.api_base_url(), patch.release().clone()).update(patch)
    }

    fn release_delete_request(
        &self,
        release: &git_cognition_core::Release,
    ) -> git_cognition_core::Request {
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

    fn authentication(&self) -> Box<dyn git_cognition_core::Authentication> {
        Box::<TransportNotConfiguredAuthentication>::default()
    }

    fn organizations(&self) -> Box<dyn git_cognition_core::Organizations> {
        Box::<TransportNotConfiguredOrganizations>::default()
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
