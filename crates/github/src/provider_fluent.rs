use git_cognition_core::{
    ManagedCodeReviewBuilder, ManagedIssueBuilder, ManagedReleaseBuilder, ManagedRepoBuilder,
    MissingCodeReviewId, MissingCodeReviewRepo, MissingIssueId, MissingIssueRepo, MissingOwnerName,
    MissingReleaseId, MissingReleaseRepo, MissingRepositoryName, PaginationBuilder, cognition,
    pagination,
};

use crate::{DEFAULT_BASE_URL, GitHubProvider};

impl GitHubProvider {
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    pub fn api_base_url(&self) -> &str {
        &self.base_url
    }

    pub fn repo(&self) -> ManagedRepoBuilder<Self, MissingOwnerName, MissingRepositoryName> {
        cognition().provider(self.clone()).repo()
    }

    pub fn issue(&self) -> ManagedIssueBuilder<Self, MissingIssueRepo, MissingIssueId> {
        cognition().provider(self.clone()).issue()
    }

    pub fn code_review(
        &self,
    ) -> ManagedCodeReviewBuilder<Self, MissingCodeReviewRepo, MissingCodeReviewId> {
        cognition().provider(self.clone()).code_review()
    }

    pub fn release(&self) -> ManagedReleaseBuilder<Self, MissingReleaseRepo, MissingReleaseId> {
        cognition().provider(self.clone()).release()
    }

    pub fn pagination(&self) -> PaginationBuilder {
        pagination()
    }
}

impl Default for GitHubProvider {
    fn default() -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.into(),
        }
    }
}
