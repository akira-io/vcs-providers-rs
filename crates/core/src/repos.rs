use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::{ProviderId, VcsError, VcsResult};

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct OwnerName(String);

impl OwnerName {
    pub fn make(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RepositoryName(String);

impl RepositoryName {
    pub fn make(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Repo {
    owner: OwnerName,
    name: RepositoryName,
}

impl Repo {
    pub fn make() -> RepoBuilder<MissingOwnerName, MissingRepositoryName> {
        crate::repo()
    }

    pub fn owner(&self) -> &OwnerName {
        &self.owner
    }

    pub fn name(&self) -> &RepositoryName {
        &self.name
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingOwnerName;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvidedOwnerName {
    pub(crate) owner_name: OwnerName,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingRepositoryName;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvidedRepositoryName {
    pub(crate) repository_name: RepositoryName,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepoBuilder<OwnerNameState, RepositoryNameState> {
    pub(crate) owner_name: OwnerNameState,
    pub(crate) repository_name: RepositoryNameState,
}

impl<RepositoryNameState> RepoBuilder<MissingOwnerName, RepositoryNameState> {
    pub fn owner(
        self,
        owner_name: impl Into<String>,
    ) -> RepoBuilder<ProvidedOwnerName, RepositoryNameState> {
        RepoBuilder {
            owner_name: ProvidedOwnerName {
                owner_name: OwnerName::make(owner_name),
            },
            repository_name: self.repository_name,
        }
    }
}

impl<OwnerNameState> RepoBuilder<OwnerNameState, MissingRepositoryName> {
    pub fn name(
        self,
        repository_name: impl Into<String>,
    ) -> RepoBuilder<OwnerNameState, ProvidedRepositoryName> {
        RepoBuilder {
            owner_name: self.owner_name,
            repository_name: ProvidedRepositoryName {
                repository_name: RepositoryName::make(repository_name),
            },
        }
    }
}

impl RepoBuilder<ProvidedOwnerName, ProvidedRepositoryName> {
    pub fn build(self) -> Repo {
        Repo {
            owner: self.owner_name.owner_name,
            name: self.repository_name.repository_name,
        }
    }

    pub fn provider(
        self,
        provider: impl Into<String>,
    ) -> RepositoryBuilder<MissingVisibility, MissingLifecycleState> {
        RepositoryBuilder {
            repo: self.build(),
            provider: ProvidedProviderId(ProviderId::make(provider)),
            visibility: MissingVisibility,
            lifecycle_state: MissingLifecycleState,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingVisibility;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvidedVisibility(Visibility);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingLifecycleState;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvidedLifecycleState(LifecycleState);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvidedProviderId(ProviderId);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepositoryBuilder<VisibilityState, LifecycleStateState> {
    repo: Repo,
    provider: ProvidedProviderId,
    visibility: VisibilityState,
    lifecycle_state: LifecycleStateState,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Private,
    Internal,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum LifecycleState {
    Active,
    Archived,
    Disabled,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Repository {
    provider: ProviderId,
    repo: Repo,
    visibility: Visibility,
    lifecycle_state: LifecycleState,
}

impl<LifecycleStateState> RepositoryBuilder<MissingVisibility, LifecycleStateState> {
    pub fn visibility(
        self,
        visibility: Visibility,
    ) -> RepositoryBuilder<ProvidedVisibility, LifecycleStateState> {
        RepositoryBuilder {
            repo: self.repo,
            provider: self.provider,
            visibility: ProvidedVisibility(visibility),
            lifecycle_state: self.lifecycle_state,
        }
    }
}

impl<VisibilityState> RepositoryBuilder<VisibilityState, MissingLifecycleState> {
    pub fn lifecycle(
        self,
        lifecycle_state: LifecycleState,
    ) -> RepositoryBuilder<VisibilityState, ProvidedLifecycleState> {
        RepositoryBuilder {
            repo: self.repo,
            provider: self.provider,
            visibility: self.visibility,
            lifecycle_state: ProvidedLifecycleState(lifecycle_state),
        }
    }
}

impl RepositoryBuilder<ProvidedVisibility, ProvidedLifecycleState> {
    pub fn build(self) -> Repository {
        Repository {
            provider: self.provider.0,
            repo: self.repo,
            visibility: self.visibility.0,
            lifecycle_state: self.lifecycle_state.0,
        }
    }
}

impl Repository {
    pub fn provider(&self) -> &ProviderId {
        &self.provider
    }

    pub fn repo(&self) -> &Repo {
        &self.repo
    }

    pub fn visibility(&self) -> &Visibility {
        &self.visibility
    }

    pub fn lifecycle_state(&self) -> &LifecycleState {
        &self.lifecycle_state
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Branch {
    name: String,
}

impl Branch {
    pub fn make(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Commit {
    id: String,
}

impl Commit {
    pub fn make(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Page<T> {
    items: Vec<T>,
}

impl<T> Page<T> {
    pub fn make(items: Vec<T>) -> Self {
        Self { items }
    }

    pub fn items(&self) -> &[T] {
        &self.items
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct RepositoryListQuery;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RepositorySearchQuery {
    text: String,
}

impl RepositorySearchQuery {
    pub fn make(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

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

fn transport_not_configured<'a, T>() -> BoxFuture<'a, VcsResult<T>> {
    Box::pin(async { Err(VcsError::TransportNotConfigured) })
}
