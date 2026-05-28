use crate::{
    IssuePatchBuilder, ManagedIssue, ManagedIssueBuilder, ProvidedIssueId, ProvidedIssueRepo,
    Request,
};

use super::ManagedIssueProvider;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedIssueUpdateBuilder<Driver> {
    managed_issue: ManagedIssue<Driver>,
    title: Option<String>,
    body: Option<String>,
    closed: Option<bool>,
}

impl<Driver> ManagedIssueBuilder<Driver, ProvidedIssueRepo, ProvidedIssueId>
where
    Driver: ManagedIssueProvider,
{
    pub fn title(self, title: impl Into<String>) -> ManagedIssueUpdateBuilder<Driver> {
        self.get().title(title)
    }

    pub fn body(self, body: impl Into<String>) -> ManagedIssueUpdateBuilder<Driver> {
        self.get().body(body)
    }

    pub fn closed(self) -> ManagedIssueUpdateBuilder<Driver> {
        self.get().closed()
    }

    pub fn open(self) -> ManagedIssueUpdateBuilder<Driver> {
        self.get().open()
    }

    pub fn close(self) -> Request {
        self.get().close()
    }
}

impl<Driver> ManagedIssue<Driver>
where
    Driver: ManagedIssueProvider,
{
    pub fn title(self, title: impl Into<String>) -> ManagedIssueUpdateBuilder<Driver> {
        ManagedIssueUpdateBuilder::make(self).title(title)
    }

    pub fn body(self, body: impl Into<String>) -> ManagedIssueUpdateBuilder<Driver> {
        ManagedIssueUpdateBuilder::make(self).body(body)
    }

    pub fn closed(self) -> ManagedIssueUpdateBuilder<Driver> {
        ManagedIssueUpdateBuilder::make(self).closed()
    }

    pub fn open(self) -> ManagedIssueUpdateBuilder<Driver> {
        ManagedIssueUpdateBuilder::make(self).open()
    }

    pub fn close(self) -> Request {
        ManagedIssueUpdateBuilder::make(self).closed().close()
    }
}

impl<Driver> ManagedIssueUpdateBuilder<Driver>
where
    Driver: ManagedIssueProvider,
{
    pub fn make(managed_issue: ManagedIssue<Driver>) -> Self {
        Self {
            managed_issue,
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

    pub fn update(self) -> Request {
        let patch = self.patch();
        self.managed_issue
            .manager
            .driver
            .issue_update_request(&patch)
    }

    pub fn close(self) -> Request {
        let mut close_builder = self;
        close_builder.closed = Some(true);
        let patch = close_builder.patch();
        let managed_issue = close_builder.managed_issue;

        managed_issue.manager.driver.issue_close_request(&patch)
    }

    fn patch(&self) -> crate::IssuePatch {
        let mut patch = self.managed_issue.issue.patch();

        if let Some(title) = self.title.clone() {
            patch = patch.title(title);
        }

        if let Some(body) = self.body.clone() {
            patch = patch.body(body);
        }

        if let Some(closed) = self.closed {
            patch = apply_issue_closed_state(patch, closed);
        }

        patch.get()
    }
}

fn apply_issue_closed_state(patch: IssuePatchBuilder, closed: bool) -> IssuePatchBuilder {
    match closed {
        true => patch.closed(),
        false => patch.open(),
    }
}
