use crate::{CodeReviewDraft, Repo};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingCodeReviewDraftRepo;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvidedCodeReviewDraftRepo {
    pub(crate) repo: Repo,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingCodeReviewTitle;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvidedCodeReviewTitle {
    pub(crate) title: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodeReviewDraftBuilder<RepoState, TitleState> {
    pub(crate) repo: RepoState,
    pub(crate) title: TitleState,
    pub(crate) source: Option<String>,
    pub(crate) target: Option<String>,
    pub(crate) body: Option<String>,
}

impl<TitleState> CodeReviewDraftBuilder<MissingCodeReviewDraftRepo, TitleState> {
    pub fn repo(
        self,
        repo: impl Into<Repo>,
    ) -> CodeReviewDraftBuilder<ProvidedCodeReviewDraftRepo, TitleState> {
        CodeReviewDraftBuilder {
            repo: ProvidedCodeReviewDraftRepo { repo: repo.into() },
            title: self.title,
            source: self.source,
            target: self.target,
            body: self.body,
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
            source: self.source,
            target: self.target,
            body: self.body,
        }
    }
}

impl<RepoState, TitleState> CodeReviewDraftBuilder<RepoState, TitleState> {
    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    pub fn target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }
}

impl CodeReviewDraftBuilder<ProvidedCodeReviewDraftRepo, ProvidedCodeReviewTitle> {
    pub fn build(self) -> CodeReviewDraft {
        self.get()
    }

    pub fn get(self) -> CodeReviewDraft {
        CodeReviewDraft {
            repo: self.repo.repo,
            title: self.title.title,
            source: self.source,
            target: self.target,
            body: self.body,
        }
    }
}
