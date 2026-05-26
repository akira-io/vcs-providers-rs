use vcs_provider_core::{
    AuthHeaderStyle, AuthKind, Capability, CodeReviews, Issues, ManagedCodeReviewProvider,
    ManagedProvider, MissingCodeReviewId, MissingCodeReviewRepo, MissingOwnerName,
    MissingRepositoryName, Pipelines, Provider, ProviderDescriptor, ProviderId, Releases, Repos,
    TransportNotConfiguredCodeReviews, TransportNotConfiguredIssues,
    TransportNotConfiguredPipelines, TransportNotConfiguredReleases, TransportNotConfiguredRepos,
    capabilities,
};

mod client;
mod code_reviews;
mod mappers;
mod pipelines;
mod provider_pipelines;
mod repos;

pub use client::BitbucketClient;
pub use code_reviews::{BitbucketCodeReview, BitbucketCodeReviewCollection};
pub use pipelines::{BitbucketPipeline, BitbucketPipelineCollection};
pub use repos::{BitbucketRepo, BitbucketRepoCollection};

pub const PROVIDER_ID: &str = "bitbucket";
pub const DISPLAY_NAME: &str = "Bitbucket";
pub const DEFAULT_BASE_URL: &str = "https://api.bitbucket.org/2.0";

#[derive(Clone, Copy, Debug, Default)]
pub struct BitbucketProvider;

impl BitbucketProvider {
    pub fn repo(
        &self,
    ) -> vcs_provider_core::ManagedRepoBuilder<Self, MissingOwnerName, MissingRepositoryName> {
        vcs_provider_core::vcs(*self).repo()
    }

    pub fn code_review(
        &self,
    ) -> vcs_provider_core::ManagedCodeReviewBuilder<Self, MissingCodeReviewRepo, MissingCodeReviewId>
    {
        vcs_provider_core::vcs(*self).code_review()
    }

    pub fn pagination(&self) -> vcs_provider_core::PaginationBuilder {
        vcs_provider_core::pagination()
    }
}

impl ManagedProvider for BitbucketProvider {
    fn repo_url(&self, repo: &vcs_provider_core::Repo) -> vcs_provider_core::RequestUrl {
        BitbucketRepo::make(DEFAULT_BASE_URL, repo.clone()).url()
    }

    fn repo_branches_url(
        &self,
        repo: &vcs_provider_core::Repo,
        page: Option<&vcs_provider_core::PageRequest>,
    ) -> vcs_provider_core::RequestUrl {
        BitbucketRepo::make(DEFAULT_BASE_URL, repo.clone()).branches(page)
    }

    fn repo_commits_url(
        &self,
        repo: &vcs_provider_core::Repo,
        page: Option<&vcs_provider_core::PageRequest>,
    ) -> vcs_provider_core::RequestUrl {
        BitbucketRepo::make(DEFAULT_BASE_URL, repo.clone()).commits(page)
    }

    fn repo_list_url(
        &self,
        query: &vcs_provider_core::RepositoryListQuery,
    ) -> vcs_provider_core::RequestUrl {
        BitbucketRepoCollection::make(DEFAULT_BASE_URL).list(query)
    }

    fn repo_search_url(
        &self,
        query: &vcs_provider_core::RepositorySearchQuery,
    ) -> vcs_provider_core::RequestUrl {
        BitbucketRepoCollection::make(DEFAULT_BASE_URL).search(query)
    }

    fn repo_create_request(
        &self,
        draft: &vcs_provider_core::RepositoryDraft,
    ) -> vcs_provider_core::Request {
        BitbucketRepo::make(DEFAULT_BASE_URL, draft.repo().clone()).create(draft)
    }

    fn repo_update_request(
        &self,
        patch: &vcs_provider_core::RepositoryPatch,
    ) -> vcs_provider_core::Request {
        BitbucketRepo::make(DEFAULT_BASE_URL, patch.repo().clone()).update(patch)
    }

    fn repo_delete_request(&self, repo: &vcs_provider_core::Repo) -> vcs_provider_core::Request {
        BitbucketRepo::make(DEFAULT_BASE_URL, repo.clone()).delete()
    }
}

impl ManagedCodeReviewProvider for BitbucketProvider {
    fn code_review_url(
        &self,
        code_review: &vcs_provider_core::CodeReview,
    ) -> vcs_provider_core::RequestUrl {
        BitbucketCodeReview::make(DEFAULT_BASE_URL, code_review.clone()).url()
    }

    fn code_review_list_url(
        &self,
        query: &vcs_provider_core::CodeReviewListQuery,
    ) -> vcs_provider_core::RequestUrl {
        BitbucketCodeReviewCollection::make(DEFAULT_BASE_URL).list(query)
    }

    fn code_review_create_request(
        &self,
        draft: &vcs_provider_core::CodeReviewDraft,
    ) -> vcs_provider_core::Request {
        BitbucketCodeReviewCollection::make(DEFAULT_BASE_URL).create(draft)
    }

    fn code_review_update_request(
        &self,
        patch: &vcs_provider_core::CodeReviewPatch,
    ) -> vcs_provider_core::Request {
        BitbucketCodeReview::make(DEFAULT_BASE_URL, patch.code_review().clone()).update(patch)
    }

    fn code_review_close_request(
        &self,
        code_review: &vcs_provider_core::CodeReview,
    ) -> vcs_provider_core::Request {
        BitbucketCodeReview::make(DEFAULT_BASE_URL, code_review.clone()).close()
    }
}

impl Provider for BitbucketProvider {
    fn descriptor(&self) -> ProviderDescriptor {
        ProviderDescriptor::make(
            ProviderId::make(PROVIDER_ID),
            DISPLAY_NAME,
            capabilities().make([
                Capability::Repos,
                Capability::CodeReviews,
                Capability::Pipelines,
                Capability::Webhooks,
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
            AuthKind::PersonalAccessToken => AuthHeaderStyle::AuthorizationBearer,
            AuthKind::OAuth => AuthHeaderStyle::AuthorizationBearer,
            AuthKind::AppInstallation => AuthHeaderStyle::AuthorizationBearer,
            AuthKind::Jwt => AuthHeaderStyle::AuthorizationBearer,
        }
    }
}

pub fn bitbucket() -> BitbucketProvider {
    BitbucketProvider
}
