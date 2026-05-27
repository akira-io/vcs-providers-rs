use serde::Serialize;
use vcs_provider_core::{
    PageRequest, Repo, RepositoryDraft, RepositoryListQuery, RepositoryPatch,
    RepositorySearchQuery, Request, RequestBody, RequestUrl, Visibility, request, url,
};

use crate::{DEFAULT_BASE_URL, request_pagination::apply_page};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitbucketRepo {
    base_url: String,
    repo: Repo,
}

impl BitbucketRepo {
    pub fn make(base_url: impl Into<String>, repo: Repo) -> Self {
        Self {
            base_url: base_url.into(),
            repo,
        }
    }

    pub fn url(&self) -> RequestUrl {
        self.request_url([], None)
    }

    pub fn branches(&self, page: Option<&PageRequest>) -> RequestUrl {
        self.request_url(["refs", "branches"], page)
    }

    pub fn commits(&self, page: Option<&PageRequest>) -> RequestUrl {
        self.request_url(["commits"], page)
    }

    pub fn create(&self, draft: &RepositoryDraft) -> Request {
        request()
            .put(self.url().value())
            .body(repository_draft_body(draft))
            .build()
    }

    pub fn update(&self, patch: &RepositoryPatch) -> Request {
        request()
            .put(self.url().value())
            .body(repository_patch_body(patch))
            .build()
    }

    pub fn delete(&self) -> Request {
        request().delete(self.url().value()).build()
    }

    fn request_url<const SIZE: usize>(
        &self,
        suffix: [&str; SIZE],
        page: Option<&PageRequest>,
    ) -> RequestUrl {
        let mut path_segments = vec![
            "repositories",
            self.repo.owner().as_str(),
            self.repo.name().as_str(),
        ];

        path_segments.extend(suffix);
        apply_page(url(&self.base_url).path_segments(path_segments), page).build()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BitbucketRepoCollection {
    base_url: String,
}

impl BitbucketRepoCollection {
    pub fn make(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }

    pub fn list(&self, query: &RepositoryListQuery) -> RequestUrl {
        apply_page(
            url(&self.base_url).path_segments(["repositories"]),
            query.page(),
        )
        .build()
    }

    pub fn search(&self, query: &RepositorySearchQuery) -> RequestUrl {
        apply_page(
            url(&self.base_url)
                .path_segments(["repositories"])
                .query_param("q", format!("name~\"{}\"", query.text())),
            query.page(),
        )
        .build()
    }
}

impl Default for BitbucketRepoCollection {
    fn default() -> Self {
        Self::make(DEFAULT_BASE_URL)
    }
}

fn repository_draft_body(draft: &RepositoryDraft) -> RequestBody {
    json_body(&BitbucketRepositoryDraftBody {
        scm: "git",
        is_private: bitbucket_private(draft.visibility()),
        description: draft.description(),
    })
}

fn repository_patch_body(patch: &RepositoryPatch) -> RequestBody {
    json_body(&BitbucketRepositoryPatchBody {
        is_private: patch.visibility().map(bitbucket_private),
        description: patch.description(),
    })
}

fn bitbucket_private(visibility: &Visibility) -> bool {
    matches!(visibility, Visibility::Private | Visibility::Internal)
}

#[derive(Serialize)]
struct BitbucketRepositoryDraftBody<'a> {
    scm: &'static str,
    is_private: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
}

#[derive(Serialize)]
struct BitbucketRepositoryPatchBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    is_private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
}

fn json_body(payload: &impl Serialize) -> RequestBody {
    match serde_json::to_string(payload) {
        Ok(body) => RequestBody::make(body),
        Err(_) => RequestBody::make("{}"),
    }
}
