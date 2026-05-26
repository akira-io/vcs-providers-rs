use crate::repos::{BoxFuture, Repo, Repos, Visibility};
use crate::{Repository, VcsResult};

pub trait ReposFluent {
    fn create(self) -> RepoCreateOperation;

    fn update(self) -> RepoUpdateOperation;
}

impl ReposFluent for Box<dyn Repos> {
    fn create(self) -> RepoCreateOperation {
        RepoCreateOperation {
            repos: self,
            repo: None,
            visibility: Visibility::Private,
            description: None,
        }
    }

    fn update(self) -> RepoUpdateOperation {
        RepoUpdateOperation {
            repos: self,
            repo: None,
            visibility: None,
            description: None,
        }
    }
}

pub struct RepoCreateOperation {
    repos: Box<dyn Repos>,
    repo: Option<Repo>,
    visibility: Visibility,
    description: Option<String>,
}

impl RepoCreateOperation {
    pub fn location(mut self, repo: Repo) -> Self {
        self.repo = Some(repo);
        self
    }

    pub fn repo(self, repo: Repo) -> Self {
        self.location(repo)
    }

    pub fn visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = visibility;
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn create(self) -> BoxFuture<'static, VcsResult<Repository>> {
        let Some(repo) = self.repo else {
            return Box::pin(async { Err(crate::error().invalid_input("repository is required")) });
        };

        let mut draft = repo.draft().visibility(self.visibility);

        if let Some(description) = self.description {
            draft = draft.description(description);
        }

        let repos = self.repos;
        let draft = draft.get();

        Box::pin(async move { Repos::create(&*repos, draft).await })
    }
}

pub struct RepoUpdateOperation {
    repos: Box<dyn Repos>,
    repo: Option<Repo>,
    visibility: Option<Visibility>,
    description: Option<String>,
}

impl RepoUpdateOperation {
    pub fn location(mut self, repo: Repo) -> Self {
        self.repo = Some(repo);
        self
    }

    pub fn repo(self, repo: Repo) -> Self {
        self.location(repo)
    }

    pub fn visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = Some(visibility);
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn update(self) -> BoxFuture<'static, VcsResult<Repository>> {
        let Some(repo) = self.repo else {
            return Box::pin(async { Err(crate::error().invalid_input("repository is required")) });
        };

        let mut patch = repo.patch();

        if let Some(visibility) = self.visibility {
            patch = patch.visibility(visibility);
        }

        if let Some(description) = self.description {
            patch = patch.description(description);
        }

        let repos = self.repos;
        let patch = patch.get();

        Box::pin(async move { Repos::update(&*repos, patch).await })
    }
}
