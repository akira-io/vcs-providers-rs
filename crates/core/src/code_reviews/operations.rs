use super::delete::CodeReviewDeleteOperation;
use crate::{
    BoxFuture, CodeReview, CodeReviewDraft, CodeReviewListOperation, CodeReviewPatchBuilder,
    CodeReviews, CognitionResult, Repo, ScopedCodeReviewOperation, error,
};

pub trait CodeReviewsFluent {
    fn location(self, repo: Repo) -> ScopedCodeReviewOperation;

    fn list(self) -> CodeReviewListOperation;

    fn create(self) -> CodeReviewCreateOperation;

    fn update(self) -> CodeReviewUpdateOperation;

    fn merge(self) -> CodeReviewMergeOperation;

    fn close(self) -> CodeReviewCloseOperation;

    fn delete(self) -> CodeReviewDeleteOperation;
}

impl CodeReviewsFluent for Box<dyn CodeReviews> {
    fn location(self, repo: Repo) -> ScopedCodeReviewOperation {
        ScopedCodeReviewOperation::make(self, repo)
    }

    fn list(self) -> CodeReviewListOperation {
        CodeReviewListOperation::make(self)
    }

    fn create(self) -> CodeReviewCreateOperation {
        CodeReviewCreateOperation {
            code_reviews: self,
            repo: None,
            title: None,
            source: None,
            target: None,
            body: None,
        }
    }

    fn update(self) -> CodeReviewUpdateOperation {
        CodeReviewUpdateOperation {
            code_reviews: self,
            repo: None,
            id: None,
            title: None,
            body: None,
            closed: None,
        }
    }

    fn merge(self) -> CodeReviewMergeOperation {
        CodeReviewMergeOperation {
            code_reviews: self,
            repo: None,
            id: None,
        }
    }

    fn close(self) -> CodeReviewCloseOperation {
        CodeReviewCloseOperation {
            code_reviews: self,
            repo: None,
            id: None,
        }
    }

    fn delete(self) -> CodeReviewDeleteOperation {
        CodeReviewDeleteOperation::make(self)
    }
}

pub struct CodeReviewCreateOperation {
    code_reviews: Box<dyn CodeReviews>,
    repo: Option<Repo>,
    title: Option<String>,
    source: Option<String>,
    target: Option<String>,
    body: Option<String>,
}

impl CodeReviewCreateOperation {
    pub fn location(mut self, repo: Repo) -> Self {
        self.repo = Some(repo);
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

    pub fn create(self) -> BoxFuture<'static, CognitionResult<CodeReview>> {
        let Some(repo) = self.repo else {
            return Box::pin(async { Err(error().invalid_input("repository is required")) });
        };

        let Some(title) = self.title else {
            return Box::pin(async { Err(error().invalid_input("code review title is required")) });
        };

        let mut draft = CodeReviewDraft::builder().repo(repo).title(title);

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
}

pub struct CodeReviewUpdateOperation {
    code_reviews: Box<dyn CodeReviews>,
    repo: Option<Repo>,
    id: Option<String>,
    title: Option<String>,
    body: Option<String>,
    closed: Option<bool>,
}

impl CodeReviewUpdateOperation {
    pub fn location(mut self, repo: Repo) -> Self {
        self.repo = Some(repo);
        self
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
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

    pub fn open(mut self) -> Self {
        self.closed = Some(false);
        self
    }

    pub fn update(self) -> BoxFuture<'static, CognitionResult<CodeReview>> {
        let Some(repo) = self.repo else {
            return Box::pin(async { Err(error().invalid_input("repository is required")) });
        };

        let Some(id) = self.id else {
            return Box::pin(async { Err(error().invalid_input("code review id is required")) });
        };

        let code_review = crate::code_review().repo(repo).id(id).get();
        let mut patch = code_review.patch();

        if let Some(title) = self.title {
            patch = patch.title(title);
        }

        if let Some(body) = self.body {
            patch = patch.body(body);
        }

        if let Some(closed) = self.closed {
            patch = apply_code_review_closed_state(patch, closed);
        }

        let code_reviews = self.code_reviews;
        let patch = patch.get();

        Box::pin(async move { CodeReviews::update(&*code_reviews, patch).await })
    }
}

pub struct CodeReviewMergeOperation {
    code_reviews: Box<dyn CodeReviews>,
    repo: Option<Repo>,
    id: Option<String>,
}

impl CodeReviewMergeOperation {
    pub fn location(mut self, repo: Repo) -> Self {
        self.repo = Some(repo);
        self
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn merge(self) -> BoxFuture<'static, CognitionResult<CodeReview>> {
        let Some(repo) = self.repo else {
            return Box::pin(async { Err(error().invalid_input("repository is required")) });
        };

        let Some(id) = self.id else {
            return Box::pin(async { Err(error().invalid_input("code review id is required")) });
        };

        let code_reviews = self.code_reviews;
        let code_review = crate::code_review().repo(repo).id(id).get();

        Box::pin(async move { CodeReviews::merge(&*code_reviews, code_review).await })
    }
}

pub struct CodeReviewCloseOperation {
    code_reviews: Box<dyn CodeReviews>,
    repo: Option<Repo>,
    id: Option<String>,
}

impl CodeReviewCloseOperation {
    pub fn location(mut self, repo: Repo) -> Self {
        self.repo = Some(repo);
        self
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn close(self) -> BoxFuture<'static, CognitionResult<CodeReview>> {
        let Some(repo) = self.repo else {
            return Box::pin(async { Err(error().invalid_input("repository is required")) });
        };

        let Some(id) = self.id else {
            return Box::pin(async { Err(error().invalid_input("code review id is required")) });
        };

        let code_reviews = self.code_reviews;
        let code_review = crate::code_review().repo(repo).id(id).get();

        Box::pin(async move { CodeReviews::close(&*code_reviews, code_review).await })
    }
}

fn apply_code_review_closed_state(
    patch: CodeReviewPatchBuilder,
    closed: bool,
) -> CodeReviewPatchBuilder {
    if closed {
        return patch.closed();
    }

    patch.open()
}
