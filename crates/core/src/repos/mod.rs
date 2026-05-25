use std::future::Future;
use std::pin::Pin;

mod contracts;
mod names;
mod queries;
mod resources;

pub use contracts::{Repos, TransportNotConfiguredRepos};
pub use names::{
    MissingOwnerName, MissingRepositoryName, OwnerName, ProvidedOwnerName, ProvidedRepositoryName,
    Repo, RepoBuilder, RepositoryName,
};
pub use queries::{RepoQueryBuilder, RepositoryListQuery, RepositorySearchQuery};
pub use resources::{
    Branch, Commit, LifecycleState, MissingLifecycleState, MissingVisibility,
    ProvidedLifecycleState, ProvidedProviderId, ProvidedVisibility, Repository, RepositoryBuilder,
    RepositoryDraft, RepositoryDraftBuilder, RepositoryPatch, RepositoryPatchBuilder, Visibility,
};

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
