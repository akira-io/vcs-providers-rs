use crate::{
    MissingReleaseId, MissingReleaseRepo, MissingReleaseTag, PageRequest, PageRequestBuilder,
    ProvidedReleaseId, ProvidedReleaseRepo, ProvidedReleaseTag, Release, ReleaseBuilder,
    ReleaseDraftBuilder, ReleaseListQuery, ReleaseQueryBuilder, Repo, Request, RequestUrl,
};

use super::{CognitionManager, ManagedReleaseProvider};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedReleaseBuilder<Driver, RepoState, ReleaseIdState> {
    pub(super) manager: CognitionManager<Driver>,
    pub(super) release: ReleaseBuilder<RepoState, ReleaseIdState>,
}

impl<Driver> ManagedReleaseBuilder<Driver, MissingReleaseRepo, MissingReleaseId>
where
    Driver: ManagedReleaseProvider,
{
    pub fn collection(&self) -> ManagedReleaseCollection<Driver> {
        ManagedReleaseCollection {
            manager: self.manager.clone(),
        }
    }

    pub fn query(&self) -> ReleaseQueryBuilder {
        ReleaseQueryBuilder
    }

    pub fn draft(
        &self,
    ) -> ManagedReleaseDraftBuilder<Driver, MissingReleaseRepo, MissingReleaseTag> {
        ManagedReleaseDraftBuilder {
            manager: self.manager.clone(),
            draft: crate::release().draft(),
        }
    }
}

impl<Driver, ReleaseIdState> ManagedReleaseBuilder<Driver, MissingReleaseRepo, ReleaseIdState>
where
    Driver: ManagedReleaseProvider,
{
    pub fn repo(
        self,
        repo: impl Into<Repo>,
    ) -> ManagedReleaseBuilder<Driver, ProvidedReleaseRepo, ReleaseIdState> {
        ManagedReleaseBuilder {
            manager: self.manager,
            release: self.release.repo(repo),
        }
    }
}

impl<Driver, RepoState> ManagedReleaseBuilder<Driver, RepoState, MissingReleaseId>
where
    Driver: ManagedReleaseProvider,
{
    pub fn id(
        self,
        id: impl Into<String>,
    ) -> ManagedReleaseBuilder<Driver, RepoState, ProvidedReleaseId> {
        ManagedReleaseBuilder {
            manager: self.manager,
            release: self.release.id(id),
        }
    }
}

impl<Driver> ManagedReleaseBuilder<Driver, ProvidedReleaseRepo, ProvidedReleaseId>
where
    Driver: ManagedReleaseProvider,
{
    pub fn build(self) -> ManagedRelease<Driver> {
        self.get()
    }

    pub fn get(self) -> ManagedRelease<Driver> {
        ManagedRelease {
            manager: self.manager,
            release: self.release.build(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedReleaseDraftBuilder<Driver, RepoState, TagState> {
    manager: CognitionManager<Driver>,
    draft: ReleaseDraftBuilder<RepoState, TagState>,
}

impl<Driver, TagState> ManagedReleaseDraftBuilder<Driver, MissingReleaseRepo, TagState>
where
    Driver: ManagedReleaseProvider,
{
    pub fn repo(
        self,
        repo: impl Into<Repo>,
    ) -> ManagedReleaseDraftBuilder<Driver, ProvidedReleaseRepo, TagState> {
        ManagedReleaseDraftBuilder {
            manager: self.manager,
            draft: self.draft.repo(repo),
        }
    }
}

impl<Driver, RepoState> ManagedReleaseDraftBuilder<Driver, RepoState, MissingReleaseTag>
where
    Driver: ManagedReleaseProvider,
{
    pub fn tag(
        self,
        tag: impl Into<String>,
    ) -> ManagedReleaseDraftBuilder<Driver, RepoState, ProvidedReleaseTag> {
        ManagedReleaseDraftBuilder {
            manager: self.manager,
            draft: self.draft.tag(tag),
        }
    }
}

impl<Driver, RepoState, TagState> ManagedReleaseDraftBuilder<Driver, RepoState, TagState>
where
    Driver: ManagedReleaseProvider,
{
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.draft = self.draft.name(name);
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.draft = self.draft.body(body);
        self
    }
}

impl<Driver> ManagedReleaseDraftBuilder<Driver, ProvidedReleaseRepo, ProvidedReleaseTag>
where
    Driver: ManagedReleaseProvider,
{
    pub fn create(self) -> Request {
        self.manager
            .driver
            .release_create_request(&self.draft.get())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedRelease<Driver> {
    pub(super) manager: CognitionManager<Driver>,
    pub(super) release: Release,
}

impl<Driver> ManagedRelease<Driver>
where
    Driver: ManagedReleaseProvider,
{
    pub fn url(&self) -> RequestUrl {
        self.manager.driver.release_url(&self.release)
    }

    pub fn release(&self) -> &Release {
        &self.release
    }

    pub fn repo(&self) -> &Repo {
        self.release.repo()
    }

    pub fn delete(&self) -> Request {
        self.manager.driver.release_delete_request(&self.release)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedReleaseCollection<Driver> {
    manager: CognitionManager<Driver>,
}

impl<Driver> ManagedReleaseCollection<Driver>
where
    Driver: ManagedReleaseProvider,
{
    pub fn list(&self, query: &ReleaseListQuery) -> RequestUrl {
        self.manager.driver.release_list_url(query)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedRepoReleases<Driver> {
    pub(super) manager: CognitionManager<Driver>,
    pub(super) repo: Repo,
    pub(super) page: Option<PageRequest>,
}

impl<Driver> ManagedRepoReleases<Driver>
where
    Driver: ManagedReleaseProvider,
{
    pub fn url(&self) -> RequestUrl {
        let query = self.query();
        self.manager.driver.release_list_url(&query)
    }

    pub fn pagination(self) -> ManagedRepoReleasesPagination<Driver> {
        ManagedRepoReleasesPagination {
            manager: self.manager,
            repo: self.repo,
            page: PageRequestBuilder::default(),
        }
    }

    pub fn repo(&self) -> &Repo {
        &self.repo
    }

    fn query(&self) -> ReleaseListQuery {
        ReleaseQueryBuilder.list(self.repo.clone(), self.page.clone())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedRepoReleasesPagination<Driver> {
    manager: CognitionManager<Driver>,
    repo: Repo,
    page: PageRequestBuilder,
}

impl<Driver> ManagedRepoReleasesPagination<Driver>
where
    Driver: ManagedReleaseProvider,
{
    pub fn limit(mut self, limit: u16) -> Self {
        self.page = self.page.limit(limit);
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.page = self.page.cursor(cursor);
        self
    }

    pub fn build(self) -> ManagedRepoReleases<Driver> {
        self.list()
    }

    pub fn get(self) -> ManagedRepoReleases<Driver> {
        self.list()
    }

    pub fn list(self) -> ManagedRepoReleases<Driver> {
        ManagedRepoReleases {
            manager: self.manager,
            repo: self.repo,
            page: Some(self.page.build()),
        }
    }

    pub fn url(self) -> RequestUrl {
        self.list().url()
    }
}
