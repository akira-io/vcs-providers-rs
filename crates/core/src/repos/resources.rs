use serde::{Deserialize, Serialize};

use crate::ProviderId;
use crate::repos::Repo;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingVisibility;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvidedVisibility(pub(crate) Visibility);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingLifecycleState;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvidedLifecycleState(pub(crate) LifecycleState);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvidedProviderId(pub(crate) ProviderId);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepositoryBuilder<VisibilityState, LifecycleStateState> {
    pub(crate) repo: Repo,
    pub(crate) provider: ProvidedProviderId,
    pub(crate) visibility: VisibilityState,
    pub(crate) lifecycle_state: LifecycleStateState,
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
