use crate::repos::{
    BoxFuture, Branch, BranchDraft, Commit, Repo, RepositoryDraft, RepositoryListQuery,
    RepositoryPatch, RepositorySearchQuery,
};
use crate::{CognitionResult, Page, Repository, transport_not_configured};

pub trait Repos: Send + Sync {
    fn get(&self, repo: Repo) -> BoxFuture<'_, CognitionResult<Repository>>;

    fn list(&self, query: RepositoryListQuery) -> BoxFuture<'_, CognitionResult<Page<Repository>>>;

    fn search(
        &self,
        query: RepositorySearchQuery,
    ) -> BoxFuture<'_, CognitionResult<Page<Repository>>>;

    fn create(&self, draft: RepositoryDraft) -> BoxFuture<'_, CognitionResult<Repository>>;

    fn update(&self, patch: RepositoryPatch) -> BoxFuture<'_, CognitionResult<Repository>>;

    fn delete(&self, repo: Repo) -> BoxFuture<'_, CognitionResult<()>>;

    fn branches(&self, repo: Repo) -> BoxFuture<'_, CognitionResult<Page<Branch>>>;

    fn create_branch(&self, draft: BranchDraft) -> BoxFuture<'_, CognitionResult<Branch>>;

    fn delete_branch(&self, repo: Repo, branch_name: String) -> BoxFuture<'_, CognitionResult<()>>;

    fn commits(&self, repo: Repo) -> BoxFuture<'_, CognitionResult<Page<Commit>>>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TransportNotConfiguredRepos;

impl Repos for TransportNotConfiguredRepos {
    fn get(&self, _repo: Repo) -> BoxFuture<'_, CognitionResult<Repository>> {
        transport_not_configured()
    }

    fn list(
        &self,
        _query: RepositoryListQuery,
    ) -> BoxFuture<'_, CognitionResult<Page<Repository>>> {
        transport_not_configured()
    }

    fn search(
        &self,
        _query: RepositorySearchQuery,
    ) -> BoxFuture<'_, CognitionResult<Page<Repository>>> {
        transport_not_configured()
    }

    fn create(&self, _draft: RepositoryDraft) -> BoxFuture<'_, CognitionResult<Repository>> {
        transport_not_configured()
    }

    fn update(&self, _patch: RepositoryPatch) -> BoxFuture<'_, CognitionResult<Repository>> {
        transport_not_configured()
    }

    fn delete(&self, _repo: Repo) -> BoxFuture<'_, CognitionResult<()>> {
        transport_not_configured()
    }

    fn branches(&self, _repo: Repo) -> BoxFuture<'_, CognitionResult<Page<Branch>>> {
        transport_not_configured()
    }

    fn create_branch(&self, _draft: BranchDraft) -> BoxFuture<'_, CognitionResult<Branch>> {
        transport_not_configured()
    }

    fn delete_branch(
        &self,
        _repo: Repo,
        _branch_name: String,
    ) -> BoxFuture<'_, CognitionResult<()>> {
        transport_not_configured()
    }

    fn commits(&self, _repo: Repo) -> BoxFuture<'_, CognitionResult<Page<Commit>>> {
        transport_not_configured()
    }
}
