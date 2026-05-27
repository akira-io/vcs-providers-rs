use serde::Serialize;
use vcs_provider_core::{
    Issue, IssueDraft, IssueListQuery, IssuePatch, Request, RequestBody, RequestUrl, request, url,
};

use crate::{DEFAULT_BASE_URL, request_pagination::apply_page};

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

    pub fn update(&self, patch: &IssuePatch) -> Request {
        request()
            .patch(self.url().value())
            .body(issue_patch_body(patch))
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

    pub fn create(&self, draft: &IssueDraft) -> Request {
        request()
            .post(
                url(&self.base_url)
                    .path_segments([
                        "repos",
                        draft.repo().owner().as_str(),
                        draft.repo().name().as_str(),
                        "issues",
                    ])
                    .build()
                    .value(),
            )
            .body(issue_draft_body(draft))
            .build()
    }
}

impl Default for GitHubIssueCollection {
    fn default() -> Self {
        Self::make(DEFAULT_BASE_URL)
    }
}

fn issue_draft_body(draft: &IssueDraft) -> RequestBody {
    json_body(&GitHubIssueDraftBody {
        title: draft.title(),
        body: draft.body(),
    })
}

fn issue_patch_body(patch: &IssuePatch) -> RequestBody {
    json_body(&GitHubIssuePatchBody {
        title: patch.title(),
        body: patch.body(),
        state: patch.closed().map(github_issue_state),
    })
}

fn github_issue_state(closed: bool) -> &'static str {
    match closed {
        true => "closed",
        false => "open",
    }
}

#[derive(Serialize)]
struct GitHubIssueDraftBody<'a> {
    title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<&'a str>,
}

#[derive(Serialize)]
struct GitHubIssuePatchBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<&'static str>,
}

fn json_body(payload: &impl Serialize) -> RequestBody {
    match serde_json::to_string(payload) {
        Ok(body) => RequestBody::make(body),
        Err(_) => RequestBody::make("{}"),
    }
}
