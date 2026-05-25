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
    pub fn repo(self, repo: Repo) -> CodeReviewBuilder<ProvidedCodeReviewRepo, CodeReviewIdState> {
        CodeReviewBuilder {
            repo: ProvidedCodeReviewRepo { repo },
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
        CodeReview {
            repo: self.repo.repo,
            id: self.id.id,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CodeReviewDraft {
    repo: Repo,
    title: String,
}

impl CodeReviewDraft {
    pub fn builder() -> CodeReviewDraftBuilder<MissingCodeReviewDraftRepo, MissingCodeReviewTitle> {
        CodeReviewDraftBuilder {
            repo: MissingCodeReviewDraftRepo,
            title: MissingCodeReviewTitle,
        }
    }

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingCodeReviewDraftRepo;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvidedCodeReviewDraftRepo {
    repo: Repo,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingCodeReviewTitle;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvidedCodeReviewTitle {
    title: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodeReviewDraftBuilder<RepoState, TitleState> {
    repo: RepoState,
    title: TitleState,
}

impl<TitleState> CodeReviewDraftBuilder<MissingCodeReviewDraftRepo, TitleState> {
    pub fn repo(
        self,
        repo: Repo,
    ) -> CodeReviewDraftBuilder<ProvidedCodeReviewDraftRepo, TitleState> {
        CodeReviewDraftBuilder {
            repo: ProvidedCodeReviewDraftRepo { repo },
            title: self.title,
        }
    }
}

impl<RepoState> CodeReviewDraftBuilder<RepoState, MissingCodeReviewTitle> {
    pub fn title(
        self,
        title: impl Into<String>,
    ) -> CodeReviewDraftBuilder<RepoState, ProvidedCodeReviewTitle> {
        CodeReviewDraftBuilder {
            repo: self.repo,
            title: ProvidedCodeReviewTitle {
                title: title.into(),
            },
        }
    }
}

impl CodeReviewDraftBuilder<ProvidedCodeReviewDraftRepo, ProvidedCodeReviewTitle> {
    pub fn build(self) -> CodeReviewDraft {
        CodeReviewDraft {
            repo: self.repo.repo,
            title: self.title.title,
        }
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
