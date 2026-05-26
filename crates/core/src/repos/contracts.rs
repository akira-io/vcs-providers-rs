use crate::repos::{
    BoxFuture, Branch, Commit, Repo, RepositoryDraft, RepositoryListQuery, RepositoryPatch,
    RepositorySearchQuery,
};
use crate::{Page, Repository, VcsResult, transport_not_configured};

pub trait Repos: Send + Sync {
    fn get(&self, repo: Repo) -> BoxFuture<'_, VcsResult<Repository>>;

    fn list(&self, query: RepositoryListQuery) -> BoxFuture<'_, VcsResult<Page<Repository>>>;

    fn search(&self, query: RepositorySearchQuery) -> BoxFuture<'_, VcsResult<Page<Repository>>>;

    fn create(&self, draft: RepositoryDraft) -> BoxFuture<'_, VcsResult<Repository>>;

    fn update(&self, patch: RepositoryPatch) -> BoxFuture<'_, VcsResult<Repository>>;

    fn delete(&self, repo: Repo) -> BoxFuture<'_, VcsResult<()>>;

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

    fn create(&self, _draft: RepositoryDraft) -> BoxFuture<'_, VcsResult<Repository>> {
        transport_not_configured()
    }

    fn update(&self, _patch: RepositoryPatch) -> BoxFuture<'_, VcsResult<Repository>> {
        transport_not_configured()
    }

    fn delete(&self, _repo: Repo) -> BoxFuture<'_, VcsResult<()>> {
        transport_not_configured()
    }

    fn branches(&self, _repo: Repo) -> BoxFuture<'_, VcsResult<Page<Branch>>> {
        transport_not_configured()
    }

    fn commits(&self, _repo: Repo) -> BoxFuture<'_, VcsResult<Page<Commit>>> {
        transport_not_configured()
    }
}
