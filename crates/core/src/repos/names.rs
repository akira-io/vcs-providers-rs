use serde::{Deserialize, Serialize};

use crate::repos::{
    MissingLifecycleState, MissingVisibility, ProvidedProviderId, RepoQueryBuilder,
    RepositoryBuilder,
};
use crate::{ProviderId, repo};

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
        repo()
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

impl RepoBuilder<MissingOwnerName, MissingRepositoryName> {
    pub fn query(self) -> RepoQueryBuilder {
        RepoQueryBuilder
    }
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
        self.get()
    }

    pub fn get(self) -> Repo {
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
            repo: self.get(),
            provider: ProvidedProviderId(ProviderId::make(provider)),
            visibility: MissingVisibility,
            lifecycle_state: MissingLifecycleState,
        }
    }
}
