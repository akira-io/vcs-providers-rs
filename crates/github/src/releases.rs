use serde::Serialize;
use vcs_provider_core::{
    Release, ReleaseDraft, ReleaseListQuery, ReleasePatch, Request, RequestBody, RequestUrl,
    request, url,
};

use crate::{DEFAULT_BASE_URL, request_pagination::apply_page};

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

#[derive(Serialize)]
struct GitHubReleaseDraftBody<'a> {
    tag_name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<&'a str>,
}

#[derive(Serialize)]
struct GitHubReleasePatchBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<&'a str>,
}

fn release_draft_body(draft: &ReleaseDraft) -> RequestBody {
    json_body(&GitHubReleaseDraftBody {
        tag_name: draft.tag(),
        name: draft.name(),
        body: draft.body(),
    })
}

fn release_patch_body(patch: &ReleasePatch) -> RequestBody {
    json_body(&GitHubReleasePatchBody {
        name: patch.name(),
        body: patch.body(),
    })
}

fn json_body(payload: &impl Serialize) -> RequestBody {
    match serde_json::to_string(payload) {
        Ok(body) => RequestBody::make(body),
        Err(_) => RequestBody::make("{}"),
    }
}
