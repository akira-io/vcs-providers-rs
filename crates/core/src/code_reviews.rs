use serde::{Deserialize, Serialize};

use crate::{BoxFuture, Page, PageRequest, Repo, VcsResult, transport_not_configured};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CodeReviewId(String);

impl CodeReviewId {
    pub fn make(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CodeReview {
    repo: Repo,
    id: CodeReviewId,
}

impl CodeReview {
    pub fn make(repo: Repo, id: CodeReviewId) -> Self {
        Self { repo, id }
    }

    pub fn repo(&self) -> &Repo {
        &self.repo
    }

    pub fn id(&self) -> &CodeReviewId {
        &self.id
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CodeReviewDraft {
    repo: Repo,
    title: String,
}

impl CodeReviewDraft {
    pub fn make(repo: Repo, title: impl Into<String>) -> Self {
        Self {
            repo,
            title: title.into(),
        }
    }

    pub fn repo(&self) -> &Repo {
        &self.repo
    }

    pub fn title(&self) -> &str {
        &self.title
    }
}

pub trait CodeReviews: Send + Sync {
    fn get(&self, repo: Repo, id: CodeReviewId) -> BoxFuture<'_, VcsResult<CodeReview>>;

    fn list(
        &self,
        repo: Repo,
        page: Option<PageRequest>,
    ) -> BoxFuture<'_, VcsResult<Page<CodeReview>>>;

    fn create(&self, draft: CodeReviewDraft) -> BoxFuture<'_, VcsResult<CodeReview>>;

    fn merge(&self, code_review: CodeReview) -> BoxFuture<'_, VcsResult<CodeReview>>;

    fn close(&self, code_review: CodeReview) -> BoxFuture<'_, VcsResult<CodeReview>>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TransportNotConfiguredCodeReviews;

impl CodeReviews for TransportNotConfiguredCodeReviews {
    fn get(&self, _repo: Repo, _id: CodeReviewId) -> BoxFuture<'_, VcsResult<CodeReview>> {
        transport_not_configured()
    }

    fn list(
        &self,
        _repo: Repo,
        _page: Option<PageRequest>,
    ) -> BoxFuture<'_, VcsResult<Page<CodeReview>>> {
        transport_not_configured()
    }

    fn create(&self, _draft: CodeReviewDraft) -> BoxFuture<'_, VcsResult<CodeReview>> {
        transport_not_configured()
    }

    fn merge(&self, _code_review: CodeReview) -> BoxFuture<'_, VcsResult<CodeReview>> {
        transport_not_configured()
    }

    fn close(&self, _code_review: CodeReview) -> BoxFuture<'_, VcsResult<CodeReview>> {
        transport_not_configured()
    }
}
