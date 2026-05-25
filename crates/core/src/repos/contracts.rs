use crate::repos::{BoxFuture, Branch, Commit, Repo, RepositoryListQuery, RepositorySearchQuery};
use crate::{Page, Repository, VcsResult, transport_not_configured};

pub trait Repos: Send + Sync {
    fn get(&self, repo: Repo) -> BoxFuture<'_, VcsResult<Repository>>;

    fn list(&self, query: RepositoryListQuery) -> BoxFuture<'_, VcsResult<Page<Repository>>>;

    fn search(&self, query: RepositorySearchQuery) -> BoxFuture<'_, VcsResult<Page<Repository>>>;

    fn branches(&self, repo: Repo) -> BoxFuture<'_, VcsResult<Page<Branch>>>;

    fn commits(&self, repo: Repo) -> BoxFuture<'_, VcsResult<Page<Commit>>>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TransportNotConfiguredRepos;

impl Repos for TransportNotConfiguredRepos {
    fn get(&self, _repo: Repo) -> BoxFuture<'_, VcsResult<Repository>> {
        transport_not_configured()
    }

    fn list(&self, _query: RepositoryListQuery) -> BoxFuture<'_, VcsResult<Page<Repository>>> {
        transport_not_configured()
    }

    fn search(&self, _query: RepositorySearchQuery) -> BoxFuture<'_, VcsResult<Page<Repository>>> {
        transport_not_configured()
    }

    fn branches(&self, _repo: Repo) -> BoxFuture<'_, VcsResult<Page<Branch>>> {
        transport_not_configured()
    }

    fn commits(&self, _repo: Repo) -> BoxFuture<'_, VcsResult<Page<Commit>>> {
        transport_not_configured()
    }
}
