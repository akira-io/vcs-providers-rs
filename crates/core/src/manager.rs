use crate::{AuthCredential, Provider, Transport};
use crate::{
    CodeReview, CodeReviewDraft, CodeReviewListQuery, CodeReviewPatch, Issue, IssueDraft,
    IssueListQuery, IssuePatch, ManagedPipelineProvider, MissingCodeReviewId,
    MissingCodeReviewRepo, MissingIssueId, MissingIssueRepo, MissingOwnerName, MissingPipelineId,
    MissingPipelineRepo, MissingReleaseId, MissingReleaseRepo, MissingRepositoryName, PageRequest,
    Release, ReleaseDraft, ReleaseListQuery, ReleasePatch, Repo, RepositoryDraft,
    RepositoryListQuery, RepositoryPatch, RepositorySearchQuery, RequestUrl, code_review, issue,
    pipeline, release, repo,
};

mod code_review_mutations;
mod code_reviews;
mod issue_mutations;
mod issues;
mod pipelines;
mod release_mutations;
mod releases;
mod repo_mutations;
mod repos;

#[allow(unused_imports)]
pub use code_review_mutations::ManagedCodeReviewUpdateBuilder;
pub use code_reviews::{
    ManagedCodeReview, ManagedCodeReviewBuilder, ManagedCodeReviewCollection,
    ManagedCodeReviewDraftBuilder, ManagedRepoCodeReviews, ManagedRepoCodeReviewsPagination,
};
#[allow(unused_imports)]
pub use issue_mutations::ManagedIssueUpdateBuilder;
pub use issues::{
    ManagedIssue, ManagedIssueBuilder, ManagedIssueCollection, ManagedIssueDraftBuilder,
    ManagedRepoIssues, ManagedRepoIssuesPagination,
};
pub use pipelines::{
    ManagedPipeline, ManagedPipelineBuilder, ManagedPipelineCollection, ManagedRepoPipelines,
    ManagedRepoPipelinesPagination,
};
#[allow(unused_imports)]
pub use release_mutations::ManagedReleaseUpdateBuilder;
pub use releases::{
    ManagedRelease, ManagedReleaseBuilder, ManagedReleaseCollection, ManagedReleaseDraftBuilder,
    ManagedRepoReleases, ManagedRepoReleasesPagination,
};
#[allow(unused_imports)]
pub use repo_mutations::ManagedRepositoryUpdateBuilder;
pub use repos::{
    ManagedRepo, ManagedRepoBuilder, ManagedRepoCollection, ManagedRepositoryDraftBuilder,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VcsManager<Driver> {
    driver: Driver,
}

impl<Driver> VcsManager<Driver>
where
    Driver: ManagedProvider,
{
    pub fn repo(&self) -> ManagedRepoBuilder<Driver, MissingOwnerName, MissingRepositoryName> {
        ManagedRepoBuilder {
            manager: self.clone(),
            repo: repo(),
        }
    }

    pub fn issue(&self) -> ManagedIssueBuilder<Driver, MissingIssueRepo, MissingIssueId>
    where
        Driver: ManagedIssueProvider,
    {
        ManagedIssueBuilder {
            manager: self.clone(),
            issue: issue(),
        }
    }

    pub fn code_review(
        &self,
    ) -> ManagedCodeReviewBuilder<Driver, MissingCodeReviewRepo, MissingCodeReviewId>
    where
        Driver: ManagedCodeReviewProvider,
    {
        ManagedCodeReviewBuilder {
            manager: self.clone(),
            code_review: code_review(),
        }
    }

    pub fn release(&self) -> ManagedReleaseBuilder<Driver, MissingReleaseRepo, MissingReleaseId>
    where
        Driver: ManagedReleaseProvider,
    {
        ManagedReleaseBuilder {
            manager: self.clone(),
            release: release(),
        }
    }

    pub fn pipeline(&self) -> ManagedPipelineBuilder<Driver, MissingPipelineRepo, MissingPipelineId>
    where
        Driver: ManagedPipelineProvider,
    {
        ManagedPipelineBuilder {
            manager: self.clone(),
            pipeline: pipeline(),
        }
    }

    pub fn driver(&self) -> &Driver {
        &self.driver
    }

    pub fn pagination(&self) -> crate::PaginationBuilder {
        crate::pagination()
    }

    pub fn transport(&self, transport: impl Transport + 'static) -> Driver::Client
    where
        Driver: ManagedClientProvider,
    {
        self.driver.client(transport)
    }
}

pub trait ProviderClient: Clone + Provider {
    fn auth(self, credential: AuthCredential) -> Self;
}

pub trait ManagedClientProvider: ManagedProvider {
    type Client: ProviderClient;

    fn client(&self, transport: impl Transport + 'static) -> Self::Client;
}

pub trait ManagedProvider: Clone + Provider {
    fn repo_url(&self, repo: &Repo) -> RequestUrl;

    fn repo_branches_url(&self, repo: &Repo, page: Option<&PageRequest>) -> RequestUrl;

    fn repo_commits_url(&self, repo: &Repo, page: Option<&PageRequest>) -> RequestUrl;

    fn repo_list_url(&self, query: &RepositoryListQuery) -> RequestUrl;

    fn repo_search_url(&self, query: &RepositorySearchQuery) -> RequestUrl;

    fn repo_create_request(&self, draft: &RepositoryDraft) -> crate::Request;

    fn repo_update_request(&self, patch: &RepositoryPatch) -> crate::Request;

    fn repo_delete_request(&self, repo: &Repo) -> crate::Request;
}

pub trait ManagedIssueProvider: ManagedProvider {
    fn issue_url(&self, issue: &Issue) -> RequestUrl;

    fn issue_list_url(&self, query: &IssueListQuery) -> RequestUrl;

    fn issue_create_request(&self, draft: &IssueDraft) -> crate::Request;

    fn issue_update_request(&self, patch: &IssuePatch) -> crate::Request;

    fn issue_close_request(&self, patch: &IssuePatch) -> crate::Request {
        self.issue_update_request(patch)
    }

    fn issue_delete_request(&self, _issue: &Issue) -> crate::VcsResult<crate::Request> {
        Err(crate::error().unsupported_operation("issue delete"))
    }
}

pub trait ManagedCodeReviewProvider: ManagedProvider {
    fn code_review_url(&self, code_review: &CodeReview) -> RequestUrl;

    fn code_review_list_url(&self, query: &CodeReviewListQuery) -> RequestUrl;

    fn code_review_create_request(&self, draft: &CodeReviewDraft) -> crate::Request;

    fn code_review_update_request(&self, patch: &CodeReviewPatch) -> crate::Request;

    fn code_review_merge_request(&self, code_review: &CodeReview) -> crate::Request;

    fn code_review_close_request(&self, code_review: &CodeReview) -> crate::Request;

    fn code_review_delete_request(
        &self,
        _code_review: &CodeReview,
    ) -> crate::VcsResult<crate::Request> {
        Err(crate::error().unsupported_operation("code review delete"))
    }
}

pub trait ManagedReleaseProvider: ManagedProvider {
    fn release_url(&self, release: &Release) -> RequestUrl;

    fn release_list_url(&self, query: &ReleaseListQuery) -> RequestUrl;

    fn release_create_request(&self, draft: &ReleaseDraft) -> crate::Request;

    fn release_update_request(&self, patch: &ReleasePatch) -> crate::Request;

    fn release_delete_request(&self, release: &Release) -> crate::Request;
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VcsManagerBuilder;

impl VcsManagerBuilder {
    pub fn driver<Driver>(self, driver: Driver) -> VcsManagerWithDriverBuilder<Driver>
    where
        Driver: ManagedProvider,
    {
        VcsManagerWithDriverBuilder { driver }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VcsManagerWithDriverBuilder<Driver> {
    driver: Driver,
}

impl<Driver> VcsManagerWithDriverBuilder<Driver>
where
    Driver: ManagedProvider,
{
    pub fn build(self) -> VcsManager<Driver> {
        VcsManager {
            driver: self.driver,
        }
    }
}
