use serde::{Deserialize, Serialize};

use crate::{BoxFuture, Page, PageRequest, Repo, VcsResult, transport_not_configured};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReleaseId(String);

impl ReleaseId {
    pub fn make(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Release {
    repo: Repo,
    id: ReleaseId,
}

impl Release {
    pub fn builder() -> ReleaseBuilder<MissingReleaseRepo, MissingReleaseId> {
        ReleaseBuilder {
            repo: MissingReleaseRepo,
            id: MissingReleaseId,
        }
    }

    pub fn make(repo: Repo, id: ReleaseId) -> Self {
        Self { repo, id }
    }

    pub fn repo(&self) -> &Repo {
        &self.repo
    }

    pub fn id(&self) -> &ReleaseId {
        &self.id
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingReleaseRepo;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvidedReleaseRepo {
    repo: Repo,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingReleaseId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvidedReleaseId {
    id: ReleaseId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseBuilder<RepoState, ReleaseIdState> {
    repo: RepoState,
    id: ReleaseIdState,
}

impl<ReleaseIdState> ReleaseBuilder<MissingReleaseRepo, ReleaseIdState> {
    pub fn repo(self, repo: Repo) -> ReleaseBuilder<ProvidedReleaseRepo, ReleaseIdState> {
        ReleaseBuilder {
            repo: ProvidedReleaseRepo { repo },
            id: self.id,
        }
    }
}

impl<RepoState> ReleaseBuilder<RepoState, MissingReleaseId> {
    pub fn id(self, id: impl Into<String>) -> ReleaseBuilder<RepoState, ProvidedReleaseId> {
        ReleaseBuilder {
            repo: self.repo,
            id: ProvidedReleaseId {
                id: ReleaseId::make(id),
            },
        }
    }
}

impl ReleaseBuilder<ProvidedReleaseRepo, ProvidedReleaseId> {
    pub fn build(self) -> Release {
        Release {
            repo: self.repo.repo,
            id: self.id.id,
        }
    }
}

pub trait Releases: Send + Sync {
    fn get(&self, repo: Repo, id: ReleaseId) -> BoxFuture<'_, VcsResult<Release>>;

    fn list(
        &self,
        repo: Repo,
        page: Option<PageRequest>,
    ) -> BoxFuture<'_, VcsResult<Page<Release>>>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TransportNotConfiguredReleases;

impl Releases for TransportNotConfiguredReleases {
    fn get(&self, _repo: Repo, _id: ReleaseId) -> BoxFuture<'_, VcsResult<Release>> {
        transport_not_configured()
    }

    fn list(
        &self,
        _repo: Repo,
        _page: Option<PageRequest>,
    ) -> BoxFuture<'_, VcsResult<Page<Release>>> {
        transport_not_configured()
    }
}
