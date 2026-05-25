use crate::{
    Issue, IssueBuilder, IssueDraft, IssueListQuery, IssuePatch, IssueQueryBuilder, MissingIssueId,
    MissingIssueRepo, PageRequest, PageRequestBuilder, ProvidedIssueId, ProvidedIssueRepo, Repo,
    Request, RequestUrl,
};

use super::{ManagedIssueProvider, VcsManager};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedIssueBuilder<Driver, RepoState, IssueIdState> {
    pub(super) manager: VcsManager<Driver>,
    pub(super) issue: IssueBuilder<RepoState, IssueIdState>,
}

impl<Driver> ManagedIssueBuilder<Driver, MissingIssueRepo, MissingIssueId>
where
    Driver: ManagedIssueProvider,
{
    pub fn collection(&self) -> ManagedIssueCollection<Driver> {
        ManagedIssueCollection {
            manager: self.manager.clone(),
        }
    }

    pub fn query(&self) -> IssueQueryBuilder {
        IssueQueryBuilder
    }
}

impl<Driver, IssueIdState> ManagedIssueBuilder<Driver, MissingIssueRepo, IssueIdState>
where
    Driver: ManagedIssueProvider,
{
    pub fn repo(
        self,
        repo: impl Into<Repo>,
    ) -> ManagedIssueBuilder<Driver, ProvidedIssueRepo, IssueIdState> {
        ManagedIssueBuilder {
            manager: self.manager,
            issue: self.issue.repo(repo),
        }
    }
}

impl<Driver, RepoState> ManagedIssueBuilder<Driver, RepoState, MissingIssueId>
where
    Driver: ManagedIssueProvider,
{
    pub fn id(
        self,
        id: impl Into<String>,
    ) -> ManagedIssueBuilder<Driver, RepoState, ProvidedIssueId> {
        ManagedIssueBuilder {
            manager: self.manager,
            issue: self.issue.id(id),
        }
    }
}

impl<Driver> ManagedIssueBuilder<Driver, ProvidedIssueRepo, ProvidedIssueId>
where
    Driver: ManagedIssueProvider,
{
    pub fn build(self) -> ManagedIssue<Driver> {
        ManagedIssue {
            manager: self.manager,
            issue: self.issue.build(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedIssue<Driver> {
    manager: VcsManager<Driver>,
    issue: Issue,
}

impl<Driver> ManagedIssue<Driver>
where
    Driver: ManagedIssueProvider,
{
    pub fn url(&self) -> RequestUrl {
        self.manager.driver.issue_url(&self.issue)
    }

    pub fn issue(&self) -> &Issue {
        &self.issue
    }

    pub fn repo(&self) -> &Repo {
        self.issue.repo()
    }

    pub fn update(&self, patch: &IssuePatch) -> Request {
        self.manager.driver.issue_update_request(patch)
    }

    pub fn delete(&self) -> Request {
        self.manager.driver.issue_delete_request(&self.issue)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedIssueCollection<Driver> {
    manager: VcsManager<Driver>,
}

impl<Driver> ManagedIssueCollection<Driver>
where
    Driver: ManagedIssueProvider,
{
    pub fn list(&self, query: &IssueListQuery) -> RequestUrl {
        self.manager.driver.issue_list_url(query)
    }

    pub fn create(&self, draft: &IssueDraft) -> Request {
        self.manager.driver.issue_create_request(draft)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedRepoIssues<Driver> {
    pub(super) manager: VcsManager<Driver>,
    pub(super) repo: Repo,
    pub(super) page: Option<PageRequest>,
}

impl<Driver> ManagedRepoIssues<Driver>
where
    Driver: ManagedIssueProvider,
{
    pub fn url(&self) -> RequestUrl {
        let query = self.query();
        self.manager.driver.issue_list_url(&query)
    }

    pub fn pagination(self) -> ManagedRepoIssuesPagination<Driver> {
        ManagedRepoIssuesPagination {
            manager: self.manager,
            repo: self.repo,
            page: PageRequestBuilder::default(),
        }
    }

    pub fn repo(&self) -> &Repo {
        &self.repo
    }

    fn query(&self) -> IssueListQuery {
        IssueQueryBuilder.list(self.repo.clone(), self.page.clone())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedRepoIssuesPagination<Driver> {
    manager: VcsManager<Driver>,
    repo: Repo,
    page: PageRequestBuilder,
}

impl<Driver> ManagedRepoIssuesPagination<Driver>
where
    Driver: ManagedIssueProvider,
{
    pub fn limit(mut self, limit: u16) -> Self {
        self.page = self.page.limit(limit);
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.page = self.page.cursor(cursor);
        self
    }

    pub fn build(self) -> ManagedRepoIssues<Driver> {
        self.get()
    }

    pub fn get(self) -> ManagedRepoIssues<Driver> {
        ManagedRepoIssues {
            manager: self.manager,
            repo: self.repo,
            page: Some(self.page.build()),
        }
    }

    pub fn url(self) -> RequestUrl {
        self.get().url()
    }
}
