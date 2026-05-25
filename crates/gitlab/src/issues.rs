use vcs_provider_core::{
    Issue, IssueDraft, IssueListQuery, IssuePatch, PageRequest, Request, RequestBody, RequestUrl,
    RequestUrlBuilder, request, url,
};

use crate::DEFAULT_BASE_URL;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitLabIssue {
    base_url: String,
    issue: Issue,
}

impl GitLabIssue {
    pub fn make(base_url: impl Into<String>, issue: Issue) -> Self {
        Self {
            base_url: base_url.into(),
            issue,
        }
    }

    pub fn url(&self) -> RequestUrl {
        let project_path = project_path(self.issue.repo());

        url(&self.base_url)
            .path_segments([
                "api",
                "v4",
                "projects",
                project_path.as_str(),
                "issues",
                self.issue.id().as_str(),
            ])
            .build()
    }

    pub fn update(&self, patch: &IssuePatch) -> Request {
        request()
            .put(self.url().value())
            .body(issue_patch_body(patch))
            .build()
    }

    pub fn delete(&self) -> Request {
        request().delete(self.url().value()).build()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitLabIssueCollection {
    base_url: String,
}

impl GitLabIssueCollection {
    pub fn make(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }

    pub fn list(&self, query: &IssueListQuery) -> RequestUrl {
        let project_path = project_path(query.repo());

        apply_page(
            url(&self.base_url).path_segments([
                "api",
                "v4",
                "projects",
                project_path.as_str(),
                "issues",
            ]),
            query.page(),
        )
        .build()
    }

    pub fn create(&self, draft: &IssueDraft) -> Request {
        let project_path = project_path(draft.repo());

        request()
            .post(
                url(&self.base_url)
                    .path_segments(["api", "v4", "projects", project_path.as_str(), "issues"])
                    .build()
                    .value(),
            )
            .body(issue_draft_body(draft))
            .build()
    }
}

impl Default for GitLabIssueCollection {
    fn default() -> Self {
        Self::make(DEFAULT_BASE_URL)
    }
}

fn project_path(repo: &vcs_provider_core::Repo) -> String {
    format!("{}/{}", repo.owner().as_str(), repo.name().as_str())
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

fn issue_draft_body(draft: &IssueDraft) -> RequestBody {
    RequestBody::make(format!("{{\"title\":\"{}\"}}", draft.title()))
}

fn issue_patch_body(patch: &IssuePatch) -> RequestBody {
    match patch.closed() {
        Some(true) => RequestBody::make("{\"state_event\":\"close\"}"),
        Some(false) => RequestBody::make("{\"state_event\":\"reopen\"}"),
        None => RequestBody::make("{}"),
    }
}
