use std::sync::Arc;

use crate::{
    BoxFuture, CognitionResult, ManagedPipelineProvider, Page, Pipeline, PipelineId,
    PipelineListQuery, Pipelines, Repo, Request, RequestHeader, Response, Transport, error,
};

pub trait PipelineResponseMapper: Send + Sync {
    fn pipeline(
        &self,
        requested_pipeline: &Pipeline,
        response: &Response,
    ) -> CognitionResult<Pipeline>;

    fn pipelines(
        &self,
        requested_repo: &Repo,
        response: &Response,
    ) -> CognitionResult<Page<Pipeline>>;
}

#[derive(Clone)]
pub struct TransportBackedPipelines<Driver, Mapper> {
    driver: Driver,
    transport: Arc<dyn Transport>,
    mapper: Mapper,
    headers: Vec<RequestHeader>,
}

impl<Driver, Mapper> TransportBackedPipelines<Driver, Mapper>
where
    Driver: ManagedPipelineProvider,
    Mapper: PipelineResponseMapper,
{
    pub fn make(driver: Driver, transport: Arc<dyn Transport>, mapper: Mapper) -> Self {
        Self {
            driver,
            transport,
            mapper,
            headers: Vec::new(),
        }
    }

    pub fn with_headers(mut self, headers: impl IntoIterator<Item = RequestHeader>) -> Self {
        self.headers.extend(headers);
        self
    }

    fn send_request<'a>(&'a self, request: Request) -> BoxFuture<'a, CognitionResult<Response>> {
        Box::pin(async move {
            let response = self.transport.send(self.apply_headers(request)).await?;

            if let Some(error) = error().from_response(&response) {
                return Err(error);
            }

            Ok(response)
        })
    }

    fn apply_headers(&self, request: Request) -> Request {
        self.headers
            .iter()
            .cloned()
            .fold(request, Request::with_header)
    }
}

impl<Driver, Mapper> Pipelines for TransportBackedPipelines<Driver, Mapper>
where
    Driver: ManagedPipelineProvider + Send + Sync,
    Mapper: PipelineResponseMapper,
{
    fn get(&self, repo: Repo, id: PipelineId) -> BoxFuture<'_, CognitionResult<Pipeline>> {
        Box::pin(async move {
            let requested_pipeline = crate::pipeline().repo(repo).id(id.as_str()).get();
            let request = crate::request()
                .get(self.driver.pipeline_url(&requested_pipeline).value())
                .build();
            let response = self.send_request(request).await?;

            self.mapper.pipeline(&requested_pipeline, &response)
        })
    }

    fn list(&self, query: PipelineListQuery) -> BoxFuture<'_, CognitionResult<Page<Pipeline>>> {
        Box::pin(async move {
            let request = crate::request()
                .get(self.driver.pipeline_list_url(&query).value())
                .build();
            let response = self.send_request(request).await?;

            self.mapper.pipelines(query.repo(), &response)
        })
    }

    fn rerun(&self, pipeline: Pipeline) -> BoxFuture<'_, CognitionResult<Pipeline>> {
        Box::pin(async move {
            let request = self.driver.pipeline_rerun_request(&pipeline)?;
            let response = self.send_request(request).await?;

            self.mapper.pipeline(&pipeline, &response)
        })
    }

    fn cancel(&self, pipeline: Pipeline) -> BoxFuture<'_, CognitionResult<Pipeline>> {
        Box::pin(async move {
            let request = self.driver.pipeline_cancel_request(&pipeline)?;
            let response = self.send_request(request).await?;

            self.mapper.pipeline(&pipeline, &response)
        })
    }
}
