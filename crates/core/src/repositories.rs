use std::future::Future;
use std::pin::Pin;

use serde::{Deserialize, Serialize};

use crate::{ProviderId, VcsResult};

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
pub struct RepositoryCoordinates {
    owner: OwnerName,
    name: RepositoryName,
}

impl RepositoryCoordinates {
    pub fn make() -> RepositoryCoordinatesBuilder {
        RepositoryCoordinatesBuilder::default()
    }

    pub fn owner(&self) -> &OwnerName {
        &self.owner
    }

    pub fn name(&self) -> &RepositoryName {
        &self.name
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RepositoryCoordinatesBuilder {
    owner: Option<OwnerName>,
    name: Option<RepositoryName>,
}

impl RepositoryCoordinatesBuilder {
    pub fn owner_name(mut self, owner: impl Into<String>) -> Self {
        self.owner = Some(OwnerName::make(owner));
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(RepositoryName::make(name));
        self
    }

    pub fn build(self) -> VcsResult<RepositoryCoordinates> {
        let Some(owner) = self.owner else {
            return Err(crate::VcsError::InvalidInput(
                "owner name is required".into(),
            ));
        };

        let Some(name) = self.name else {
            return Err(crate::VcsError::InvalidInput(
                "repository name is required".into(),
            ));
        };

        Ok(RepositoryCoordinates { owner, name })
    }
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
    coordinates: RepositoryCoordinates,
    visibility: Visibility,
    lifecycle_state: LifecycleState,
}

impl Repository {
    pub fn make(
        provider: ProviderId,
        coordinates: RepositoryCoordinates,
        visibility: Visibility,
        lifecycle_state: LifecycleState,
    ) -> Self {
        Self {
            provider,
            coordinates,
            visibility,
            lifecycle_state,
        }
    }

    pub fn provider(&self) -> &ProviderId {
        &self.provider
    }

    pub fn coordinates(&self) -> &RepositoryCoordinates {
        &self.coordinates
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

pub trait Repositories: Send + Sync {
    fn get(&self, coordinates: RepositoryCoordinates) -> BoxFuture<'_, VcsResult<Repository>>;

    fn list(&self, query: RepositoryListQuery) -> BoxFuture<'_, VcsResult<Page<Repository>>>;

    fn search(&self, query: RepositorySearchQuery) -> BoxFuture<'_, VcsResult<Page<Repository>>>;

    fn branches(
        &self,
        coordinates: RepositoryCoordinates,
    ) -> BoxFuture<'_, VcsResult<Page<Branch>>>;

    fn commits(&self, coordinates: RepositoryCoordinates)
    -> BoxFuture<'_, VcsResult<Page<Commit>>>;
}

#[cfg(test)]
mod tests {
    use crate::{
        LifecycleState, ProviderId, Repository, RepositoryCoordinates, VcsResult, Visibility,
    };

    #[test]
    fn repository_resource_is_provider_neutral() -> VcsResult<()> {
        let coordinates = RepositoryCoordinates::make()
            .owner_name("akira-io")
            .name("core")
            .build()?;
        let repository = Repository::make(
            ProviderId::make("github"),
            coordinates,
            Visibility::Public,
            LifecycleState::Active,
        );

        assert_eq!(repository.provider().as_str(), "github");
        assert_eq!(repository.coordinates().owner().as_str(), "akira-io");
        assert_eq!(repository.coordinates().name().as_str(), "core");
        assert_eq!(repository.visibility(), &Visibility::Public);
        assert_eq!(repository.lifecycle_state(), &LifecycleState::Active);

        Ok(())
    }
}
