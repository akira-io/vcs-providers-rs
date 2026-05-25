use crate::{CodeReview, CodeReviewPatch};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodeReviewPatchBuilder {
    code_review: CodeReview,
    title: Option<String>,
    body: Option<String>,
    closed: Option<bool>,
}

impl CodeReviewPatchBuilder {
    pub fn make(code_review: CodeReview) -> Self {
        Self {
            code_review,
            title: None,
            body: None,
            closed: None,
        }
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

    pub fn build(self) -> CodeReviewPatch {
        self.get()
    }

    pub fn get(self) -> CodeReviewPatch {
        CodeReviewPatch {
            code_review: self.code_review,
            title: self.title,
            body: self.body,
            closed: self.closed,
        }
    }
}
