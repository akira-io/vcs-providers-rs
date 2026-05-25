use vcs_provider_core::{Issue, IssueListQuery, PageRequest, RequestUrl, RequestUrlBuilder, url};

use crate::DEFAULT_BASE_URL;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitHubIssue {
    base_url: String,
    issue: Issue,
}

impl GitHubIssue {
    pub fn make(base_url: impl Into<String>, issue: Issue) -> Self {
        Self {
            base_url: base_url.into(),
            issue,
        }
    }

    pub fn url(&self) -> RequestUrl {
        url(&self.base_url)
            .path_segments([
                "repos",
                self.issue.repo().owner().as_str(),
                self.issue.repo().name().as_str(),
                "issues",
                self.issue.id().as_str(),
            ])
            .build()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitHubIssueCollection {
    base_url: String,
}

impl GitHubIssueCollection {
    pub fn make(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }

    pub fn list(&self, query: &IssueListQuery) -> RequestUrl {
        apply_page(
            url(&self.base_url).path_segments([
                "repos",
                query.repo().owner().as_str(),
                query.repo().name().as_str(),
                "issues",
            ]),
            query.page(),
        )
        .build()
    }
}

impl Default for GitHubIssueCollection {
    fn default() -> Self {
        Self::make(DEFAULT_BASE_URL)
    }
}

fn apply_page(request_url: RequestUrlBuilder, page: Option<&PageRequest>) -> RequestUrlBuilder {
    match page {
        Some(page) => request_url
            .optional_query_param(
                "per_page",
                page.limit().map(|limit| limit.as_u16().to_string()),
            )
            .optional_query_param(
                "page",
                page.cursor().map(|cursor| cursor.as_str().to_owned()),
            ),
        None => request_url,
    }
}
