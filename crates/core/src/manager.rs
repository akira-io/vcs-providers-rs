use crate::Provider;
use crate::{
    MissingOwnerName, MissingRepositoryName, PageRequest, ProvidedOwnerName,
    ProvidedRepositoryName, Repo, RepoBuilder, RepoQueryBuilder, RepositoryListQuery,
    RepositorySearchQuery, RequestUrl, repo,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VcsManager<Driver> {
    driver: Driver,
}

impl<Driver> VcsManager<Driver>
where
    Driver: ManagedProvider,
{
    pub fn repo(&self) -> ManagedRepoBuilder<Driver, MissingOwnerName, MissingRepositoryName> {
        ManagedRepoBuilder {
            manager: self.clone(),
            repo: repo(),
        }
    }

    pub fn driver(&self) -> &Driver {
        &self.driver
    }

    pub fn pagination(&self) -> crate::PaginationBuilder {
        crate::pagination()
    }
}

pub trait ManagedProvider: Clone + Provider {
    fn repo_url(&self, repo: &Repo) -> RequestUrl;

    fn repo_branches_url(&self, repo: &Repo, page: Option<&PageRequest>) -> RequestUrl;

    fn repo_commits_url(&self, repo: &Repo, page: Option<&PageRequest>) -> RequestUrl;

    fn repo_list_url(&self, query: &RepositoryListQuery) -> RequestUrl;

    fn repo_search_url(&self, query: &RepositorySearchQuery) -> RequestUrl;
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VcsManagerBuilder;

impl VcsManagerBuilder {
    pub fn driver<Driver>(self, driver: Driver) -> VcsManagerWithDriverBuilder<Driver>
    where
        Driver: ManagedProvider,
    {
        VcsManagerWithDriverBuilder { driver }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VcsManagerWithDriverBuilder<Driver> {
    driver: Driver,
}

impl<Driver> VcsManagerWithDriverBuilder<Driver>
where
    Driver: ManagedProvider,
{
    pub fn build(self) -> VcsManager<Driver> {
        VcsManager {
            driver: self.driver,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedRepoBuilder<Driver, OwnerNameState, RepositoryNameState> {
    manager: VcsManager<Driver>,
    repo: RepoBuilder<OwnerNameState, RepositoryNameState>,
}

impl<Driver> ManagedRepoBuilder<Driver, MissingOwnerName, MissingRepositoryName>
where
    Driver: ManagedProvider,
{
    pub fn collection(&self) -> ManagedRepoCollection<Driver> {
        ManagedRepoCollection {
            manager: self.manager.clone(),
        }
    }

    pub fn query(&self) -> crate::RepoQueryBuilder {
        RepoQueryBuilder
    }
}

impl<Driver, RepositoryNameState> ManagedRepoBuilder<Driver, MissingOwnerName, RepositoryNameState>
where
    Driver: ManagedProvider,
{
    pub fn owner(
        self,
        owner_name: impl Into<String>,
    ) -> ManagedRepoBuilder<Driver, ProvidedOwnerName, RepositoryNameState> {
        ManagedRepoBuilder {
            manager: self.manager,
            repo: self.repo.owner(owner_name),
        }
    }
}

impl<Driver, OwnerNameState> ManagedRepoBuilder<Driver, OwnerNameState, MissingRepositoryName>
where
    Driver: ManagedProvider,
{
    pub fn name(
        self,
        repository_name: impl Into<String>,
    ) -> ManagedRepoBuilder<Driver, OwnerNameState, ProvidedRepositoryName> {
        ManagedRepoBuilder {
            manager: self.manager,
            repo: self.repo.name(repository_name),
        }
    }
}

impl<Driver> ManagedRepoBuilder<Driver, ProvidedOwnerName, ProvidedRepositoryName>
where
    Driver: ManagedProvider,
{
    pub fn build(self) -> ManagedRepo<Driver> {
        ManagedRepo {
            manager: self.manager,
            repo: self.repo.build(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedRepo<Driver> {
    manager: VcsManager<Driver>,
    repo: Repo,
}

impl<Driver> ManagedRepo<Driver>
where
    Driver: ManagedProvider,
{
    pub fn url(&self) -> RequestUrl {
        self.manager.driver.repo_url(&self.repo)
    }

    pub fn branches(&self, page: Option<&PageRequest>) -> RequestUrl {
        self.manager.driver.repo_branches_url(&self.repo, page)
    }

    pub fn commits(&self, page: Option<&PageRequest>) -> RequestUrl {
        self.manager.driver.repo_commits_url(&self.repo, page)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedRepoCollection<Driver> {
    manager: VcsManager<Driver>,
}

impl<Driver> ManagedRepoCollection<Driver>
where
    Driver: ManagedProvider,
{
    pub fn list(&self, query: &RepositoryListQuery) -> RequestUrl {
        self.manager.driver.repo_list_url(query)
    }

    pub fn search(&self, query: &RepositorySearchQuery) -> RequestUrl {
        self.manager.driver.repo_search_url(query)
    }
}
