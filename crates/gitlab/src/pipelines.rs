use vcs_provider_core::{
    Pipeline, PipelineListQuery, Request, RequestUrl, RequestUrlBuilder, request, url,
};

use crate::{DEFAULT_BASE_URL, request_pagination::apply_page};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitLabPipeline {
    base_url: String,
    pipeline: Pipeline,
}

impl GitLabPipeline {
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
            .post(self.request_url(["retry"]).build().value())
            .build()
    }

    pub fn cancel(&self) -> Request {
        request()
            .post(self.request_url(["cancel"]).build().value())
            .build()
    }

    fn request_url<const SIZE: usize>(&self, suffix: [&str; SIZE]) -> RequestUrlBuilder {
        let project_path = project_path(self.pipeline.repo());
        let mut path_segments = vec![
            "api",
            "v4",
            "projects",
            project_path.as_str(),
            "pipelines",
            self.pipeline.id().as_str(),
        ];

        path_segments.extend(suffix);
        url(&self.base_url).path_segments(path_segments)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GitLabPipelineCollection {
    base_url: String,
}

impl GitLabPipelineCollection {
    pub fn make(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
        }
    }

    pub fn list(&self, query: &PipelineListQuery) -> RequestUrl {
        let project_path = project_path(query.repo());

        apply_page(
            url(&self.base_url).path_segments([
                "api",
                "v4",
                "projects",
                project_path.as_str(),
                "pipelines",
            ]),
            query.page(),
        )
        .build()
    }
}

impl Default for GitLabPipelineCollection {
    fn default() -> Self {
        Self::make(DEFAULT_BASE_URL)
    }
}

fn project_path(repo: &vcs_provider_core::Repo) -> String {
    format!("{}/{}", repo.owner().as_str(), repo.name().as_str())
}
