use crate::{
    BoxFuture, CodeReview, CodeReviewDraft, CodeReviewId, CodeReviewPatchBuilder, CodeReviews,
    Page, Repo, VcsResult, code_review, error,
};

pub struct ScopedCodeReviewOperation {
    code_reviews: Box<dyn CodeReviews>,
    repo: Repo,
    id: Option<String>,
    title: Option<String>,
    source: Option<String>,
    target: Option<String>,
    body: Option<String>,
    closed: Option<bool>,
}

impl ScopedCodeReviewOperation {
    pub fn make(code_reviews: Box<dyn CodeReviews>, repo: Repo) -> Self {
        Self {
            code_reviews,
            repo,
            id: None,
            title: None,
            source: None,
            target: None,
            body: None,
            closed: None,
        }
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    pub fn target(mut self, target: impl Into<String>) -> Self {
        self.target = Some(target.into());
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn closed(mut self) -> Self {
        self.closed = Some(true);
        self
    }

    pub fn get(self) -> BoxFuture<'static, VcsResult<CodeReview>> {
        let Some(id) = self.id else {
            return Box::pin(async { Err(error().invalid_input("code review id is required")) });
        };

        let code_reviews = self.code_reviews;
        let repo = self.repo;

        Box::pin(
            async move { CodeReviews::get(&*code_reviews, repo, CodeReviewId::make(id)).await },
        )
    }

    pub fn list(self) -> BoxFuture<'static, VcsResult<Page<CodeReview>>> {
        let code_reviews = self.code_reviews;
        let query = code_review().query().location(self.repo).list();

        Box::pin(async move { CodeReviews::list(&*code_reviews, query).await })
    }

    pub fn create(self) -> BoxFuture<'static, VcsResult<CodeReview>> {
        let Some(title) = self.title else {
            return Box::pin(async { Err(error().invalid_input("code review title is required")) });
        };

        let mut draft = CodeReviewDraft::builder().repo(self.repo).title(title);

        if let Some(source) = self.source {
            draft = draft.source(source);
        }

        if let Some(target) = self.target {
            draft = draft.target(target);
        }

        if let Some(body) = self.body {
            draft = draft.body(body);
        }

        let code_reviews = self.code_reviews;
        let draft = draft.get();

        Box::pin(async move { CodeReviews::create(&*code_reviews, draft).await })
    }

    pub fn update(self) -> BoxFuture<'static, VcsResult<CodeReview>> {
        let Some(id) = self.id else {
            return Box::pin(async { Err(error().invalid_input("code review id is required")) });
        };

        let code_review = CodeReview::make(self.repo, CodeReviewId::make(id));
        let mut patch = CodeReviewPatchBuilder::make(code_review);

        if let Some(title) = self.title {
            patch = patch.title(title);
        }

        if let Some(body) = self.body {
            patch = patch.body(body);
        }

        if let Some(true) = self.closed {
            patch = patch.closed();
        }

        let code_reviews = self.code_reviews;
        let patch = patch.get();

        Box::pin(async move { CodeReviews::update(&*code_reviews, patch).await })
    }

    pub fn close(self) -> BoxFuture<'static, VcsResult<CodeReview>> {
        let Some(id) = self.id else {
            return Box::pin(async { Err(error().invalid_input("code review id is required")) });
        };

        let code_reviews = self.code_reviews;
        let code_review = CodeReview::make(self.repo, CodeReviewId::make(id));

        Box::pin(async move { CodeReviews::close(&*code_reviews, code_review).await })
    }

    pub fn merge(self) -> BoxFuture<'static, VcsResult<CodeReview>> {
        let Some(id) = self.id else {
            return Box::pin(async { Err(error().invalid_input("code review id is required")) });
        };

        let code_reviews = self.code_reviews;
        let code_review = CodeReview::make(self.repo, CodeReviewId::make(id));

        Box::pin(async move { CodeReviews::merge(&*code_reviews, code_review).await })
    }
}
