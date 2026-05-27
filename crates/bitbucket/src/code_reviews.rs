use serde::Serialize;
use vcs_provider_core::{
    CodeReview, CodeReviewDraft, CodeReviewListQuery, CodeReviewPatch, Request, RequestBody,
    RequestUrl, request, url,
};

use crate::{DEFAULT_BASE_URL, request_pagination::apply_page};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitbucketCodeReview {
    base_url: String,
    code_review: CodeReview,
}

impl BitbucketCodeReview {
    pub fn make(base_url: impl Into<String>, code_review: CodeReview) -> Self {
        Self {
            base_url: base_url.into(),
            code_review,
        }
    }

    pub fn url(&self) -> RequestUrl {
        url(&self.base_url)
            .path_segments([
                "repositories",
                self.code_review.repo().owner().as_str(),
                self.code_review.repo().name().as_str(),
                "pullrequests",
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

    pub fn merge(&self) -> Request {
        request()
            .post(
                url(&self.base_url)
                    .path_segments([
                        "repositories",
                        self.code_review.repo().owner().as_str(),
                        self.code_review.repo().name().as_str(),
                        "pullrequests",
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
            .post(
                url(&self.base_url)
                    .path_segments([
                        "repositories",
                        self.code_review.repo().owner().as_str(),
                        self.code_review.repo().name().as_str(),
                        "pullrequests",
                        self.code_review.id().as_str(),
                        "decline",
                    ])
                    .build()
                    .value(),
            )
            .build()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitbucketCodeReviewCollection {
    base_url: String,
}

impl BitbucketCodeReviewCollection {
    pub fn make(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }

    pub fn list(&self, query: &CodeReviewListQuery) -> RequestUrl {
        apply_page(
            url(&self.base_url).path_segments([
                "repositories",
                query.repo().owner().as_str(),
                query.repo().name().as_str(),
                "pullrequests",
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
                        "repositories",
                        draft.repo().owner().as_str(),
                        draft.repo().name().as_str(),
                        "pullrequests",
                    ])
                    .build()
                    .value(),
            )
            .body(code_review_draft_body(draft))
            .build()
    }
}

impl Default for BitbucketCodeReviewCollection {
    fn default() -> Self {
        Self::make(DEFAULT_BASE_URL)
    }
}

fn code_review_draft_body(draft: &CodeReviewDraft) -> RequestBody {
    json_body(&BitbucketCodeReviewDraftBody {
        title: draft.title(),
        source: draft.source().map(bitbucket_branch_body),
        destination: draft.target().map(bitbucket_branch_body),
        description: draft.body(),
    })
}

fn code_review_patch_body(patch: &CodeReviewPatch) -> RequestBody {
    json_body(&BitbucketCodeReviewPatchBody {
        title: patch.title(),
        description: patch.body(),
    })
}

fn bitbucket_branch_body(name: &str) -> BitbucketBranchBody<'_> {
    BitbucketBranchBody {
        branch: BitbucketBranchNameBody { name },
    }
}

#[derive(Serialize)]
struct BitbucketCodeReviewDraftBody<'a> {
    title: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    source: Option<BitbucketBranchBody<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    destination: Option<BitbucketBranchBody<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
}

#[derive(Serialize)]
struct BitbucketBranchBody<'a> {
    branch: BitbucketBranchNameBody<'a>,
}

#[derive(Serialize)]
struct BitbucketBranchNameBody<'a> {
    name: &'a str,
}

#[derive(Serialize)]
struct BitbucketCodeReviewPatchBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
}

fn json_body(payload: &impl Serialize) -> RequestBody {
    match serde_json::to_string(payload) {
        Ok(body) => RequestBody::make(body),
        Err(_) => RequestBody::make("{}"),
    }
}
