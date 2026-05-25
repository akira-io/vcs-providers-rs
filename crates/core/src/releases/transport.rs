use std::sync::Arc;

use crate::{
    BoxFuture, ManagedReleaseProvider, Page, Release, ReleaseDraft, ReleaseId, ReleaseListQuery,
    ReleasePatch, Releases, Repo, Request, RequestHeader, Response, Transport, VcsResult, error,
};

pub trait ReleaseResponseMapper: Send + Sync {
    fn release(&self, requested_release: &Release, response: &Response) -> VcsResult<Release>;

    fn releases(&self, requested_repo: &Repo, response: &Response) -> VcsResult<Page<Release>>;
}

#[derive(Clone)]
pub struct TransportBackedReleases<Driver, Mapper> {
    driver: Driver,
    transport: Arc<dyn Transport>,
    mapper: Mapper,
    headers: Vec<RequestHeader>,
}

impl<Driver, Mapper> TransportBackedReleases<Driver, Mapper>
where
    Driver: ManagedReleaseProvider,
    Mapper: ReleaseResponseMapper,
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

    fn send_request<'a>(&'a self, request: Request) -> BoxFuture<'a, VcsResult<Response>> {
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

impl<Driver, Mapper> Releases for TransportBackedReleases<Driver, Mapper>
where
    Driver: ManagedReleaseProvider + Send + Sync,
    Mapper: ReleaseResponseMapper,
{
    fn get(&self, repo: Repo, id: ReleaseId) -> BoxFuture<'_, VcsResult<Release>> {
        Box::pin(async move {
            let requested_release = Release::make(repo, id);
            let request = crate::request()
                .get(self.driver.release_url(&requested_release).value())
                .build();
            let response = self.send_request(request).await?;

            self.mapper.release(&requested_release, &response)
        })
    }

    fn list(&self, query: ReleaseListQuery) -> BoxFuture<'_, VcsResult<Page<Release>>> {
        Box::pin(async move {
            let request = crate::request()
                .get(self.driver.release_list_url(&query).value())
                .build();
            let response = self.send_request(request).await?;

            self.mapper.releases(query.repo(), &response)
        })
    }

    fn create(&self, draft: ReleaseDraft) -> BoxFuture<'_, VcsResult<Release>> {
        Box::pin(async move {
            let requested_release =
                Release::make(draft.repo().clone(), ReleaseId::make(draft.tag()));
            let response = self
                .send_request(self.driver.release_create_request(&draft))
                .await?;

            self.mapper.release(&requested_release, &response)
        })
    }

    fn update(&self, patch: ReleasePatch) -> BoxFuture<'_, VcsResult<Release>> {
        Box::pin(async move {
            let response = self
                .send_request(self.driver.release_update_request(&patch))
                .await?;

            self.mapper.release(patch.release(), &response)
        })
    }

    fn delete(&self, release: Release) -> BoxFuture<'_, VcsResult<()>> {
        Box::pin(async move {
            self.send_request(self.driver.release_delete_request(&release))
                .await?;

            Ok(())
        })
    }
}
