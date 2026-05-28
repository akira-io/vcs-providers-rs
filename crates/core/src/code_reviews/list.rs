use crate::{
    BoxFuture, CodeReview, CodeReviews, CognitionResult, Page, PageRequest, PageRequestBuilder,
    Repo, code_review, error,
};

pub struct CodeReviewListOperation {
    code_reviews: Box<dyn CodeReviews>,
    repo: Option<Repo>,
    page: Option<PageRequest>,
}

impl CodeReviewListOperation {
    pub fn make(code_reviews: Box<dyn CodeReviews>) -> Self {
        Self {
            code_reviews,
            repo: None,
            page: None,
        }
    }

    pub fn location(mut self, repo: Repo) -> Self {
        self.repo = Some(repo);
        self
    }

    pub fn pagination(self) -> CodeReviewListPaginationOperation {
        CodeReviewListPaginationOperation {
            code_reviews: self.code_reviews,
            repo: self.repo,
            page: PageRequestBuilder::default(),
        }
    }

    pub fn list(self) -> BoxFuture<'static, CognitionResult<Page<CodeReview>>> {
        let Some(repo) = self.repo else {
            return Box::pin(async { Err(error().invalid_input("repository is required")) });
        };

        let code_reviews = self.code_reviews;
        let query = code_review()
            .query()
            .location(repo)
            .optional_pagination(self.page)
            .list();

        Box::pin(async move { CodeReviews::list(&*code_reviews, query).await })
    }
}

pub struct CodeReviewListPaginationOperation {
    code_reviews: Box<dyn CodeReviews>,
    repo: Option<Repo>,
    page: PageRequestBuilder,
}

impl CodeReviewListPaginationOperation {
    pub fn limit(mut self, limit: u16) -> Self {
        self.page = self.page.limit(limit);
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.page = self.page.cursor(cursor);
        self
    }

    pub fn list(self) -> CodeReviewListOperation {
        CodeReviewListOperation {
            code_reviews: self.code_reviews,
            repo: self.repo,
            page: Some(self.page.build()),
        }
    }
}
