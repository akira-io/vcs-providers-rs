use serde::Serialize;
use vcs_provider_core::{
    BranchDraft, PageRequest, Repo, RepositoryDraft, RepositoryListQuery, RepositoryPatch,
    RepositorySearchQuery, Request, RequestBody, RequestUrl, Visibility, request, request_body,
    url,
};

use crate::{DEFAULT_BASE_URL, request_pagination::apply_page};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitLabRepo {
    base_url: String,
    repo: Repo,
}

impl GitLabRepo {
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
        self.request_url(["repository", "branches"], page)
    }

    pub fn commits(&self, page: Option<&PageRequest>) -> RequestUrl {
        self.request_url(["repository", "commits"], page)
    }

    pub fn create_branch(&self, draft: &BranchDraft) -> Request {
        request()
            .post(self.branches(None).value())
            .body(branch_draft_body(draft))
            .build()
    }

    pub fn delete_branch(&self, branch_name: &str) -> Request {
        request()
            .delete(
                self.request_url(["repository", "branches", branch_name], None)
                    .value(),
            )
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
        let project_path = format!(
            "{}/{}",
            self.repo.owner().as_str(),
            self.repo.name().as_str()
        );
        let mut path_segments = vec!["api", "v4", "projects", project_path.as_str()];

        path_segments.extend(suffix);
        apply_page(url(&self.base_url).path_segments(path_segments), page).build()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitLabRepoCollection {
    base_url: String,
}

impl GitLabRepoCollection {
    pub fn make(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }

    pub fn list(&self, query: &RepositoryListQuery) -> RequestUrl {
        apply_page(
            url(&self.base_url).path_segments(["api", "v4", "projects"]),
            query.page(),
        )
        .build()
    }

    pub fn search(&self, query: &RepositorySearchQuery) -> RequestUrl {
        apply_page(
            url(&self.base_url)
                .path_segments(["api", "v4", "projects"])
                .query_param("search", query.text()),
            query.page(),
        )
        .build()
    }

    pub fn create(&self, draft: &RepositoryDraft) -> Request {
        request()
            .post(
                url(&self.base_url)
                    .path_segments(["api", "v4", "projects"])
                    .build()
                    .value(),
            )
            .body(repository_draft_body(draft))
            .build()
    }
}

impl Default for GitLabRepoCollection {
    fn default() -> Self {
        Self::make(DEFAULT_BASE_URL)
    }
}

fn repository_draft_body(draft: &RepositoryDraft) -> RequestBody {
    json_body(&GitLabRepositoryDraftBody {
        name: draft.repo().name().as_str(),
        path: draft.repo().name().as_str(),
        namespace_path: draft.repo().owner().as_str(),
        visibility: gitlab_visibility(draft.visibility()),
        description: draft.description(),
    })
}

fn repository_patch_body(patch: &RepositoryPatch) -> RequestBody {
    json_body(&GitLabRepositoryPatchBody {
        visibility: patch.visibility().map(gitlab_visibility),
        description: patch.description(),
    })
}

fn branch_draft_body(draft: &BranchDraft) -> RequestBody {
    json_body(&GitLabBranchDraftBody {
        branch: draft.name(),
        reference: draft.sha(),
    })
}

fn gitlab_visibility(visibility: &Visibility) -> &'static str {
    match visibility {
        Visibility::Public => "public",
        Visibility::Private => "private",
        Visibility::Internal => "internal",
    }
}

#[derive(Serialize)]
struct GitLabRepositoryDraftBody<'a> {
    name: &'a str,
    path: &'a str,
    namespace_path: &'a str,
    visibility: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
}

#[derive(Serialize)]
struct GitLabRepositoryPatchBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    visibility: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
}

#[derive(Serialize)]
struct GitLabBranchDraftBody<'a> {
    branch: &'a str,
    #[serde(rename = "ref")]
    reference: &'a str,
}

fn json_body(payload: &impl Serialize) -> RequestBody {
    match serde_json::to_string(payload) {
        Ok(body) => request_body(body),
        Err(_) => request_body("{}"),
    }
}
