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

pub trait Issues: Send + Sync {
    fn get(&self, repo: Repo, id: IssueId) -> BoxFuture<'_, VcsResult<Issue>>;

    fn list(&self, repo: Repo, page: Option<PageRequest>) -> BoxFuture<'_, VcsResult<Page<Issue>>>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TransportNotConfiguredIssues;

impl Issues for TransportNotConfiguredIssues {
    fn get(&self, _repo: Repo, _id: IssueId) -> BoxFuture<'_, VcsResult<Issue>> {
        transport_not_configured()
    }

    fn list(
        &self,
        _repo: Repo,
        _page: Option<PageRequest>,
    ) -> BoxFuture<'_, VcsResult<Page<Issue>>> {
        transport_not_configured()
    }
}
