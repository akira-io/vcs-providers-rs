use crate::{
    CodeReviewPatchBuilder, ManagedCodeReview, ManagedCodeReviewBuilder, ProvidedCodeReviewId,
    ProvidedCodeReviewRepo,
};

use super::ManagedCodeReviewProvider;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedCodeReviewUpdateBuilder<Driver> {
    managed_code_review: ManagedCodeReview<Driver>,
    title: Option<String>,
    body: Option<String>,
    closed: Option<bool>,
}

impl<Driver> ManagedCodeReviewBuilder<Driver, ProvidedCodeReviewRepo, ProvidedCodeReviewId>
where
    Driver: ManagedCodeReviewProvider,
{
    pub fn title(self, title: impl Into<String>) -> ManagedCodeReviewUpdateBuilder<Driver> {
        self.get().title(title)
    }

    pub fn body(self, body: impl Into<String>) -> ManagedCodeReviewUpdateBuilder<Driver> {
        self.get().body(body)
    }

    pub fn closed(self) -> ManagedCodeReviewUpdateBuilder<Driver> {
        self.get().closed()
    }

    pub fn open(self) -> ManagedCodeReviewUpdateBuilder<Driver> {
        self.get().open()
    }
}

impl<Driver> ManagedCodeReview<Driver>
where
    Driver: ManagedCodeReviewProvider,
{
    pub fn title(self, title: impl Into<String>) -> ManagedCodeReviewUpdateBuilder<Driver> {
        ManagedCodeReviewUpdateBuilder::make(self).title(title)
    }

    pub fn body(self, body: impl Into<String>) -> ManagedCodeReviewUpdateBuilder<Driver> {
        ManagedCodeReviewUpdateBuilder::make(self).body(body)
    }

    pub fn closed(self) -> ManagedCodeReviewUpdateBuilder<Driver> {
        ManagedCodeReviewUpdateBuilder::make(self).closed()
    }

    pub fn open(self) -> ManagedCodeReviewUpdateBuilder<Driver> {
        ManagedCodeReviewUpdateBuilder::make(self).open()
    }
}

impl<Driver> ManagedCodeReviewUpdateBuilder<Driver>
where
    Driver: ManagedCodeReviewProvider,
{
    pub fn make(managed_code_review: ManagedCodeReview<Driver>) -> Self {
        Self {
            managed_code_review,
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

    pub fn update(self) -> crate::Request {
        let patch = self.patch();
        self.managed_code_review
            .manager
            .driver
            .code_review_update_request(&patch)
    }

    fn patch(&self) -> crate::CodeReviewPatch {
        let mut patch = self.managed_code_review.code_review.patch();

        if let Some(title) = self.title.clone() {
            patch = patch.title(title);
        }

        if let Some(body) = self.body.clone() {
            patch = patch.body(body);
        }

        if let Some(closed) = self.closed {
            patch = apply_code_review_closed_state(patch, closed);
        }

        patch.get()
    }
}

fn apply_code_review_closed_state(
    patch: CodeReviewPatchBuilder,
    closed: bool,
) -> CodeReviewPatchBuilder {
    match closed {
        true => patch.closed(),
        false => patch.open(),
    }
}
