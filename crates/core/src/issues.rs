use serde::{Deserialize, Serialize};

use crate::{BoxFuture, Page, PageRequest, Repo, VcsResult, transport_not_configured};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct IssueId(String);

impl IssueId {
    pub fn make(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Issue {
    repo: Repo,
    id: IssueId,
}

impl Issue {
    pub fn builder() -> IssueBuilder<MissingIssueRepo, MissingIssueId> {
        IssueBuilder {
            repo: MissingIssueRepo,
            id: MissingIssueId,
        }
    }

    pub fn make(repo: Repo, id: IssueId) -> Self {
        Self { repo, id }
    }

    pub fn repo(&self) -> &Repo {
        &self.repo
    }

    pub fn id(&self) -> &IssueId {
        &self.id
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingIssueRepo;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvidedIssueRepo {
    repo: Repo,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingIssueId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvidedIssueId {
    id: IssueId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssueBuilder<RepoState, IssueIdState> {
    repo: RepoState,
    id: IssueIdState,
}

impl<IssueIdState> IssueBuilder<MissingIssueRepo, IssueIdState> {
    pub fn repo(self, repo: impl Into<Repo>) -> IssueBuilder<ProvidedIssueRepo, IssueIdState> {
        IssueBuilder {
            repo: ProvidedIssueRepo { repo: repo.into() },
            id: self.id,
        }
    }
}

impl<RepoState> IssueBuilder<RepoState, MissingIssueId> {
    pub fn id(self, id: impl Into<String>) -> IssueBuilder<RepoState, ProvidedIssueId> {
        IssueBuilder {
            repo: self.repo,
            id: ProvidedIssueId {
                id: IssueId::make(id),
            },
        }
    }
}

impl IssueBuilder<ProvidedIssueRepo, ProvidedIssueId> {
    pub fn build(self) -> Issue {
        Issue {
            repo: self.repo.repo,
            id: self.id.id,
        }
    }
}

impl IssueBuilder<MissingIssueRepo, MissingIssueId> {
    pub fn query(self) -> IssueQueryBuilder {
        IssueQueryBuilder
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IssueQueryBuilder;

impl IssueQueryBuilder {
    pub fn list(self, repo: impl Into<Repo>, page: Option<PageRequest>) -> IssueListQuery {
        IssueListQuery {
            repo: repo.into(),
            page,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct IssueListQuery {
    repo: Repo,
    page: Option<PageRequest>,
}

impl IssueListQuery {
    pub fn repo(&self) -> &Repo {
        &self.repo
    }

    pub fn page(&self) -> Option<&PageRequest> {
        self.page.as_ref()
    }
}

pub trait Issues: Send + Sync {
    fn get(&self, repo: Repo, id: IssueId) -> BoxFuture<'_, VcsResult<Issue>>;

    fn list(&self, query: IssueListQuery) -> BoxFuture<'_, VcsResult<Page<Issue>>>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TransportNotConfiguredIssues;

impl Issues for TransportNotConfiguredIssues {
    fn get(&self, _repo: Repo, _id: IssueId) -> BoxFuture<'_, VcsResult<Issue>> {
        transport_not_configured()
    }

    fn list(&self, _query: IssueListQuery) -> BoxFuture<'_, VcsResult<Page<Issue>>> {
        transport_not_configured()
    }
}
