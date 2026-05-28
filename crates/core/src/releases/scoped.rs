use crate::{BoxFuture, CognitionResult, Page, Release, ReleaseId, Releases, Repo, error, release};

pub struct ScopedReleaseOperation {
    releases: Box<dyn Releases>,
    repo: Repo,
    id: Option<String>,
    tag: Option<String>,
    name: Option<String>,
    body: Option<String>,
}

impl ScopedReleaseOperation {
    pub fn make(releases: Box<dyn Releases>, repo: Repo) -> Self {
        Self {
            releases,
            repo,
            id: None,
            tag: None,
            name: None,
            body: None,
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn get(self) -> BoxFuture<'static, CognitionResult<Release>> {
        let Some(id) = self.id else {
            return Box::pin(async { Err(error().invalid_input("release id is required")) });
        };

        let releases = self.releases;
        let repo = self.repo;

        Box::pin(async move { Releases::get(&*releases, repo, ReleaseId::make(id)).await })
    }

    pub fn list(self) -> BoxFuture<'static, CognitionResult<Page<Release>>> {
        let releases = self.releases;
        let query = release().query().location(self.repo).list();

        Box::pin(async move { Releases::list(&*releases, query).await })
    }

    pub fn create(self) -> BoxFuture<'static, CognitionResult<Release>> {
        let Some(tag) = self.tag else {
            return Box::pin(async { Err(error().invalid_input("release tag is required")) });
        };

        let mut draft = release().draft().repo(self.repo).tag(tag);

        if let Some(name) = self.name {
            draft = draft.name(name);
        }

        if let Some(body) = self.body {
            draft = draft.body(body);
        }

        let releases = self.releases;
        let draft = draft.get();

        Box::pin(async move { Releases::create(&*releases, draft).await })
    }

    pub fn update(self) -> BoxFuture<'static, CognitionResult<Release>> {
        let Some(id) = self.id else {
            return Box::pin(async { Err(error().invalid_input("release id is required")) });
        };

        let release = crate::release().repo(self.repo).id(id).get();
        let mut patch = release.patch();

        if let Some(name) = self.name {
            patch = patch.name(name);
        }

        if let Some(body) = self.body {
            patch = patch.body(body);
        }

        let releases = self.releases;
        let patch = patch.get();

        Box::pin(async move { Releases::update(&*releases, patch).await })
    }

    pub fn delete(self) -> BoxFuture<'static, CognitionResult<()>> {
        let Some(id) = self.id else {
            return Box::pin(async { Err(error().invalid_input("release id is required")) });
        };

        let releases = self.releases;
        let release = crate::release().repo(self.repo).id(id).get();

        Box::pin(async move { Releases::delete(&*releases, release).await })
    }
}
