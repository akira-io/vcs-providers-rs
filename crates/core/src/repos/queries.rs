use serde::{Deserialize, Serialize};

use crate::PageRequest;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RepoQueryBuilder;

impl RepoQueryBuilder {
    pub fn list(self, page: Option<PageRequest>) -> RepositoryListQuery {
        self.optional_pagination(page).list()
    }

    pub fn pagination(self, page: PageRequest) -> RepositoryListQueryBuilder {
        RepositoryListQueryBuilder { page: Some(page) }
    }

    pub fn optional_pagination(self, page: Option<PageRequest>) -> RepositoryListQueryBuilder {
        RepositoryListQueryBuilder { page }
    }

    pub fn search(self, text: impl Into<String>) -> RepositorySearchQueryBuilder {
        RepositorySearchQueryBuilder {
            text: text.into(),
            page: None,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RepositoryListQueryBuilder {
    page: Option<PageRequest>,
}

impl RepositoryListQueryBuilder {
    pub fn list(self) -> RepositoryListQuery {
        RepositoryListQuery { page: self.page }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepositorySearchQueryBuilder {
    text: String,
    page: Option<PageRequest>,
}

impl RepositorySearchQueryBuilder {
    pub fn pagination(mut self, page: PageRequest) -> Self {
        self.page = Some(page);
        self
    }

    pub fn optional_pagination(mut self, page: Option<PageRequest>) -> Self {
        self.page = page;
        self
    }

    pub fn search(self) -> RepositorySearchQuery {
        RepositorySearchQuery {
            text: self.text,
            page: self.page,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct RepositoryListQuery {
    page: Option<PageRequest>,
}

impl RepositoryListQuery {
    pub fn page(&self) -> Option<&PageRequest> {
        self.page.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RepositorySearchQuery {
    text: String,
    page: Option<PageRequest>,
}

impl RepositorySearchQuery {
    pub fn with_page(text: impl Into<String>, page: PageRequest) -> Self {
        RepoQueryBuilder.search(text).pagination(page).search()
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn page(&self) -> Option<&PageRequest> {
        self.page.as_ref()
    }
}
