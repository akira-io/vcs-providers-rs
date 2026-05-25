use serde::{Deserialize, Serialize};

use crate::PageRequest;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RepoQueryBuilder;

impl RepoQueryBuilder {
    pub fn list(self, page: Option<PageRequest>) -> RepositoryListQuery {
        RepositoryListQuery::make(page)
    }

    pub fn search(
        self,
        text: impl Into<String>,
        page: Option<PageRequest>,
    ) -> RepositorySearchQuery {
        RepositorySearchQuery::make(text, page)
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct RepositoryListQuery {
    page: Option<PageRequest>,
}

impl RepositoryListQuery {
    pub fn make(page: Option<PageRequest>) -> Self {
        Self { page }
    }

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
    pub fn make(text: impl Into<String>, page: Option<PageRequest>) -> Self {
        Self {
            text: text.into(),
            page,
        }
    }

    pub fn with_page(text: impl Into<String>, page: PageRequest) -> Self {
        Self::make(text, Some(page))
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn page(&self) -> Option<&PageRequest> {
        self.page.as_ref()
    }
}
