use serde::{Deserialize, Serialize};

use crate::{BoxFuture, Page, PageRequest, Repo, VcsResult, transport_not_configured};

#[path = "code_reviews/drafts.rs"]
mod drafts;
#[path = "code_reviews/patches.rs"]
mod patches;

pub use drafts::{
    CodeReviewDraftBuilder, MissingCodeReviewDraftRepo, MissingCodeReviewTitle,
    ProvidedCodeReviewDraftRepo, ProvidedCodeReviewTitle,
};
pub use patches::CodeReviewPatchBuilder;

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
    pub fn builder() -> CodeReviewBuilder<MissingCodeReviewRepo, MissingCodeReviewId> {
        CodeReviewBuilder {
            repo: MissingCodeReviewRepo,
            id: MissingCodeReviewId,
        }
    }

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingCodeReviewRepo;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvidedCodeReviewRepo {
    repo: Repo,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingCodeReviewId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvidedCodeReviewId {
    id: CodeReviewId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodeReviewBuilder<RepoState, CodeReviewIdState> {
    repo: RepoState,
    id: CodeReviewIdState,
}

impl CodeReviewBuilder<MissingCodeReviewRepo, MissingCodeReviewId> {
    pub fn draft(
        self,
    ) -> CodeReviewDraftBuilder<MissingCodeReviewDraftRepo, MissingCodeReviewTitle> {
        CodeReviewDraft::builder()
    }
}

impl<CodeReviewIdState> CodeReviewBuilder<MissingCodeReviewRepo, CodeReviewIdState> {
    pub fn repo(
        self,
        repo: impl Into<Repo>,
    ) -> CodeReviewBuilder<ProvidedCodeReviewRepo, CodeReviewIdState> {
        CodeReviewBuilder {
            repo: ProvidedCodeReviewRepo { repo: repo.into() },
            id: self.id,
        }
    }
}

impl<RepoState> CodeReviewBuilder<RepoState, MissingCodeReviewId> {
    pub fn id(self, id: impl Into<String>) -> CodeReviewBuilder<RepoState, ProvidedCodeReviewId> {
        CodeReviewBuilder {
            repo: self.repo,
            id: ProvidedCodeReviewId {
                id: CodeReviewId::make(id),
            },
        }
    }
}

impl CodeReviewBuilder<ProvidedCodeReviewRepo, ProvidedCodeReviewId> {
    pub fn build(self) -> CodeReview {
        self.get()
    }

    pub fn get(self) -> CodeReview {
        CodeReview {
            repo: self.repo.repo,
            id: self.id.id,
        }
    }
}

impl CodeReviewBuilder<MissingCodeReviewRepo, MissingCodeReviewId> {
    pub fn query(self) -> CodeReviewQueryBuilder {
        CodeReviewQueryBuilder
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CodeReviewQueryBuilder;

impl CodeReviewQueryBuilder {
    pub fn list(self, repo: Repo, page: Option<PageRequest>) -> CodeReviewListQuery {
        CodeReviewListQuery { repo, page }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CodeReviewListQuery {
    repo: Repo,
    page: Option<PageRequest>,
}

impl CodeReviewListQuery {
    pub fn repo(&self) -> &Repo {
        &self.repo
    }

    pub fn page(&self) -> Option<&PageRequest> {
        self.page.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CodeReviewDraft {
    repo: Repo,
    title: String,
    source: Option<String>,
    target: Option<String>,
    body: Option<String>,
}

impl CodeReviewDraft {
    pub fn builder() -> CodeReviewDraftBuilder<MissingCodeReviewDraftRepo, MissingCodeReviewTitle> {
        CodeReviewDraftBuilder {
            repo: MissingCodeReviewDraftRepo,
            title: MissingCodeReviewTitle,
            source: None,
            target: None,
            body: None,
        }
    }

    pub fn make(repo: Repo, title: impl Into<String>) -> Self {
        Self {
            repo,
            title: title.into(),
            source: None,
            target: None,
            body: None,
        }
    }

    pub fn repo(&self) -> &Repo {
        &self.repo
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }

    pub fn target(&self) -> Option<&str> {
        self.target.as_deref()
    }

    pub fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CodeReviewPatch {
    code_review: CodeReview,
    title: Option<String>,
    body: Option<String>,
    closed: Option<bool>,
}

impl CodeReviewPatch {
    pub fn code_review(&self) -> &CodeReview {
        &self.code_review
    }

    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    pub fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }

    pub fn closed(&self) -> Option<bool> {
        self.closed
    }
}

pub trait CodeReviews: Send + Sync {
    fn get(&self, repo: Repo, id: CodeReviewId) -> BoxFuture<'_, VcsResult<CodeReview>>;

    fn list(&self, query: CodeReviewListQuery) -> BoxFuture<'_, VcsResult<Page<CodeReview>>>;

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

    fn list(&self, _query: CodeReviewListQuery) -> BoxFuture<'_, VcsResult<Page<CodeReview>>> {
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
