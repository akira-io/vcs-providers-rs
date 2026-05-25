use crate::{Issue, IssuePatch};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IssuePatchBuilder {
    issue: Issue,
    title: Option<String>,
    body: Option<String>,
    closed: Option<bool>,
}

impl IssuePatchBuilder {
    pub fn make(issue: Issue) -> Self {
        Self {
            issue,
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

    pub fn build(self) -> IssuePatch {
        self.get()
    }

    pub fn get(self) -> IssuePatch {
        IssuePatch {
            issue: self.issue,
            title: self.title,
            body: self.body,
            closed: self.closed,
        }
    }
}
