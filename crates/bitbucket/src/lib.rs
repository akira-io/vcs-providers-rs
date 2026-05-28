use git_cognition_core::{
    AuthHeaderStyle, AuthKind, CodeReviews, Issues, ManagedAuthProvider, ManagedCodeReviewProvider,
    ManagedIssueProvider, ManagedOrganizationProvider, ManagedProvider, MissingCodeReviewId,
    MissingCodeReviewRepo, MissingIssueId, MissingIssueRepo, MissingOwnerName,
    MissingRepositoryName, Organizations, Pipelines, Provider, ProviderDescriptor, ProviderId,
    Releases, Repos, TransportNotConfiguredAuthentication, TransportNotConfiguredCodeReviews,
    TransportNotConfiguredIssues, TransportNotConfiguredOrganizations,
    TransportNotConfiguredPipelines, TransportNotConfiguredRepos, UnsupportedReleases,
};

mod capabilities;
mod client;
mod code_reviews;
mod issues;
mod mappers;
mod pagination;
mod pipelines;
mod provider_pipelines;
mod repos;
mod request_pagination;
mod response_fixture;

pub use client::BitbucketClient;
pub use code_reviews::{BitbucketCodeReview, BitbucketCodeReviewCollection};
pub use issues::{BitbucketIssue, BitbucketIssueCollection};
pub use pipelines::{BitbucketPipeline, BitbucketPipelineCollection};
pub use repos::{BitbucketRepo, BitbucketRepoCollection};
pub use response_fixture::BitbucketResponseBuilder;

use capabilities::bitbucket_capabilities;

pub const PROVIDER_ID: &str = "bitbucket";
pub const DISPLAY_NAME: &str = "Bitbucket";
pub const DEFAULT_BASE_URL: &str = "https://api.bitbucket.org/2.0";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitbucketProvider {
    base_url: String,
}

impl BitbucketProvider {
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

    pub fn issue(
        &self,
    ) -> git_cognition_core::ManagedIssueBuilder<Self, MissingIssueRepo, MissingIssueId> {
        git_cognition_core::cognition()
            .provider(self.clone())
            .issue()
    }

    pub fn pagination(&self) -> git_cognition_core::PaginationBuilder {
        git_cognition_core::pagination()
    }
}

impl ManagedProvider for BitbucketProvider {
    fn repo_url(&self, repo: &git_cognition_core::Repo) -> git_cognition_core::RequestUrl {
        BitbucketRepo::make(self.api_base_url(), repo.clone()).url()
    }

    fn repo_branches_url(
        &self,
        repo: &git_cognition_core::Repo,
        page: Option<&git_cognition_core::PageRequest>,
    ) -> git_cognition_core::RequestUrl {
        BitbucketRepo::make(self.api_base_url(), repo.clone()).branches(page)
    }

    fn repo_commits_url(
        &self,
        repo: &git_cognition_core::Repo,
        page: Option<&git_cognition_core::PageRequest>,
    ) -> git_cognition_core::RequestUrl {
        BitbucketRepo::make(self.api_base_url(), repo.clone()).commits(page)
    }

    fn repo_list_url(
        &self,
        query: &git_cognition_core::RepositoryListQuery,
    ) -> git_cognition_core::RequestUrl {
        BitbucketRepoCollection::make(self.api_base_url()).list(query)
    }

    fn repo_search_url(
        &self,
        query: &git_cognition_core::RepositorySearchQuery,
    ) -> git_cognition_core::RequestUrl {
        BitbucketRepoCollection::make(self.api_base_url()).search(query)
    }

    fn repo_create_request(
        &self,
        draft: &git_cognition_core::RepositoryDraft,
    ) -> git_cognition_core::Request {
        BitbucketRepo::make(self.api_base_url(), draft.repo().clone()).create(draft)
    }

    fn repo_update_request(
        &self,
        patch: &git_cognition_core::RepositoryPatch,
    ) -> git_cognition_core::Request {
        BitbucketRepo::make(self.api_base_url(), patch.repo().clone()).update(patch)
    }

    fn repo_delete_request(&self, repo: &git_cognition_core::Repo) -> git_cognition_core::Request {
        BitbucketRepo::make(self.api_base_url(), repo.clone()).delete()
    }

    fn repo_branch_create_request(
        &self,
        draft: &git_cognition_core::BranchDraft,
    ) -> git_cognition_core::CognitionResult<git_cognition_core::Request> {
        Ok(BitbucketRepo::make(self.api_base_url(), draft.repo().clone()).create_branch(draft))
    }

    fn repo_branch_delete_request(
        &self,
        repo: &git_cognition_core::Repo,
        branch_name: &str,
    ) -> git_cognition_core::CognitionResult<git_cognition_core::Request> {
        Ok(BitbucketRepo::make(self.api_base_url(), repo.clone()).delete_branch(branch_name))
    }
}

impl ManagedAuthProvider for BitbucketProvider {
    fn auth_validate_url(&self) -> git_cognition_core::RequestUrl {
        git_cognition_core::url(self.api_base_url())
            .path_segments(["user"])
            .build()
    }
}

impl ManagedOrganizationProvider for BitbucketProvider {
    fn organization_list_url(
        &self,
        query: Option<&git_cognition_core::OrganizationListQuery>,
    ) -> git_cognition_core::RequestUrl {
        let url =
            git_cognition_core::url(self.api_base_url()).path_segments(["user", "workspaces"]);

        match query.and_then(git_cognition_core::OrganizationListQuery::page) {
            Some(page) => crate::request_pagination::apply_page(url, Some(page)).build(),
            None => url.build(),
        }
    }
}

impl ManagedIssueProvider for BitbucketProvider {
    fn issue_url(&self, issue: &git_cognition_core::Issue) -> git_cognition_core::RequestUrl {
        BitbucketIssue::make(self.api_base_url(), issue.clone()).url()
    }

    fn issue_list_url(
        &self,
        query: &git_cognition_core::IssueListQuery,
    ) -> git_cognition_core::RequestUrl {
        BitbucketIssueCollection::make(self.api_base_url()).list(query)
    }

    fn issue_create_request(
        &self,
        draft: &git_cognition_core::IssueDraft,
    ) -> git_cognition_core::Request {
        BitbucketIssueCollection::make(self.api_base_url()).create(draft)
    }

    fn issue_update_request(
        &self,
        patch: &git_cognition_core::IssuePatch,
    ) -> git_cognition_core::Request {
        BitbucketIssue::make(self.api_base_url(), patch.issue().clone()).update(patch)
    }

    fn issue_delete_request(
        &self,
        issue: &git_cognition_core::Issue,
    ) -> git_cognition_core::CognitionResult<git_cognition_core::Request> {
        Ok(BitbucketIssue::make(self.api_base_url(), issue.clone()).delete())
    }
}

impl ManagedCodeReviewProvider for BitbucketProvider {
    fn code_review_url(
        &self,
        code_review: &git_cognition_core::CodeReview,
    ) -> git_cognition_core::RequestUrl {
        BitbucketCodeReview::make(self.api_base_url(), code_review.clone()).url()
    }

    fn code_review_list_url(
        &self,
        query: &git_cognition_core::CodeReviewListQuery,
    ) -> git_cognition_core::RequestUrl {
        BitbucketCodeReviewCollection::make(self.api_base_url()).list(query)
    }

    fn code_review_create_request(
        &self,
        draft: &git_cognition_core::CodeReviewDraft,
    ) -> git_cognition_core::Request {
        BitbucketCodeReviewCollection::make(self.api_base_url()).create(draft)
    }

    fn code_review_update_request(
        &self,
        patch: &git_cognition_core::CodeReviewPatch,
    ) -> git_cognition_core::Request {
        BitbucketCodeReview::make(self.api_base_url(), patch.code_review().clone()).update(patch)
    }

    fn code_review_merge_request(
        &self,
        code_review: &git_cognition_core::CodeReview,
    ) -> git_cognition_core::Request {
        BitbucketCodeReview::make(self.api_base_url(), code_review.clone()).merge()
    }

    fn code_review_close_request(
        &self,
        code_review: &git_cognition_core::CodeReview,
    ) -> git_cognition_core::Request {
        BitbucketCodeReview::make(self.api_base_url(), code_review.clone()).close()
    }
}

impl Provider for BitbucketProvider {
    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptor::make(
            ProviderId::make(PROVIDER_ID),
            DISPLAY_NAME,
            bitbucket_capabilities(),
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
        Box::<UnsupportedReleases>::default()
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

pub fn bitbucket() -> BitbucketProvider {
    BitbucketProvider::default()
}

impl Default for BitbucketProvider {
    fn default() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.into(),
        }
    }
}
