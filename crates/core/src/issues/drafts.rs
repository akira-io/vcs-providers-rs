use crate::{IssueDraft, MissingIssueRepo, ProvidedIssueRepo, Repo};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueDraftBuilder<RepoState, TitleState> {
    pub(crate) repo: RepoState,
    pub(crate) title: TitleState,
    pub(crate) body: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingIssueTitle;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvidedIssueTitle {
    pub(crate) title: String,
}

impl<TitleState> IssueDraftBuilder<MissingIssueRepo, TitleState> {
    pub fn repo(self, repo: impl Into<Repo>) -> IssueDraftBuilder<ProvidedIssueRepo, TitleState> {
        IssueDraftBuilder {
            repo: ProvidedIssueRepo { repo: repo.into() },
            title: self.title,
            body: self.body,
        }
    }
}

impl<RepoState> IssueDraftBuilder<RepoState, MissingIssueTitle> {
    pub fn title(
        self,
        title: impl Into<String>,
    ) -> IssueDraftBuilder<RepoState, ProvidedIssueTitle> {
        IssueDraftBuilder {
            repo: self.repo,
            title: ProvidedIssueTitle {
                title: title.into(),
            },
            body: self.body,
        }
    }
}

impl<RepoState, TitleState> IssueDraftBuilder<RepoState, TitleState> {
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }
}

impl IssueDraftBuilder<ProvidedIssueRepo, ProvidedIssueTitle> {
    pub fn build(self) -> IssueDraft {
        self.get()
    }

    pub fn get(self) -> IssueDraft {
        IssueDraft {
            repo: self.repo.repo,
            title: self.title.title,
            body: self.body,
        }
    }
}
