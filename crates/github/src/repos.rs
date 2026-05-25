use vcs_provider_core::{
    PageRequest, Repo, RepositoryDraft, RepositoryListQuery, RepositoryPatch,
    RepositorySearchQuery, Request, RequestBody, RequestUrl, RequestUrlBuilder, request, url,
};

use crate::DEFAULT_BASE_URL;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitHubRepo {
    base_url: String,
    repo: Repo,
}

impl GitHubRepo {
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
        self.request_url(["branches"], page)
    }

    pub fn commits(&self, page: Option<&PageRequest>) -> RequestUrl {
        self.request_url(["commits"], page)
    }

    pub fn update(&self, patch: &RepositoryPatch) -> Request {
        request()
            .patch(self.url().value())
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
            "repos",
            self.repo.owner().as_str(),
            self.repo.name().as_str(),
        ];

        path_segments.extend(suffix);
        apply_page(url(&self.base_url).path_segments(path_segments), page).build()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitHubRepoCollection {
    base_url: String,
}

impl GitHubRepoCollection {
    pub fn make(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }

    pub fn list(&self, query: &RepositoryListQuery) -> RequestUrl {
        apply_page(
            url(&self.base_url).path_segments(["user", "repos"]),
            query.page(),
        )
        .build()
    }

    pub fn search(&self, query: &RepositorySearchQuery) -> RequestUrl {
        apply_page(
            url(&self.base_url)
                .path_segments(["search", "repositories"])
                .query_param("q", query.text()),
            query.page(),
        )
        .build()
    }

    pub fn create(&self, draft: &RepositoryDraft) -> Request {
        request()
            .post(
                url(&self.base_url)
                    .path_segments(["orgs", draft.repo().owner().as_str(), "repos"])
                    .build()
                    .value(),
            )
            .body(repository_draft_body(draft))
            .build()
    }
}

impl Default for GitHubRepoCollection {
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

fn repository_draft_body(draft: &RepositoryDraft) -> RequestBody {
    RequestBody::make(format!(
        "{{\"name\":\"{}\",\"private\":{}}}",
        draft.repo().name().as_str(),
        matches!(draft.visibility(), vcs_provider_core::Visibility::Private)
    ))
}

fn repository_patch_body(patch: &RepositoryPatch) -> RequestBody {
    let visibility = patch
        .visibility()
        .map(|visibility| matches!(visibility, vcs_provider_core::Visibility::Private));

    match visibility {
        Some(is_private) => RequestBody::make(format!("{{\"private\":{is_private}}}")),
        None => RequestBody::make("{}"),
    }
}
