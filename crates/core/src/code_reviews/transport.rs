use std::sync::Arc;

use crate::{
    BoxFuture, CodeReview, CodeReviewDraft, CodeReviewId, CodeReviewListQuery, CodeReviewPatch,
    CodeReviews, ManagedCodeReviewProvider, Page, Repo, Request, RequestHeader, Response,
    Transport, VcsResult, error,
};

pub trait CodeReviewResponseMapper: Send + Sync {
    fn code_review(
        &self,
        requested_code_review: &CodeReview,
        response: &Response,
    ) -> VcsResult<CodeReview>;

    fn code_reviews(
        &self,
        requested_repo: &Repo,
        response: &Response,
    ) -> VcsResult<Page<CodeReview>>;
}

#[derive(Clone)]
pub struct TransportBackedCodeReviews<Driver, Mapper> {
    driver: Driver,
    transport: Arc<dyn Transport>,
    mapper: Mapper,
    headers: Vec<RequestHeader>,
}

impl<Driver, Mapper> TransportBackedCodeReviews<Driver, Mapper>
where
    Driver: ManagedCodeReviewProvider,
    Mapper: CodeReviewResponseMapper,
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

impl<Driver, Mapper> CodeReviews for TransportBackedCodeReviews<Driver, Mapper>
where
    Driver: ManagedCodeReviewProvider + Send + Sync,
    Mapper: CodeReviewResponseMapper,
{
    fn get(&self, repo: Repo, id: CodeReviewId) -> BoxFuture<'_, VcsResult<CodeReview>> {
        Box::pin(async move {
            let requested_code_review = CodeReview::make(repo, id);
            let request = crate::request()
                .get(self.driver.code_review_url(&requested_code_review).value())
                .build();
            let response = self.send_request(request).await?;

            self.mapper.code_review(&requested_code_review, &response)
        })
    }

    fn list(&self, query: CodeReviewListQuery) -> BoxFuture<'_, VcsResult<Page<CodeReview>>> {
        Box::pin(async move {
            let request = crate::request()
                .get(self.driver.code_review_list_url(&query).value())
                .build();
            let response = self.send_request(request).await?;

            self.mapper.code_reviews(query.repo(), &response)
        })
    }

    fn create(&self, draft: CodeReviewDraft) -> BoxFuture<'_, VcsResult<CodeReview>> {
        Box::pin(async move {
            let requested_code_review =
                CodeReview::make(draft.repo().clone(), CodeReviewId::make(""));
            let response = self
                .send_request(self.driver.code_review_create_request(&draft))
                .await?;

            self.mapper.code_review(&requested_code_review, &response)
        })
    }

    fn update(&self, patch: CodeReviewPatch) -> BoxFuture<'_, VcsResult<CodeReview>> {
        Box::pin(async move {
            let response = self
                .send_request(self.driver.code_review_update_request(&patch))
                .await?;

            self.mapper.code_review(patch.code_review(), &response)
        })
    }

    fn merge(&self, code_review: CodeReview) -> BoxFuture<'_, VcsResult<CodeReview>> {
        Box::pin(async move { Ok(code_review) })
    }

    fn close(&self, code_review: CodeReview) -> BoxFuture<'_, VcsResult<CodeReview>> {
        Box::pin(async move {
            let response = self
                .send_request(self.driver.code_review_close_request(&code_review))
                .await?;

            self.mapper.code_review(&code_review, &response)
        })
    }
}
