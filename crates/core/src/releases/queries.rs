use serde::{Deserialize, Serialize};

use crate::{PageRequest, Repo};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ReleaseQueryBuilder;

impl ReleaseQueryBuilder {
    pub fn location(self, repo: impl Into<Repo>) -> ReleaseListQueryBuilder {
        ReleaseListQueryBuilder {
            repo: repo.into(),
            page: None,
        }
    }

    pub fn list(self, repo: impl Into<Repo>, page: Option<PageRequest>) -> ReleaseListQuery {
        self.location(repo).optional_pagination(page).list()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReleaseListQueryBuilder {
    repo: Repo,
    page: Option<PageRequest>,
}

impl ReleaseListQueryBuilder {
    pub fn pagination(mut self, page: PageRequest) -> Self {
        self.page = Some(page);
        self
    }

    pub fn optional_pagination(mut self, page: Option<PageRequest>) -> Self {
        self.page = page;
        self
    }

    pub fn list(self) -> ReleaseListQuery {
        ReleaseListQuery {
            repo: self.repo,
            page: self.page,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ReleaseListQuery {
    repo: Repo,
    page: Option<PageRequest>,
}

impl ReleaseListQuery {
    pub fn repo(&self) -> &Repo {
        &self.repo
    }

    pub fn page(&self) -> Option<&PageRequest> {
        self.page.as_ref()
    }
}
