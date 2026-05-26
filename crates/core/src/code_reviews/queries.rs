use serde::{Deserialize, Serialize};

use crate::{PageRequest, Repo};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CodeReviewQueryBuilder;

impl CodeReviewQueryBuilder {
    pub fn location(self, repo: impl Into<Repo>) -> CodeReviewListQueryBuilder {
        CodeReviewListQueryBuilder {
            repo: repo.into(),
            page: None,
        }
    }

    pub fn list(self, repo: impl Into<Repo>, page: Option<PageRequest>) -> CodeReviewListQuery {
        self.location(repo).optional_pagination(page).list()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodeReviewListQueryBuilder {
    repo: Repo,
    page: Option<PageRequest>,
}

impl CodeReviewListQueryBuilder {
    pub fn pagination(mut self, page: PageRequest) -> Self {
        self.page = Some(page);
        self
    }

    pub fn optional_pagination(mut self, page: Option<PageRequest>) -> Self {
        self.page = page;
        self
    }

    pub fn list(self) -> CodeReviewListQuery {
        CodeReviewListQuery {
            repo: self.repo,
            page: self.page,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CodeReviewListQuery {
    repo: Repo,
    page: Option<PageRequest>,
}

impl CodeReviewListQuery {
    pub fn repo(&self) -> &Repo {
        &self.repo
    }

    pub fn page(&self) -> Option<&PageRequest> {
        self.page.as_ref()
    }
}
