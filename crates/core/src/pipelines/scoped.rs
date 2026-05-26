use crate::{
    BoxFuture, Page, PageRequestBuilder, Pipeline, PipelineId, Pipelines, Repo, VcsResult, error,
};

pub trait PipelinesFluent {
    fn location(self, repo: Repo) -> ScopedPipelineOperation;
}

impl PipelinesFluent for Box<dyn Pipelines> {
    fn location(self, repo: Repo) -> ScopedPipelineOperation {
        ScopedPipelineOperation::make(self, repo)
    }
}

pub struct ScopedPipelineOperation {
    pipelines: Box<dyn Pipelines>,
    repo: Repo,
    id: Option<String>,
}

impl ScopedPipelineOperation {
    pub fn make(pipelines: Box<dyn Pipelines>, repo: Repo) -> Self {
        Self {
            pipelines,
            repo,
            id: None,
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn pagination(self) -> PipelinePaginationOperation {
        PipelinePaginationOperation {
            pipelines: self.pipelines,
            repo: self.repo,
            page: PageRequestBuilder::default(),
        }
    }

    pub fn get(self) -> BoxFuture<'static, VcsResult<Pipeline>> {
        let Some(id) = self.id else {
            return Box::pin(async { Err(error().invalid_input("pipeline id is required")) });
        };

        let pipelines = self.pipelines;
        let repo = self.repo;

        Box::pin(async move { Pipelines::get(&*pipelines, repo, PipelineId::make(id)).await })
    }

    pub fn list(self) -> BoxFuture<'static, VcsResult<Page<Pipeline>>> {
        let pipelines = self.pipelines;
        let query = crate::pipeline().query().list(self.repo, None);

        Box::pin(async move { Pipelines::list(&*pipelines, query).await })
    }

    pub fn rerun(self) -> BoxFuture<'static, VcsResult<Pipeline>> {
        let Some(id) = self.id else {
            return Box::pin(async { Err(error().invalid_input("pipeline id is required")) });
        };

        let pipelines = self.pipelines;
        let pipeline = Pipeline::make(self.repo, PipelineId::make(id));

        Box::pin(async move { Pipelines::rerun(&*pipelines, pipeline).await })
    }

    pub fn cancel(self) -> BoxFuture<'static, VcsResult<Pipeline>> {
        let Some(id) = self.id else {
            return Box::pin(async { Err(error().invalid_input("pipeline id is required")) });
        };

        let pipelines = self.pipelines;
        let pipeline = Pipeline::make(self.repo, PipelineId::make(id));

        Box::pin(async move { Pipelines::cancel(&*pipelines, pipeline).await })
    }
}

pub struct PipelinePaginationOperation {
    pipelines: Box<dyn Pipelines>,
    repo: Repo,
    page: PageRequestBuilder,
}

impl PipelinePaginationOperation {
    pub fn limit(mut self, limit: u16) -> Self {
        self.page = self.page.limit(limit);
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.page = self.page.cursor(cursor);
        self
    }

    pub fn list(self) -> BoxFuture<'static, VcsResult<Page<Pipeline>>> {
        let pipelines = self.pipelines;
        let query = crate::pipeline()
            .query()
            .list(self.repo, Some(self.page.build()));

        Box::pin(async move { Pipelines::list(&*pipelines, query).await })
    }
}
