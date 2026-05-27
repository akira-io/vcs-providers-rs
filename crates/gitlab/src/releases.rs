use serde::Serialize;
use vcs_provider_core::{
    Release, ReleaseDraft, ReleaseListQuery, ReleasePatch, Request, RequestBody, RequestUrl,
    request, url,
};

use crate::{DEFAULT_BASE_URL, request_pagination::apply_page};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitLabRelease {
    base_url: String,
    release: Release,
}

impl GitLabRelease {
    pub fn make(base_url: impl Into<String>, release: Release) -> Self {
        Self {
            base_url: base_url.into(),
            release,
        }
    }

    pub fn url(&self) -> RequestUrl {
        let project_path = project_path(self.release.repo());

        url(&self.base_url)
            .path_segments([
                "api",
                "v4",
                "projects",
                project_path.as_str(),
                "releases",
                self.release.id().as_str(),
            ])
            .build()
    }

    pub fn update(&self, patch: &ReleasePatch) -> Request {
        request()
            .put(self.url().value())
            .body(release_patch_body(patch))
            .build()
    }

    pub fn delete(&self) -> Request {
        request().delete(self.url().value()).build()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitLabReleaseCollection {
    base_url: String,
}

impl GitLabReleaseCollection {
    pub fn make(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }

    pub fn list(&self, query: &ReleaseListQuery) -> RequestUrl {
        let project_path = project_path(query.repo());

        apply_page(
            url(&self.base_url).path_segments([
                "api",
                "v4",
                "projects",
                project_path.as_str(),
                "releases",
            ]),
            query.page(),
        )
        .build()
    }

    pub fn create(&self, draft: &ReleaseDraft) -> Request {
        let project_path = project_path(draft.repo());

        request()
            .post(
                url(&self.base_url)
                    .path_segments(["api", "v4", "projects", project_path.as_str(), "releases"])
                    .build()
                    .value(),
            )
            .body(release_draft_body(draft))
            .build()
    }
}

impl Default for GitLabReleaseCollection {
    fn default() -> Self {
        Self::make(DEFAULT_BASE_URL)
    }
}

fn project_path(repo: &vcs_provider_core::Repo) -> String {
    format!("{}/{}", repo.owner().as_str(), repo.name().as_str())
}

#[derive(Serialize)]
struct GitLabReleaseDraftBody<'a> {
    tag_name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
}

#[derive(Serialize)]
struct GitLabReleasePatchBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
}

fn release_draft_body(draft: &ReleaseDraft) -> RequestBody {
    json_body(&GitLabReleaseDraftBody {
        tag_name: draft.tag(),
        name: draft.name(),
        description: draft.body(),
    })
}

fn release_patch_body(patch: &ReleasePatch) -> RequestBody {
    json_body(&GitLabReleasePatchBody {
        name: patch.name(),
        description: patch.body(),
    })
}

fn json_body(payload: &impl Serialize) -> RequestBody {
    match serde_json::to_string(payload) {
        Ok(body) => RequestBody::make(body),
        Err(_) => RequestBody::make("{}"),
    }
}
