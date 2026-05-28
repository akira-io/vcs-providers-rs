use git_cognition_core::{
    AuthHeaderStyle, AuthKind, CodeReviews, Issues, ManagedAuthProvider,
    ManagedOrganizationProvider, ManagedProvider, MissingCodeReviewId, MissingCodeReviewRepo,
    MissingOwnerName, MissingReleaseId, MissingReleaseRepo, MissingRepositoryName, Organizations,
    Pipelines, Provider, ProviderDescriptor, ProviderId, Releases, Repos,
    TransportNotConfiguredAuthentication, TransportNotConfiguredCodeReviews,
    TransportNotConfiguredIssues, TransportNotConfiguredOrganizations,
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
    ) -> git_cognition_core::ManagedRepoBuilder<Self, MissingOwnerName, MissingRepositoryName> {
        git_cognition_core::cognition()
            .provider(self.clone())
            .repo()
    }

    pub fn issue(
        &self,
    ) -> git_cognition_core::ManagedIssueBuilder<
        Self,
        git_cognition_core::MissingIssueRepo,
        git_cognition_core::MissingIssueId,
    > {
        git_cognition_core::cognition()
            .provider(self.clone())
            .issue()
    }

    pub fn code_review(
        &self,
    ) -> git_cognition_core::ManagedCodeReviewBuilder<
        Self,
        MissingCodeReviewRepo,
        MissingCodeReviewId,
    > {
        git_cognition_core::cognition()
            .provider(self.clone())
            .code_review()
    }

    pub fn release(
        &self,
    ) -> git_cognition_core::ManagedReleaseBuilder<Self, MissingReleaseRepo, MissingReleaseId> {
        git_cognition_core::cognition()
            .provider(self.clone())
            .release()
    }

    pub fn pagination(&self) -> git_cognition_core::PaginationBuilder {
        git_cognition_core::pagination()
    }
}

impl ManagedProvider for GitLabProvider {
    fn repo_url(&self, repo: &git_cognition_core::Repo) -> git_cognition_core::RequestUrl {
        GitLabRepo::make(self.api_base_url(), repo.clone()).url()
    }

    fn repo_branches_url(
        &self,
        repo: &git_cognition_core::Repo,
        page: Option<&git_cognition_core::PageRequest>,
    ) -> git_cognition_core::RequestUrl {
        GitLabRepo::make(self.api_base_url(), repo.clone()).branches(page)
    }

    fn repo_commits_url(
        &self,
        repo: &git_cognition_core::Repo,
        page: Option<&git_cognition_core::PageRequest>,
    ) -> git_cognition_core::RequestUrl {
        GitLabRepo::make(self.api_base_url(), repo.clone()).commits(page)
    }

    fn repo_list_url(
        &self,
        query: &git_cognition_core::RepositoryListQuery,
    ) -> git_cognition_core::RequestUrl {
        GitLabRepoCollection::make(self.api_base_url()).list(query)
    }

    fn repo_search_url(
        &self,
        query: &git_cognition_core::RepositorySearchQuery,
    ) -> git_cognition_core::RequestUrl {
        GitLabRepoCollection::make(self.api_base_url()).search(query)
    }

    fn repo_create_request(
        &self,
        draft: &git_cognition_core::RepositoryDraft,
    ) -> git_cognition_core::Request {
        GitLabRepoCollection::make(self.api_base_url()).create(draft)
    }

    fn repo_update_request(
        &self,
        patch: &git_cognition_core::RepositoryPatch,
    ) -> git_cognition_core::Request {
        GitLabRepo::make(self.api_base_url(), patch.repo().clone()).update(patch)
    }

    fn repo_delete_request(&self, repo: &git_cognition_core::Repo) -> git_cognition_core::Request {
        GitLabRepo::make(self.api_base_url(), repo.clone()).delete()
    }

    fn repo_branch_create_request(
        &self,
        draft: &git_cognition_core::BranchDraft,
    ) -> git_cognition_core::CognitionResult<git_cognition_core::Request> {
        Ok(GitLabRepo::make(self.api_base_url(), draft.repo().clone()).create_branch(draft))
    }

    fn repo_branch_delete_request(
        &self,
        repo: &git_cognition_core::Repo,
        branch_name: &str,
    ) -> git_cognition_core::CognitionResult<git_cognition_core::Request> {
        Ok(GitLabRepo::make(self.api_base_url(), repo.clone()).delete_branch(branch_name))
    }
}

impl ManagedAuthProvider for GitLabProvider {
    fn auth_validate_url(&self) -> git_cognition_core::RequestUrl {
        git_cognition_core::url(self.api_base_url())
            .path_segments(["api", "v4", "user"])
            .build()
    }
}

impl ManagedOrganizationProvider for GitLabProvider {
    fn organization_list_url(
        &self,
        query: Option<&git_cognition_core::OrganizationListQuery>,
    ) -> git_cognition_core::RequestUrl {
        let url =
            git_cognition_core::url(self.api_base_url()).path_segments(["api", "v4", "groups"]);

        match query.and_then(git_cognition_core::OrganizationListQuery::page) {
            Some(page) => crate::request_pagination::apply_page(url, Some(page)).build(),
            None => url.build(),
        }
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

    fn authentication(&self) -> Box<dyn git_cognition_core::Authentication> {
        Box::<TransportNotConfiguredAuthentication>::default()
    }

    fn organizations(&self) -> Box<dyn Organizations> {
        Box::<TransportNotConfiguredOrganizations>::default()
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
