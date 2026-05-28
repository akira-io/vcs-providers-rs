use crate::{
    BoxFuture, CognitionResult, Release, ReleaseListOperation, Releases, Repo,
    ScopedReleaseOperation, error,
};

pub trait ReleasesFluent {
    fn location(self, repo: Repo) -> ScopedReleaseOperation;

    fn list(self) -> ReleaseListOperation;

    fn create(self) -> ReleaseCreateOperation;

    fn update(self) -> ReleaseUpdateOperation;

    fn delete(self) -> ReleaseDeleteOperation;
}

impl ReleasesFluent for Box<dyn Releases> {
    fn location(self, repo: Repo) -> ScopedReleaseOperation {
        ScopedReleaseOperation::make(self, repo)
    }

    fn list(self) -> ReleaseListOperation {
        ReleaseListOperation::make(self)
    }

    fn create(self) -> ReleaseCreateOperation {
        ReleaseCreateOperation {
            releases: self,
            repo: None,
            tag: None,
            name: None,
            body: None,
        }
    }

    fn update(self) -> ReleaseUpdateOperation {
        ReleaseUpdateOperation {
            releases: self,
            repo: None,
            id: None,
            name: None,
            body: None,
        }
    }

    fn delete(self) -> ReleaseDeleteOperation {
        ReleaseDeleteOperation {
            releases: self,
            repo: None,
            id: None,
        }
    }
}

pub struct ReleaseCreateOperation {
    releases: Box<dyn Releases>,
    repo: Option<Repo>,
    tag: Option<String>,
    name: Option<String>,
    body: Option<String>,
}

impl ReleaseCreateOperation {
    pub fn location(mut self, repo: Repo) -> Self {
        self.repo = Some(repo);
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

    pub fn create(self) -> BoxFuture<'static, CognitionResult<Release>> {
        let Some(repo) = self.repo else {
            return Box::pin(async { Err(error().invalid_input("repository is required")) });
        };

        let Some(tag) = self.tag else {
            return Box::pin(async { Err(error().invalid_input("release tag is required")) });
        };

        let mut draft = crate::release().draft().repo(repo).tag(tag);

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
}

pub struct ReleaseUpdateOperation {
    releases: Box<dyn Releases>,
    repo: Option<Repo>,
    id: Option<String>,
    name: Option<String>,
    body: Option<String>,
}

impl ReleaseUpdateOperation {
    pub fn location(mut self, repo: Repo) -> Self {
        self.repo = Some(repo);
        self
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
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

    pub fn update(self) -> BoxFuture<'static, CognitionResult<Release>> {
        let Some(repo) = self.repo else {
            return Box::pin(async { Err(error().invalid_input("repository is required")) });
        };

        let Some(id) = self.id else {
            return Box::pin(async { Err(error().invalid_input("release id is required")) });
        };

        let release = crate::release().repo(repo).id(id).get();
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
}

pub struct ReleaseDeleteOperation {
    releases: Box<dyn Releases>,
    repo: Option<Repo>,
    id: Option<String>,
}

impl ReleaseDeleteOperation {
    pub fn location(mut self, repo: Repo) -> Self {
        self.repo = Some(repo);
        self
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn delete(self) -> BoxFuture<'static, CognitionResult<()>> {
        let Some(repo) = self.repo else {
            return Box::pin(async { Err(error().invalid_input("repository is required")) });
        };

        let Some(id) = self.id else {
            return Box::pin(async { Err(error().invalid_input("release id is required")) });
        };

        let releases = self.releases;
        let release = crate::release().repo(repo).id(id).get();

        Box::pin(async move { Releases::delete(&*releases, release).await })
    }
}
