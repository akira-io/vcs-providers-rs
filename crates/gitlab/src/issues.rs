use serde::Serialize;
use vcs_provider_core::{
    Issue, IssueDraft, IssueListQuery, IssuePatch, Request, RequestBody, RequestUrl, request, url,
};

use crate::{DEFAULT_BASE_URL, request_pagination::apply_page};

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

fn issue_draft_body(draft: &IssueDraft) -> RequestBody {
    json_body(&GitLabIssueDraftBody {
        title: draft.title(),
        description: draft.body(),
    })
}

fn issue_patch_body(patch: &IssuePatch) -> RequestBody {
    json_body(&GitLabIssuePatchBody {
        title: patch.title(),
        description: patch.body(),
        state_event: patch.closed().map(gitlab_issue_state_event),
    })
}

fn gitlab_issue_state_event(closed: bool) -> &'static str {
    match closed {
        true => "close",
        false => "reopen",
    }
}

#[derive(Serialize)]
struct GitLabIssueDraftBody<'a> {
    title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
}

#[derive(Serialize)]
struct GitLabIssuePatchBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state_event: Option<&'static str>,
}

fn json_body(payload: &impl Serialize) -> RequestBody {
    match serde_json::to_string(payload) {
        Ok(body) => RequestBody::make(body),
        Err(_) => RequestBody::make("{}"),
    }
}
