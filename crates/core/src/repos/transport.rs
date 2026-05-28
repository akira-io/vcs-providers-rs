use std::sync::Arc;

use crate::repos::{
    BoxFuture, Branch, BranchDraft, Commit, Repo, Repos, RepositoryDraft, RepositoryListQuery,
    RepositoryPatch, RepositorySearchQuery,
};
use crate::{
    CognitionResult, ManagedProvider, Page, Repository, Request, RequestHeader, Response,
    Transport, error, request,
};

pub trait RepositoryResponseMapper: Send + Sync {
    fn repository(&self, requested_repo: &Repo, response: &Response)
    -> CognitionResult<Repository>;

    fn repositories(&self, response: &Response) -> CognitionResult<Page<Repository>>;

    fn branches(&self, response: &Response) -> CognitionResult<Page<Branch>>;

    fn branch(&self, response: &Response) -> CognitionResult<Branch>;

    fn commits(&self, response: &Response) -> CognitionResult<Page<Commit>>;
}

#[derive(Clone)]
pub struct TransportBackedRepos<Driver, Mapper> {
    driver: Driver,
    transport: Arc<dyn Transport>,
    mapper: Mapper,
    headers: Vec<RequestHeader>,
}

impl<Driver, Mapper> TransportBackedRepos<Driver, Mapper>
where
    Driver: ManagedProvider,
    Mapper: RepositoryResponseMapper,
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

impl<Driver, Mapper> Repos for TransportBackedRepos<Driver, Mapper>
where
    Driver: ManagedProvider + Send + Sync,
    Mapper: RepositoryResponseMapper,
{
    fn get(&self, repo: Repo) -> BoxFuture<'_, CognitionResult<Repository>> {
        Box::pin(async move {
            let request = request().get(self.driver.repo_url(&repo).value()).build();
            let response = self.send_request(request).await?;

            self.mapper.repository(&repo, &response)
        })
    }

    fn list(&self, query: RepositoryListQuery) -> BoxFuture<'_, CognitionResult<Page<Repository>>> {
        Box::pin(async move {
            let request = request()
                .get(self.driver.repo_list_url(&query).value())
                .build();
            let response = self.send_request(request).await?;

            self.mapper.repositories(&response)
        })
    }

    fn search(
        &self,
        query: RepositorySearchQuery,
    ) -> BoxFuture<'_, CognitionResult<Page<Repository>>> {
        Box::pin(async move {
            let request = request()
                .get(self.driver.repo_search_url(&query).value())
                .build();
            let response = self.send_request(request).await?;

            self.mapper.repositories(&response)
        })
    }

    fn create(&self, draft: RepositoryDraft) -> BoxFuture<'_, CognitionResult<Repository>> {
        Box::pin(async move {
            let request = self.driver.repo_create_request(&draft);
            let response = self.send_request(request).await?;

            self.mapper.repository(draft.repo(), &response)
        })
    }

    fn update(&self, patch: RepositoryPatch) -> BoxFuture<'_, CognitionResult<Repository>> {
        Box::pin(async move {
            let request = self.driver.repo_update_request(&patch);
            let response = self.send_request(request).await?;

            self.mapper.repository(patch.repo(), &response)
        })
    }

    fn delete(&self, repo: Repo) -> BoxFuture<'_, CognitionResult<()>> {
        Box::pin(async move {
            let request = self.driver.repo_delete_request(&repo);
            self.send_request(request).await?;

            Ok(())
        })
    }

    fn branches(&self, repo: Repo) -> BoxFuture<'_, CognitionResult<Page<Branch>>> {
        Box::pin(async move {
            let request = request()
                .get(self.driver.repo_branches_url(&repo, None).value())
                .build();
            let response = self.send_request(request).await?;

            self.mapper.branches(&response)
        })
    }

    fn create_branch(&self, draft: BranchDraft) -> BoxFuture<'_, CognitionResult<Branch>> {
        Box::pin(async move {
            let request = self.driver.repo_branch_create_request(&draft)?;
            let response = self.send_request(request).await?;

            self.mapper.branch(&response)
        })
    }

    fn delete_branch(&self, repo: Repo, branch_name: String) -> BoxFuture<'_, CognitionResult<()>> {
        Box::pin(async move {
            let request = self
                .driver
                .repo_branch_delete_request(&repo, &branch_name)?;
            self.send_request(request).await?;

            Ok(())
        })
    }

    fn commits(&self, repo: Repo) -> BoxFuture<'_, CognitionResult<Page<Commit>>> {
        Box::pin(async move {
            let request = request()
                .get(self.driver.repo_commits_url(&repo, None).value())
                .build();
            let response = self.send_request(request).await?;

            self.mapper.commits(&response)
        })
    }
}
