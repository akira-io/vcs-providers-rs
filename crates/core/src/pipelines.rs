use serde::{Deserialize, Serialize};

use crate::{BoxFuture, Page, PageRequest, Repo, VcsResult, transport_not_configured};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PipelineId(String);

impl PipelineId {
    pub fn make(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Pipeline {
    repo: Repo,
    id: PipelineId,
}

impl Pipeline {
    pub fn make(repo: Repo, id: PipelineId) -> Self {
        Self { repo, id }
    }

    pub fn repo(&self) -> &Repo {
        &self.repo
    }

    pub fn id(&self) -> &PipelineId {
        &self.id
    }
}

pub trait Pipelines: Send + Sync {
    fn get(&self, repo: Repo, id: PipelineId) -> BoxFuture<'_, VcsResult<Pipeline>>;

    fn list(
        &self,
        repo: Repo,
        page: Option<PageRequest>,
    ) -> BoxFuture<'_, VcsResult<Page<Pipeline>>>;

    fn rerun(&self, pipeline: Pipeline) -> BoxFuture<'_, VcsResult<Pipeline>>;

    fn cancel(&self, pipeline: Pipeline) -> BoxFuture<'_, VcsResult<Pipeline>>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TransportNotConfiguredPipelines;

impl Pipelines for TransportNotConfiguredPipelines {
    fn get(&self, _repo: Repo, _id: PipelineId) -> BoxFuture<'_, VcsResult<Pipeline>> {
        transport_not_configured()
    }

    fn list(
        &self,
        _repo: Repo,
        _page: Option<PageRequest>,
    ) -> BoxFuture<'_, VcsResult<Page<Pipeline>>> {
        transport_not_configured()
    }

    fn rerun(&self, _pipeline: Pipeline) -> BoxFuture<'_, VcsResult<Pipeline>> {
        transport_not_configured()
    }

    fn cancel(&self, _pipeline: Pipeline) -> BoxFuture<'_, VcsResult<Pipeline>> {
        transport_not_configured()
    }
}
