use std::future::Future;
use std::pin::Pin;

mod contracts;
mod names;
mod operations;
mod queries;
mod resources;
mod transport;

pub use contracts::{Repos, TransportNotConfiguredRepos};
pub use names::{
    MissingOwnerName, MissingRepositoryName, OwnerName, ProvidedOwnerName, ProvidedRepositoryName,
    Repo, RepoBuilder, RepositoryName,
};
pub use operations::{RepoBranchOperation, RepoCreateOperation, RepoUpdateOperation, ReposFluent};
pub use queries::{RepoQueryBuilder, RepositoryListQuery, RepositorySearchQuery};
pub use resources::{
    Branch, BranchDraft, BranchDraftBuilder, Commit, LifecycleState, MissingLifecycleState,
    MissingVisibility, ProvidedLifecycleState, ProvidedProviderId, ProvidedVisibility, Repository,
    RepositoryBuilder, RepositoryDraft, RepositoryDraftBuilder, RepositoryPatch,
    RepositoryPatchBuilder, Visibility,
};
pub use transport::{RepositoryResponseMapper, TransportBackedRepos};

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
