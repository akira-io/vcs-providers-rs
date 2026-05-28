use serde::{Deserialize, Serialize};

use crate::{BoxFuture, CognitionResult, Page, Repo, transport_not_configured};

#[path = "pipelines/queries.rs"]
mod queries;
#[path = "pipelines/scoped.rs"]
mod scoped;
#[path = "pipelines/transport.rs"]
mod transport;

#[allow(unused_imports)]
pub use queries::{PipelineListQuery, PipelineListQueryBuilder, PipelineQueryBuilder};
pub use scoped::{PipelinePaginationOperation, PipelinesFluent, ScopedPipelineOperation};
pub use transport::{PipelineResponseMapper, TransportBackedPipelines};

pub trait ManagedPipelineProvider: crate::ManagedProvider {
    fn pipeline_url(&self, pipeline: &Pipeline) -> crate::RequestUrl;

    fn pipeline_list_url(&self, query: &PipelineListQuery) -> crate::RequestUrl;

    fn pipeline_rerun_request(&self, pipeline: &Pipeline) -> CognitionResult<crate::Request>;

    fn pipeline_cancel_request(&self, pipeline: &Pipeline) -> CognitionResult<crate::Request>;
}

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
    pub fn builder() -> PipelineBuilder<MissingPipelineRepo, MissingPipelineId> {
        PipelineBuilder {
            repo: MissingPipelineRepo,
            id: MissingPipelineId,
        }
    }

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingPipelineRepo;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvidedPipelineRepo {
    repo: Repo,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingPipelineId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvidedPipelineId {
    id: PipelineId,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PipelineBuilder<RepoState, PipelineIdState> {
    repo: RepoState,
    id: PipelineIdState,
}

impl<PipelineIdState> PipelineBuilder<MissingPipelineRepo, PipelineIdState> {
    pub fn repo(self, repo: Repo) -> PipelineBuilder<ProvidedPipelineRepo, PipelineIdState> {
        PipelineBuilder {
            repo: ProvidedPipelineRepo { repo },
            id: self.id,
        }
    }
}

impl<RepoState> PipelineBuilder<RepoState, MissingPipelineId> {
    pub fn id(self, id: impl Into<String>) -> PipelineBuilder<RepoState, ProvidedPipelineId> {
        PipelineBuilder {
            repo: self.repo,
            id: ProvidedPipelineId {
                id: PipelineId::make(id),
            },
        }
    }
}

impl PipelineBuilder<ProvidedPipelineRepo, ProvidedPipelineId> {
    pub fn build(self) -> Pipeline {
        self.get()
    }

    pub fn get(self) -> Pipeline {
        Pipeline {
            repo: self.repo.repo,
            id: self.id.id,
        }
    }
}

impl PipelineBuilder<MissingPipelineRepo, MissingPipelineId> {
    pub fn query(self) -> PipelineQueryBuilder {
        PipelineQueryBuilder
    }
}

pub trait Pipelines: Send + Sync {
    fn get(&self, repo: Repo, id: PipelineId) -> BoxFuture<'_, CognitionResult<Pipeline>>;

    fn list(&self, query: PipelineListQuery) -> BoxFuture<'_, CognitionResult<Page<Pipeline>>>;

    fn rerun(&self, pipeline: Pipeline) -> BoxFuture<'_, CognitionResult<Pipeline>>;

    fn cancel(&self, pipeline: Pipeline) -> BoxFuture<'_, CognitionResult<Pipeline>>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TransportNotConfiguredPipelines;

impl Pipelines for TransportNotConfiguredPipelines {
    fn get(&self, _repo: Repo, _id: PipelineId) -> BoxFuture<'_, CognitionResult<Pipeline>> {
        transport_not_configured()
    }

    fn list(&self, _query: PipelineListQuery) -> BoxFuture<'_, CognitionResult<Page<Pipeline>>> {
        transport_not_configured()
    }

    fn rerun(&self, _pipeline: Pipeline) -> BoxFuture<'_, CognitionResult<Pipeline>> {
        transport_not_configured()
    }

    fn cancel(&self, _pipeline: Pipeline) -> BoxFuture<'_, CognitionResult<Pipeline>> {
        transport_not_configured()
    }
}
