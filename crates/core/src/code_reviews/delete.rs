use crate::{BoxFuture, CodeReviews, CognitionResult, Repo, error};

pub struct CodeReviewDeleteOperation {
    code_reviews: Box<dyn CodeReviews>,
    repo: Option<Repo>,
    id: Option<String>,
}

impl CodeReviewDeleteOperation {
    pub fn make(code_reviews: Box<dyn CodeReviews>) -> Self {
        Self {
            code_reviews,
            repo: None,
            id: None,
        }
    }

    pub fn location(mut self, repo: Repo) -> Self {
        self.repo = Some(repo);
        self
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn delete(self) -> BoxFuture<'static, CognitionResult<()>> {
        let Some(repo) = self.repo else {
            return Box::pin(async { Err(error().invalid_input("repository is required")) });
        };

        let Some(id) = self.id else {
            return Box::pin(async { Err(error().invalid_input("code review id is required")) });
        };

        let code_reviews = self.code_reviews;
        let code_review = crate::code_review().repo(repo).id(id).get();

        Box::pin(async move { CodeReviews::delete(&*code_reviews, code_review).await })
    }
}
