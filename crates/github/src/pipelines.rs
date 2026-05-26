use vcs_provider_core::{
    PageRequest, Pipeline, PipelineListQuery, Request, RequestUrl, RequestUrlBuilder, request, url,
};

use crate::DEFAULT_BASE_URL;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitHubPipeline {
    base_url: String,
    pipeline: Pipeline,
}

impl GitHubPipeline {
    pub fn make(base_url: impl Into<String>, pipeline: Pipeline) -> Self {
        Self {
            base_url: base_url.into(),
            pipeline,
        }
    }

    pub fn url(&self) -> RequestUrl {
        self.request_url([]).build()
    }

    pub fn rerun(&self) -> Request {
        request()
            .post(self.request_url(["rerun"]).build().value())
            .build()
    }

    pub fn cancel(&self) -> Request {
        request()
            .post(self.request_url(["cancel"]).build().value())
            .build()
    }

    fn request_url<const SIZE: usize>(&self, suffix: [&str; SIZE]) -> RequestUrlBuilder {
        let mut path_segments = vec![
            "repos",
            self.pipeline.repo().owner().as_str(),
            self.pipeline.repo().name().as_str(),
            "actions",
            "runs",
            self.pipeline.id().as_str(),
        ];

        path_segments.extend(suffix);
        url(&self.base_url).path_segments(path_segments)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitHubPipelineCollection {
    base_url: String,
}

impl GitHubPipelineCollection {
    pub fn make(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }

    pub fn list(&self, query: &PipelineListQuery) -> RequestUrl {
        apply_page(
            url(&self.base_url).path_segments([
                "repos",
                query.repo().owner().as_str(),
                query.repo().name().as_str(),
                "actions",
                "runs",
            ]),
            query.page(),
        )
        .build()
    }
}

impl Default for GitHubPipelineCollection {
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
