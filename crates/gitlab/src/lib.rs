use vcs_provider_core::{
    AuthHeaderStyle, AuthKind, Capability, CodeReviewPatchBuilder, CodeReviews, Issues,
    ManagedCodeReviewProvider, ManagedIssueDeleteProvider, ManagedIssueProvider, ManagedProvider,
    MissingCodeReviewId, MissingCodeReviewRepo, MissingOwnerName, MissingReleaseId,
    MissingReleaseRepo, MissingRepositoryName, Pipelines, Provider, ProviderDescriptor, ProviderId,
    Releases, Repos, TransportNotConfiguredCodeReviews, TransportNotConfiguredIssues,
    TransportNotConfiguredPipelines, TransportNotConfiguredReleases, TransportNotConfiguredRepos,
    capabilities,
};

mod client;
mod code_reviews;
mod issues;
mod mappers;
mod pipelines;
mod provider_pipelines;
mod releases;
mod repos;

pub use client::GitLabClient;
pub use code_reviews::{GitLabCodeReview, GitLabCodeReviewCollection};
pub use issues::{GitLabIssue, GitLabIssueCollection};
pub use pipelines::{GitLabPipeline, GitLabPipelineCollection};
pub use releases::{GitLabRelease, GitLabReleaseCollection};
pub use repos::{GitLabRepo, GitLabRepoCollection};

pub const PROVIDER_ID: &str = "gitlab";
pub const DISPLAY_NAME: &str = "GitLab";
pub const DEFAULT_BASE_URL: &str = "https://gitlab.com";

#[derive(Clone, Copy, Debug, Default)]
pub struct GitLabProvider;

impl GitLabProvider {
    pub fn repo(
        &self,
    ) -> vcs_provider_core::ManagedRepoBuilder<Self, MissingOwnerName, MissingRepositoryName> {
        vcs_provider_core::vcs(*self).repo()
    }

    pub fn issue(
        &self,
    ) -> vcs_provider_core::ManagedIssueBuilder<
        Self,
        vcs_provider_core::MissingIssueRepo,
        vcs_provider_core::MissingIssueId,
    > {
        vcs_provider_core::vcs(*self).issue()
    }

    pub fn code_review(
        &self,
    ) -> vcs_provider_core::ManagedCodeReviewBuilder<Self, MissingCodeReviewRepo, MissingCodeReviewId>
    {
        vcs_provider_core::vcs(*self).code_review()
    }

    pub fn release(
        &self,
    ) -> vcs_provider_core::ManagedReleaseBuilder<Self, MissingReleaseRepo, MissingReleaseId> {
        vcs_provider_core::vcs(*self).release()
    }

    pub fn pagination(&self) -> vcs_provider_core::PaginationBuilder {
        vcs_provider_core::pagination()
    }
}

impl ManagedProvider for GitLabProvider {
    fn repo_url(&self, repo: &vcs_provider_core::Repo) -> vcs_provider_core::RequestUrl {
        GitLabRepo::make(DEFAULT_BASE_URL, repo.clone()).url()
    }

    fn repo_branches_url(
        &self,
        repo: &vcs_provider_core::Repo,
        page: Option<&vcs_provider_core::PageRequest>,
    ) -> vcs_provider_core::RequestUrl {
        GitLabRepo::make(DEFAULT_BASE_URL, repo.clone()).branches(page)
    }

    fn repo_commits_url(
        &self,
        repo: &vcs_provider_core::Repo,
        page: Option<&vcs_provider_core::PageRequest>,
    ) -> vcs_provider_core::RequestUrl {
        GitLabRepo::make(DEFAULT_BASE_URL, repo.clone()).commits(page)
    }

    fn repo_list_url(
        &self,
        query: &vcs_provider_core::RepositoryListQuery,
    ) -> vcs_provider_core::RequestUrl {
        GitLabRepoCollection::make(DEFAULT_BASE_URL).list(query)
    }

    fn repo_search_url(
        &self,
        query: &vcs_provider_core::RepositorySearchQuery,
    ) -> vcs_provider_core::RequestUrl {
        GitLabRepoCollection::make(DEFAULT_BASE_URL).search(query)
    }

    fn repo_create_request(
        &self,
        draft: &vcs_provider_core::RepositoryDraft,
    ) -> vcs_provider_core::Request {
        GitLabRepoCollection::make(DEFAULT_BASE_URL).create(draft)
    }

    fn repo_update_request(
        &self,
        patch: &vcs_provider_core::RepositoryPatch,
    ) -> vcs_provider_core::Request {
        GitLabRepo::make(DEFAULT_BASE_URL, patch.repo().clone()).update(patch)
    }

    fn repo_delete_request(&self, repo: &vcs_provider_core::Repo) -> vcs_provider_core::Request {
        GitLabRepo::make(DEFAULT_BASE_URL, repo.clone()).delete()
    }
}

impl ManagedIssueProvider for GitLabProvider {
    fn issue_url(&self, issue: &vcs_provider_core::Issue) -> vcs_provider_core::RequestUrl {
        GitLabIssue::make(DEFAULT_BASE_URL, issue.clone()).url()
    }

    fn issue_list_url(
        &self,
        query: &vcs_provider_core::IssueListQuery,
    ) -> vcs_provider_core::RequestUrl {
        GitLabIssueCollection::make(DEFAULT_BASE_URL).list(query)
    }

    fn issue_create_request(
        &self,
        draft: &vcs_provider_core::IssueDraft,
    ) -> vcs_provider_core::Request {
        GitLabIssueCollection::make(DEFAULT_BASE_URL).create(draft)
    }

    fn issue_update_request(
        &self,
        patch: &vcs_provider_core::IssuePatch,
    ) -> vcs_provider_core::Request {
        GitLabIssue::make(DEFAULT_BASE_URL, patch.issue().clone()).update(patch)
    }
}

impl ManagedIssueDeleteProvider for GitLabProvider {
    fn issue_delete_request(&self, issue: &vcs_provider_core::Issue) -> vcs_provider_core::Request {
        GitLabIssue::make(DEFAULT_BASE_URL, issue.clone()).delete()
    }
}

impl ManagedCodeReviewProvider for GitLabProvider {
    fn code_review_url(
        &self,
        code_review: &vcs_provider_core::CodeReview,
    ) -> vcs_provider_core::RequestUrl {
        GitLabCodeReview::make(DEFAULT_BASE_URL, code_review.clone()).url()
    }

    fn code_review_list_url(
        &self,
        query: &vcs_provider_core::CodeReviewListQuery,
    ) -> vcs_provider_core::RequestUrl {
        GitLabCodeReviewCollection::make(DEFAULT_BASE_URL).list(query)
    }

    fn code_review_create_request(
        &self,
        draft: &vcs_provider_core::CodeReviewDraft,
    ) -> vcs_provider_core::Request {
        GitLabCodeReviewCollection::make(DEFAULT_BASE_URL).create(draft)
    }

    fn code_review_update_request(
        &self,
        patch: &vcs_provider_core::CodeReviewPatch,
    ) -> vcs_provider_core::Request {
        GitLabCodeReview::make(DEFAULT_BASE_URL, patch.code_review().clone()).update(patch)
    }

    fn code_review_close_request(
        &self,
        code_review: &vcs_provider_core::CodeReview,
    ) -> vcs_provider_core::Request {
        let close_patch = CodeReviewPatchBuilder::make(code_review.clone())
            .closed()
            .get();

        GitLabCodeReview::make(DEFAULT_BASE_URL, code_review.clone()).update(&close_patch)
    }
}

impl vcs_provider_core::ManagedCodeReviewDeleteProvider for GitLabProvider {
    fn code_review_delete_request(
        &self,
        code_review: &vcs_provider_core::CodeReview,
    ) -> vcs_provider_core::Request {
        GitLabCodeReview::make(DEFAULT_BASE_URL, code_review.clone()).delete()
    }
}

impl vcs_provider_core::ManagedReleaseProvider for GitLabProvider {
    fn release_url(&self, release: &vcs_provider_core::Release) -> vcs_provider_core::RequestUrl {
        GitLabRelease::make(DEFAULT_BASE_URL, release.clone()).url()
    }

    fn release_list_url(
        &self,
        query: &vcs_provider_core::ReleaseListQuery,
    ) -> vcs_provider_core::RequestUrl {
        GitLabReleaseCollection::make(DEFAULT_BASE_URL).list(query)
    }

    fn release_create_request(
        &self,
        draft: &vcs_provider_core::ReleaseDraft,
    ) -> vcs_provider_core::Request {
        GitLabReleaseCollection::make(DEFAULT_BASE_URL).create(draft)
    }

    fn release_update_request(
        &self,
        patch: &vcs_provider_core::ReleasePatch,
    ) -> vcs_provider_core::Request {
        GitLabRelease::make(DEFAULT_BASE_URL, patch.release().clone()).update(patch)
    }

    fn release_delete_request(
        &self,
        release: &vcs_provider_core::Release,
    ) -> vcs_provider_core::Request {
        GitLabRelease::make(DEFAULT_BASE_URL, release.clone()).delete()
    }
}

impl Provider for GitLabProvider {
    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptor::make(
            ProviderId::make(PROVIDER_ID),
            DISPLAY_NAME,
            capabilities().make([
                Capability::Repos,
                Capability::Issues,
                Capability::CodeReviews,
                Capability::Pipelines,
                Capability::Releases,
                Capability::Organizations,
                Capability::Webhooks,
                Capability::SelfHosted,
            ]),
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
        DEFAULT_BASE_URL
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
    GitLabProvider
}
