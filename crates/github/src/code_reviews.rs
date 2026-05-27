use serde::Serialize;
use vcs_provider_core::{
    CodeReview, CodeReviewDraft, CodeReviewListQuery, CodeReviewPatch, Request, RequestBody,
    RequestUrl, request, url,
};

use crate::{DEFAULT_BASE_URL, request_pagination::apply_page};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitHubCodeReview {
    base_url: String,
    code_review: CodeReview,
}

impl GitHubCodeReview {
    pub fn make(base_url: impl Into<String>, code_review: CodeReview) -> Self {
        Self {
            base_url: base_url.into(),
            code_review,
        }
    }

    pub fn url(&self) -> RequestUrl {
        url(&self.base_url)
            .path_segments([
                "repos",
                self.code_review.repo().owner().as_str(),
                self.code_review.repo().name().as_str(),
                "pulls",
                self.code_review.id().as_str(),
            ])
            .build()
    }

    pub fn update(&self, patch: &CodeReviewPatch) -> Request {
        request()
            .patch(self.url().value())
            .body(code_review_patch_body(patch))
            .build()
    }

    pub fn merge(&self) -> Request {
        request()
            .put(
                url(&self.base_url)
                    .path_segments([
                        "repos",
                        self.code_review.repo().owner().as_str(),
                        self.code_review.repo().name().as_str(),
                        "pulls",
                        self.code_review.id().as_str(),
                        "merge",
                    ])
                    .build()
                    .value(),
            )
            .build()
    }

    pub fn close(&self) -> Request {
        request()
            .patch(self.url().value())
            .body(RequestBody::make("{\"state\":\"closed\"}"))
            .build()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitHubCodeReviewCollection {
    base_url: String,
}

impl GitHubCodeReviewCollection {
    pub fn make(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }

    pub fn list(&self, query: &CodeReviewListQuery) -> RequestUrl {
        apply_page(
            url(&self.base_url).path_segments([
                "repos",
                query.repo().owner().as_str(),
                query.repo().name().as_str(),
                "pulls",
            ]),
            query.page(),
        )
        .build()
    }

    pub fn create(&self, draft: &CodeReviewDraft) -> Request {
        request()
            .post(
                url(&self.base_url)
                    .path_segments([
                        "repos",
                        draft.repo().owner().as_str(),
                        draft.repo().name().as_str(),
                        "pulls",
                    ])
                    .build()
                    .value(),
            )
            .body(code_review_draft_body(draft))
            .build()
    }
}

impl Default for GitHubCodeReviewCollection {
    fn default() -> Self {
        Self::make(DEFAULT_BASE_URL)
    }
}

fn code_review_draft_body(draft: &CodeReviewDraft) -> RequestBody {
    json_body(&GitHubCodeReviewDraftBody {
        title: draft.title(),
        head: draft.source(),
        base: draft.target(),
        body: draft.body(),
    })
}

fn code_review_patch_body(patch: &CodeReviewPatch) -> RequestBody {
    json_body(&GitHubCodeReviewPatchBody {
        title: patch.title(),
        body: patch.body(),
        state: patch.closed().map(github_code_review_state),
    })
}

fn github_code_review_state(closed: bool) -> &'static str {
    match closed {
        true => "closed",
        false => "open",
    }
}

#[derive(Serialize)]
struct GitHubCodeReviewDraftBody<'a> {
    title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    head: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    base: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<&'a str>,
}

#[derive(Serialize)]
struct GitHubCodeReviewPatchBody<'a> {
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
