use serde::Serialize;
use vcs_provider_core::{
    CodeReview, CodeReviewDraft, CodeReviewListQuery, CodeReviewPatch, Request, RequestBody,
    RequestUrl, request, url,
};

use crate::{DEFAULT_BASE_URL, request_pagination::apply_page};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitLabCodeReview {
    base_url: String,
    code_review: CodeReview,
}

impl GitLabCodeReview {
    pub fn make(base_url: impl Into<String>, code_review: CodeReview) -> Self {
        Self {
            base_url: base_url.into(),
            code_review,
        }
    }

    pub fn url(&self) -> RequestUrl {
        let project_path = project_path(self.code_review.repo());

        url(&self.base_url)
            .path_segments([
                "api",
                "v4",
                "projects",
                project_path.as_str(),
                "merge_requests",
                self.code_review.id().as_str(),
            ])
            .build()
    }

    pub fn update(&self, patch: &CodeReviewPatch) -> Request {
        request()
            .put(self.url().value())
            .body(code_review_patch_body(patch))
            .build()
    }

    pub fn delete(&self) -> Request {
        request().delete(self.url().value()).build()
    }

    pub fn merge(&self) -> Request {
        let project_path = project_path(self.code_review.repo());

        request()
            .put(
                url(&self.base_url)
                    .path_segments([
                        "api",
                        "v4",
                        "projects",
                        project_path.as_str(),
                        "merge_requests",
                        self.code_review.id().as_str(),
                        "merge",
                    ])
                    .build()
                    .value(),
            )
            .build()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitLabCodeReviewCollection {
    base_url: String,
}

impl GitLabCodeReviewCollection {
    pub fn make(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }

    pub fn list(&self, query: &CodeReviewListQuery) -> RequestUrl {
        let project_path = project_path(query.repo());

        apply_page(
            url(&self.base_url).path_segments([
                "api",
                "v4",
                "projects",
                project_path.as_str(),
                "merge_requests",
            ]),
            query.page(),
        )
        .build()
    }

    pub fn create(&self, draft: &CodeReviewDraft) -> Request {
        let project_path = project_path(draft.repo());

        request()
            .post(
                url(&self.base_url)
                    .path_segments([
                        "api",
                        "v4",
                        "projects",
                        project_path.as_str(),
                        "merge_requests",
                    ])
                    .build()
                    .value(),
            )
            .body(code_review_draft_body(draft))
            .build()
    }
}

impl Default for GitLabCodeReviewCollection {
    fn default() -> Self {
        Self::make(DEFAULT_BASE_URL)
    }
}

fn project_path(repo: &vcs_provider_core::Repo) -> String {
    format!("{}/{}", repo.owner().as_str(), repo.name().as_str())
}

fn code_review_draft_body(draft: &CodeReviewDraft) -> RequestBody {
    json_body(&GitLabCodeReviewDraftBody {
        title: draft.title(),
        source_branch: draft.source(),
        target_branch: draft.target(),
        description: draft.body(),
    })
}

fn code_review_patch_body(patch: &CodeReviewPatch) -> RequestBody {
    json_body(&GitLabCodeReviewPatchBody {
        title: patch.title(),
        description: patch.body(),
        state_event: patch.closed().map(gitlab_code_review_state_event),
    })
}

fn gitlab_code_review_state_event(closed: bool) -> &'static str {
    match closed {
        true => "close",
        false => "reopen",
    }
}

#[derive(Serialize)]
struct GitLabCodeReviewDraftBody<'a> {
    title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    source_branch: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    target_branch: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
}

#[derive(Serialize)]
struct GitLabCodeReviewPatchBody<'a> {
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
