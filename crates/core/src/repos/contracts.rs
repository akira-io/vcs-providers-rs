use std::sync::Arc;

use crate::repos::{BoxFuture, Branch, Commit, Repo, RepositoryListQuery, RepositorySearchQuery};
use crate::{
    ManagedProvider, Page, Repository, Request, RequestHeader, Response, Transport, VcsResult,
    error, request, transport_not_configured,
};

pub trait Repos: Send + Sync {
    fn get(&self, repo: Repo) -> BoxFuture<'_, VcsResult<Repository>>;

    fn list(&self, query: RepositoryListQuery) -> BoxFuture<'_, VcsResult<Page<Repository>>>;

    fn search(&self, query: RepositorySearchQuery) -> BoxFuture<'_, VcsResult<Page<Repository>>>;

    fn branches(&self, repo: Repo) -> BoxFuture<'_, VcsResult<Page<Branch>>>;

    fn commits(&self, repo: Repo) -> BoxFuture<'_, VcsResult<Page<Commit>>>;
}

#[derive(Clone, Copy, Debug, Default)]
pub struct TransportNotConfiguredRepos;

impl Repos for TransportNotConfiguredRepos {
    fn get(&self, _repo: Repo) -> BoxFuture<'_, VcsResult<Repository>> {
        transport_not_configured()
    }

    fn list(&self, _query: RepositoryListQuery) -> BoxFuture<'_, VcsResult<Page<Repository>>> {
        transport_not_configured()
    }

    fn search(&self, _query: RepositorySearchQuery) -> BoxFuture<'_, VcsResult<Page<Repository>>> {
        transport_not_configured()
    }

    fn branches(&self, _repo: Repo) -> BoxFuture<'_, VcsResult<Page<Branch>>> {
        transport_not_configured()
    }

    fn commits(&self, _repo: Repo) -> BoxFuture<'_, VcsResult<Page<Commit>>> {
        transport_not_configured()
    }
}

pub trait RepositoryResponseMapper: Send + Sync {
    fn repository(&self, requested_repo: &Repo, response: &Response) -> VcsResult<Repository>;

    fn repositories(&self, response: &Response) -> VcsResult<Page<Repository>>;

    fn branches(&self, response: &Response) -> VcsResult<Page<Branch>>;

    fn commits(&self, response: &Response) -> VcsResult<Page<Commit>>;
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

impl<Driver, Mapper> Repos for TransportBackedRepos<Driver, Mapper>
where
    Driver: ManagedProvider + Send + Sync,
    Mapper: RepositoryResponseMapper,
{
    fn get(&self, repo: Repo) -> BoxFuture<'_, VcsResult<Repository>> {
        Box::pin(async move {
            let request = request().get(self.driver.repo_url(&repo).value()).build();
            let response = self.send_request(request).await?;

            self.mapper.repository(&repo, &response)
        })
    }

    fn list(&self, query: RepositoryListQuery) -> BoxFuture<'_, VcsResult<Page<Repository>>> {
        Box::pin(async move {
            let request = request()
                .get(self.driver.repo_list_url(&query).value())
                .build();
            let response = self.send_request(request).await?;

            self.mapper.repositories(&response)
        })
    }

    fn search(&self, query: RepositorySearchQuery) -> BoxFuture<'_, VcsResult<Page<Repository>>> {
        Box::pin(async move {
            let request = request()
                .get(self.driver.repo_search_url(&query).value())
                .build();
            let response = self.send_request(request).await?;

            self.mapper.repositories(&response)
        })
    }

    fn branches(&self, repo: Repo) -> BoxFuture<'_, VcsResult<Page<Branch>>> {
        Box::pin(async move {
            let request = request()
                .get(self.driver.repo_branches_url(&repo, None).value())
                .build();
            let response = self.send_request(request).await?;

            self.mapper.branches(&response)
        })
    }

    fn commits(&self, repo: Repo) -> BoxFuture<'_, VcsResult<Page<Commit>>> {
        Box::pin(async move {
            let request = request()
                .get(self.driver.repo_commits_url(&repo, None).value())
                .build();
            let response = self.send_request(request).await?;

            self.mapper.commits(&response)
        })
    }
}
