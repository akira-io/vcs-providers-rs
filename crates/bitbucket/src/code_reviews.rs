use vcs_provider_core::{
    CodeReview, CodeReviewDraft, CodeReviewListQuery, CodeReviewPatch, PageRequest, Request,
    RequestBody, RequestUrl, RequestUrlBuilder, request, url,
};

use crate::DEFAULT_BASE_URL;

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

fn apply_page(request_url: RequestUrlBuilder, page: Option<&PageRequest>) -> RequestUrlBuilder {
    match page {
        Some(page) => request_url
            .optional_query_param(
                "pagelen",
                page.limit().map(|limit| limit.as_u16().to_string()),
            )
            .optional_query_param(
                "page",
                page.cursor().map(|cursor| cursor.as_str().to_owned()),
            ),
        None => request_url,
    }
}

fn code_review_draft_body(draft: &CodeReviewDraft) -> RequestBody {
    RequestBody::make(format!("{{\"title\":\"{}\"}}", draft.title()))
}

fn code_review_patch_body(_patch: &CodeReviewPatch) -> RequestBody {
    RequestBody::make("{}")
}
