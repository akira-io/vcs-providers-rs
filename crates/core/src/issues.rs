use serde::{Deserialize, Serialize};

use crate::{BoxFuture, Page, Repo, VcsResult, transport_not_configured};

#[path = "issues/drafts.rs"]
mod drafts;
#[path = "issues/list.rs"]
mod list;
#[path = "issues/operations.rs"]
mod operations;
#[path = "issues/patches.rs"]
mod patches;
#[path = "issues/queries.rs"]
mod queries;
#[path = "issues/scoped.rs"]
mod scoped;
#[path = "issues/transport.rs"]
mod transport;

pub use drafts::{IssueDraftBuilder, MissingIssueTitle, ProvidedIssueTitle};
pub use list::{IssueListOperation, IssueListPaginationOperation};
#[allow(unused_imports)]
pub use operations::{
    IssueCloseOperation, IssueCreateOperation, IssueUpdateOperation, IssuesFluent,
};
pub use patches::IssuePatchBuilder;
#[allow(unused_imports)]
pub use queries::{IssueListQuery, IssueListQueryBuilder, IssueQueryBuilder};
pub use scoped::ScopedIssueOperation;
pub use transport::{IssueResponseMapper, TransportBackedIssues};

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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct IssueDraft {
    repo: Repo,
    title: String,
    body: Option<String>,
}

impl IssueDraft {
    pub fn repo(&self) -> &Repo {
        &self.repo
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct IssuePatch {
    issue: Issue,
    title: Option<String>,
    body: Option<String>,
    closed: Option<bool>,
}

impl IssuePatch {
    pub fn issue(&self) -> &Issue {
        &self.issue
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
        self.get()
    }

    pub fn get(self) -> Issue {
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

    pub fn draft(self) -> IssueDraftBuilder<MissingIssueRepo, MissingIssueTitle> {
        IssueDraftBuilder {
            repo: MissingIssueRepo,
            title: MissingIssueTitle,
            body: None,
        }
    }
}

pub trait Issues: Send + Sync {
    fn get(&self, repo: Repo, id: IssueId) -> BoxFuture<'_, VcsResult<Issue>>;

    fn list(&self, query: IssueListQuery) -> BoxFuture<'_, VcsResult<Page<Issue>>>;

    fn create(&self, draft: IssueDraft) -> BoxFuture<'_, VcsResult<Issue>>;

    fn update(&self, patch: IssuePatch) -> BoxFuture<'_, VcsResult<Issue>>;

    fn close(&self, patch: IssuePatch) -> BoxFuture<'_, VcsResult<Issue>>;
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

    fn create(&self, _draft: IssueDraft) -> BoxFuture<'_, VcsResult<Issue>> {
        transport_not_configured()
    }

    fn update(&self, _patch: IssuePatch) -> BoxFuture<'_, VcsResult<Issue>> {
        transport_not_configured()
    }

    fn close(&self, _patch: IssuePatch) -> BoxFuture<'_, VcsResult<Issue>> {
        transport_not_configured()
    }
}
