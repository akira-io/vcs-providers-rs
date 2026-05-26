use serde::{Deserialize, Serialize};

use crate::{BoxFuture, Page, Repo, VcsResult, transport_not_configured};

#[path = "releases/drafts.rs"]
mod drafts;
#[path = "releases/list.rs"]
mod list;
#[path = "releases/operations.rs"]
mod operations;
#[path = "releases/patches.rs"]
mod patches;
#[path = "releases/queries.rs"]
mod queries;
#[path = "releases/scoped.rs"]
mod scoped;
#[path = "releases/transport.rs"]
mod transport;

pub use drafts::{MissingReleaseTag, ProvidedReleaseTag, ReleaseDraftBuilder};
pub use list::{ReleaseListOperation, ReleaseListPaginationOperation};
#[allow(unused_imports)]
pub use operations::{
    ReleaseCreateOperation, ReleaseDeleteOperation, ReleaseUpdateOperation, ReleasesFluent,
};
pub use patches::ReleasePatchBuilder;
#[allow(unused_imports)]
pub use queries::{ReleaseListQuery, ReleaseListQueryBuilder, ReleaseQueryBuilder};
pub use scoped::ScopedReleaseOperation;
pub use transport::{ReleaseResponseMapper, TransportBackedReleases};

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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReleaseDraft {
    repo: Repo,
    tag: String,
    name: Option<String>,
    body: Option<String>,
}

impl ReleaseDraft {
    pub fn repo(&self) -> &Repo {
        &self.repo
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn body(&self) -> Option<&str> {
        self.body.as_deref()
    }
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReleasePatch {
    release: Release,
    name: Option<String>,
    body: Option<String>,
}

impl ReleasePatch {
    pub fn release(&self) -> &Release {
        &self.release
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn body(&self) -> Option<&str> {
        self.body.as_deref()
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
    pub fn repo(
        self,
        repo: impl Into<Repo>,
    ) -> ReleaseBuilder<ProvidedReleaseRepo, ReleaseIdState> {
        ReleaseBuilder {
            repo: ProvidedReleaseRepo { repo: repo.into() },
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
        self.get()
    }

    pub fn get(self) -> Release {
        Release {
            repo: self.repo.repo,
            id: self.id.id,
        }
    }
}

impl ReleaseBuilder<MissingReleaseRepo, MissingReleaseId> {
    pub fn query(self) -> ReleaseQueryBuilder {
        ReleaseQueryBuilder
    }

    pub fn draft(self) -> ReleaseDraftBuilder<MissingReleaseRepo, MissingReleaseTag> {
        ReleaseDraftBuilder {
            repo: MissingReleaseRepo,
            tag: MissingReleaseTag,
            name: None,
            body: None,
        }
    }
}

pub trait Releases: Send + Sync {
    fn get(&self, repo: Repo, id: ReleaseId) -> BoxFuture<'_, VcsResult<Release>>;

    fn list(&self, query: ReleaseListQuery) -> BoxFuture<'_, VcsResult<Page<Release>>>;

    fn create(&self, draft: ReleaseDraft) -> BoxFuture<'_, VcsResult<Release>>;

    fn update(&self, patch: ReleasePatch) -> BoxFuture<'_, VcsResult<Release>>;

    fn delete(&self, release: Release) -> BoxFuture<'_, VcsResult<()>>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TransportNotConfiguredReleases;

impl Releases for TransportNotConfiguredReleases {
    fn get(&self, _repo: Repo, _id: ReleaseId) -> BoxFuture<'_, VcsResult<Release>> {
        transport_not_configured()
    }

    fn list(&self, _query: ReleaseListQuery) -> BoxFuture<'_, VcsResult<Page<Release>>> {
        transport_not_configured()
    }

    fn create(&self, _draft: ReleaseDraft) -> BoxFuture<'_, VcsResult<Release>> {
        transport_not_configured()
    }

    fn update(&self, _patch: ReleasePatch) -> BoxFuture<'_, VcsResult<Release>> {
        transport_not_configured()
    }

    fn delete(&self, _release: Release) -> BoxFuture<'_, VcsResult<()>> {
        transport_not_configured()
    }
}
