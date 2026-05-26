use crate::{
    ManagedPipelineProvider, MissingPipelineId, MissingPipelineRepo, PageRequest,
    PageRequestBuilder, Pipeline, PipelineBuilder, PipelineListQuery, PipelineQueryBuilder,
    ProvidedPipelineId, ProvidedPipelineRepo, Repo, Request, RequestUrl, VcsManager,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedPipelineBuilder<Driver, RepoState, PipelineIdState> {
    pub(super) manager: VcsManager<Driver>,
    pub(super) pipeline: PipelineBuilder<RepoState, PipelineIdState>,
}

impl<Driver> ManagedPipelineBuilder<Driver, MissingPipelineRepo, MissingPipelineId>
where
    Driver: ManagedPipelineProvider,
{
    pub fn collection(&self) -> ManagedPipelineCollection<Driver> {
        ManagedPipelineCollection {
            manager: self.manager.clone(),
        }
    }

    pub fn query(&self) -> PipelineQueryBuilder {
        PipelineQueryBuilder
    }
}

impl<Driver, PipelineIdState> ManagedPipelineBuilder<Driver, MissingPipelineRepo, PipelineIdState>
where
    Driver: ManagedPipelineProvider,
{
    pub fn repo(
        self,
        repo: impl Into<Repo>,
    ) -> ManagedPipelineBuilder<Driver, ProvidedPipelineRepo, PipelineIdState> {
        ManagedPipelineBuilder {
            manager: self.manager,
            pipeline: self.pipeline.repo(repo.into()),
        }
    }
}

impl<Driver, RepoState> ManagedPipelineBuilder<Driver, RepoState, MissingPipelineId>
where
    Driver: ManagedPipelineProvider,
{
    pub fn id(
        self,
        id: impl Into<String>,
    ) -> ManagedPipelineBuilder<Driver, RepoState, ProvidedPipelineId> {
        ManagedPipelineBuilder {
            manager: self.manager,
            pipeline: self.pipeline.id(id),
        }
    }
}

impl<Driver> ManagedPipelineBuilder<Driver, ProvidedPipelineRepo, ProvidedPipelineId>
where
    Driver: ManagedPipelineProvider,
{
    pub fn build(self) -> ManagedPipeline<Driver> {
        self.get()
    }

    pub fn get(self) -> ManagedPipeline<Driver> {
        ManagedPipeline {
            manager: self.manager,
            pipeline: self.pipeline.get(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedPipeline<Driver> {
    manager: VcsManager<Driver>,
    pipeline: Pipeline,
}

impl<Driver> ManagedPipeline<Driver>
where
    Driver: ManagedPipelineProvider,
{
    pub fn url(&self) -> RequestUrl {
        self.manager.driver.pipeline_url(&self.pipeline)
    }

    pub fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }

    pub fn repo(&self) -> &Repo {
        self.pipeline.repo()
    }

    pub fn rerun(&self) -> crate::VcsResult<Request> {
        self.manager.driver.pipeline_rerun_request(&self.pipeline)
    }

    pub fn cancel(&self) -> crate::VcsResult<Request> {
        self.manager.driver.pipeline_cancel_request(&self.pipeline)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedPipelineCollection<Driver> {
    manager: VcsManager<Driver>,
}

impl<Driver> ManagedPipelineCollection<Driver>
where
    Driver: ManagedPipelineProvider,
{
    pub fn list(&self, query: &PipelineListQuery) -> RequestUrl {
        self.manager.driver.pipeline_list_url(query)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedRepoPipelines<Driver> {
    pub(super) manager: VcsManager<Driver>,
    pub(super) repo: Repo,
    pub(super) page: Option<PageRequest>,
}

impl<Driver> ManagedRepoPipelines<Driver>
where
    Driver: ManagedPipelineProvider,
{
    pub fn url(&self) -> RequestUrl {
        let query = self.query();
        self.manager.driver.pipeline_list_url(&query)
    }

    pub fn pagination(self) -> ManagedRepoPipelinesPagination<Driver> {
        ManagedRepoPipelinesPagination {
            manager: self.manager,
            repo: self.repo,
            page: PageRequestBuilder::default(),
        }
    }

    pub fn repo(&self) -> &Repo {
        &self.repo
    }

    fn query(&self) -> PipelineListQuery {
        PipelineQueryBuilder
            .location(self.repo.clone())
            .optional_pagination(self.page.clone())
            .list()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedRepoPipelinesPagination<Driver> {
    manager: VcsManager<Driver>,
    repo: Repo,
    page: PageRequestBuilder,
}

impl<Driver> ManagedRepoPipelinesPagination<Driver>
where
    Driver: ManagedPipelineProvider,
{
    pub fn limit(mut self, limit: u16) -> Self {
        self.page = self.page.limit(limit);
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.page = self.page.cursor(cursor);
        self
    }

    pub fn build(self) -> ManagedRepoPipelines<Driver> {
        self.list()
    }

    pub fn get(self) -> ManagedRepoPipelines<Driver> {
        self.list()
    }

    pub fn list(self) -> ManagedRepoPipelines<Driver> {
        ManagedRepoPipelines {
            manager: self.manager,
            repo: self.repo,
            page: Some(self.page.build()),
        }
    }

    pub fn url(self) -> RequestUrl {
        self.list().url()
    }
}
