use vcs_provider_core::{
    BoxFuture, Branch, Commit, Page, Repositories, Repository, RepositoryCoordinates,
    RepositoryListQuery, RepositorySearchQuery, VcsError,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct BitbucketRepositories;

impl Repositories for BitbucketRepositories {
    fn get(
        &self,
        _coordinates: RepositoryCoordinates,
    ) -> BoxFuture<'_, Result<Repository, VcsError>> {
        Box::pin(async { Err(VcsError::TransportNotConfigured) })
    }

    fn list(
        &self,
        _query: RepositoryListQuery,
    ) -> BoxFuture<'_, Result<Page<Repository>, VcsError>> {
        Box::pin(async { Err(VcsError::TransportNotConfigured) })
    }

    fn search(
        &self,
        _query: RepositorySearchQuery,
    ) -> BoxFuture<'_, Result<Page<Repository>, VcsError>> {
        Box::pin(async { Err(VcsError::TransportNotConfigured) })
    }

    fn branches(
        &self,
        _coordinates: RepositoryCoordinates,
    ) -> BoxFuture<'_, Result<Page<Branch>, VcsError>> {
        Box::pin(async { Err(VcsError::TransportNotConfigured) })
    }

    fn commits(
        &self,
        _coordinates: RepositoryCoordinates,
    ) -> BoxFuture<'_, Result<Page<Commit>, VcsError>> {
        Box::pin(async { Err(VcsError::TransportNotConfigured) })
    }
}
