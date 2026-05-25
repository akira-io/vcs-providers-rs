use crate::{
    Issue, IssueBuilder, IssueDraftBuilder, IssueListQuery, IssuePatch, IssueQueryBuilder,
    MissingIssueId, MissingIssueRepo, MissingIssueTitle, PageRequest, PageRequestBuilder,
    ProvidedIssueId, ProvidedIssueRepo, ProvidedIssueTitle, Repo, Request, RequestUrl,
};

use super::{ManagedIssueDeleteProvider, ManagedIssueProvider, VcsManager};

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

    pub fn draft(&self) -> ManagedIssueDraftBuilder<Driver, MissingIssueRepo, MissingIssueTitle> {
        ManagedIssueDraftBuilder {
            manager: self.manager.clone(),
            draft: crate::issue().draft(),
        }
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
        self.get()
    }

    pub fn get(self) -> ManagedIssue<Driver> {
        ManagedIssue {
            manager: self.manager,
            issue: self.issue.build(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedIssueDraftBuilder<Driver, RepoState, TitleState> {
    manager: VcsManager<Driver>,
    draft: IssueDraftBuilder<RepoState, TitleState>,
}

impl<Driver, TitleState> ManagedIssueDraftBuilder<Driver, MissingIssueRepo, TitleState>
where
    Driver: ManagedIssueProvider,
{
    pub fn repo(
        self,
        repo: impl Into<Repo>,
    ) -> ManagedIssueDraftBuilder<Driver, ProvidedIssueRepo, TitleState> {
        ManagedIssueDraftBuilder {
            manager: self.manager,
            draft: self.draft.repo(repo),
        }
    }
}

impl<Driver, RepoState> ManagedIssueDraftBuilder<Driver, RepoState, MissingIssueTitle>
where
    Driver: ManagedIssueProvider,
{
    pub fn title(
        self,
        title: impl Into<String>,
    ) -> ManagedIssueDraftBuilder<Driver, RepoState, ProvidedIssueTitle> {
        ManagedIssueDraftBuilder {
            manager: self.manager,
            draft: self.draft.title(title),
        }
    }
}

impl<Driver, RepoState, TitleState> ManagedIssueDraftBuilder<Driver, RepoState, TitleState>
where
    Driver: ManagedIssueProvider,
{
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.draft = self.draft.body(body);
        self
    }
}

impl<Driver> ManagedIssueDraftBuilder<Driver, ProvidedIssueRepo, ProvidedIssueTitle>
where
    Driver: ManagedIssueProvider,
{
    pub fn create(self) -> Request {
        self.manager.driver.issue_create_request(&self.draft.get())
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

    pub fn close(&self, patch: &IssuePatch) -> Request {
        self.manager.driver.issue_close_request(patch)
    }
}

impl<Driver> ManagedIssue<Driver>
where
    Driver: ManagedIssueDeleteProvider,
{
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
        self.list()
    }

    pub fn get(self) -> ManagedRepoIssues<Driver> {
        self.list()
    }

    pub fn list(self) -> ManagedRepoIssues<Driver> {
        ManagedRepoIssues {
            manager: self.manager,
            repo: self.repo,
            page: Some(self.page.build()),
        }
    }

    pub fn url(self) -> RequestUrl {
        self.list().url()
    }
}
