use std::sync::Arc;

use crate::{
    BoxFuture, CognitionResult, Issue, IssueDraft, IssueId, IssueListQuery, IssuePatch, Issues,
    ManagedIssueProvider, Page, Repo, Request, RequestHeader, Response, Transport, error,
};

pub trait IssueResponseMapper: Send + Sync {
    fn issue(&self, requested_issue: &Issue, response: &Response) -> CognitionResult<Issue>;

    fn issues(&self, requested_repo: &Repo, response: &Response) -> CognitionResult<Page<Issue>>;
}

#[derive(Clone)]
pub struct TransportBackedIssues<Driver, Mapper> {
    driver: Driver,
    transport: Arc<dyn Transport>,
    mapper: Mapper,
    headers: Vec<RequestHeader>,
}

impl<Driver, Mapper> TransportBackedIssues<Driver, Mapper>
where
    Driver: ManagedIssueProvider,
    Mapper: IssueResponseMapper,
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

impl<Driver, Mapper> Issues for TransportBackedIssues<Driver, Mapper>
where
    Driver: ManagedIssueProvider + Send + Sync,
    Mapper: IssueResponseMapper,
{
    fn get(&self, repo: Repo, id: IssueId) -> BoxFuture<'_, CognitionResult<Issue>> {
        Box::pin(async move {
            let requested_issue = crate::issue().repo(repo).id(id.as_str()).get();
            let request = crate::request()
                .get(self.driver.issue_url(&requested_issue).value())
                .build();
            let response = self.send_request(request).await?;

            self.mapper.issue(&requested_issue, &response)
        })
    }

    fn list(&self, query: IssueListQuery) -> BoxFuture<'_, CognitionResult<Page<Issue>>> {
        Box::pin(async move {
            let request = crate::request()
                .get(self.driver.issue_list_url(&query).value())
                .build();
            let response = self.send_request(request).await?;

            self.mapper.issues(query.repo(), &response)
        })
    }

    fn create(&self, draft: IssueDraft) -> BoxFuture<'_, CognitionResult<Issue>> {
        Box::pin(async move {
            let requested_issue = crate::issue().repo(draft.repo().clone()).id("").get();
            let response = self
                .send_request(self.driver.issue_create_request(&draft))
                .await?;

            self.mapper.issue(&requested_issue, &response)
        })
    }

    fn update(&self, patch: IssuePatch) -> BoxFuture<'_, CognitionResult<Issue>> {
        Box::pin(async move {
            let response = self
                .send_request(self.driver.issue_update_request(&patch))
                .await?;

            self.mapper.issue(patch.issue(), &response)
        })
    }

    fn close(&self, patch: IssuePatch) -> BoxFuture<'_, CognitionResult<Issue>> {
        Box::pin(async move {
            let response = self
                .send_request(self.driver.issue_close_request(&patch))
                .await?;

            self.mapper.issue(patch.issue(), &response)
        })
    }

    fn delete(&self, issue: Issue) -> BoxFuture<'_, CognitionResult<()>> {
        Box::pin(async move {
            let request = self.driver.issue_delete_request(&issue)?;
            self.send_request(request).await?;

            Ok(())
        })
    }
}
