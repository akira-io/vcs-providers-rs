use vcs_provider_core::{
    PageRequest, Release, ReleaseDraft, ReleaseListQuery, ReleasePatch, Request, RequestBody,
    RequestUrl, RequestUrlBuilder, request, url,
};

use crate::DEFAULT_BASE_URL;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitHubRelease {
    base_url: String,
    release: Release,
}

impl GitHubRelease {
    pub fn make(base_url: impl Into<String>, release: Release) -> Self {
        Self {
            base_url: base_url.into(),
            release,
        }
    }

    pub fn url(&self) -> RequestUrl {
        url(&self.base_url)
            .path_segments([
                "repos",
                self.release.repo().owner().as_str(),
                self.release.repo().name().as_str(),
                "releases",
                self.release.id().as_str(),
            ])
            .build()
    }

    pub fn update(&self, patch: &ReleasePatch) -> Request {
        request()
            .patch(self.url().value())
            .body(release_patch_body(patch))
            .build()
    }

    pub fn delete(&self) -> Request {
        request().delete(self.url().value()).build()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitHubReleaseCollection {
    base_url: String,
}

impl GitHubReleaseCollection {
    pub fn make(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }

    pub fn list(&self, query: &ReleaseListQuery) -> RequestUrl {
        apply_page(
            url(&self.base_url).path_segments([
                "repos",
                query.repo().owner().as_str(),
                query.repo().name().as_str(),
                "releases",
            ]),
            query.page(),
        )
        .build()
    }

    pub fn create(&self, draft: &ReleaseDraft) -> Request {
        request()
            .post(
                url(&self.base_url)
                    .path_segments([
                        "repos",
                        draft.repo().owner().as_str(),
                        draft.repo().name().as_str(),
                        "releases",
                    ])
                    .build()
                    .value(),
            )
            .body(release_draft_body(draft))
            .build()
    }
}

impl Default for GitHubReleaseCollection {
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

fn release_draft_body(draft: &ReleaseDraft) -> RequestBody {
    RequestBody::make(format!("{{\"tag_name\":\"{}\"}}", draft.tag()))
}

fn release_patch_body(_patch: &ReleasePatch) -> RequestBody {
    RequestBody::make("{}")
}
