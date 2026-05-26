use crate::{
    BoxFuture, Issue, IssueId, IssueListOperation, IssuePatchBuilder, Issues, Repo,
    ScopedIssueOperation, VcsResult, error,
};

pub trait IssuesFluent {
    fn location(self, repo: Repo) -> ScopedIssueOperation;

    fn list(self) -> IssueListOperation;

    fn create(self) -> IssueCreateOperation;

    fn update(self) -> IssueUpdateOperation;

    fn close(self) -> IssueCloseOperation;
}

impl IssuesFluent for Box<dyn Issues> {
    fn location(self, repo: Repo) -> ScopedIssueOperation {
        ScopedIssueOperation::make(self, repo)
    }

    fn list(self) -> IssueListOperation {
        IssueListOperation::make(self)
    }

    fn create(self) -> IssueCreateOperation {
        IssueCreateOperation {
            issues: self,
            repo: None,
            title: None,
            body: None,
        }
    }

    fn update(self) -> IssueUpdateOperation {
        IssueUpdateOperation {
            issues: self,
            repo: None,
            id: None,
            title: None,
            body: None,
            closed: None,
        }
    }

    fn close(self) -> IssueCloseOperation {
        IssueCloseOperation {
            issues: self,
            repo: None,
            id: None,
        }
    }
}

pub struct IssueCreateOperation {
    issues: Box<dyn Issues>,
    repo: Option<Repo>,
    title: Option<String>,
    body: Option<String>,
}

impl IssueCreateOperation {
    pub fn location(mut self, repo: Repo) -> Self {
        self.repo = Some(repo);
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

    pub fn create(self) -> BoxFuture<'static, VcsResult<Issue>> {
        let Some(repo) = self.repo else {
            return Box::pin(async { Err(error().invalid_input("repository is required")) });
        };

        let Some(title) = self.title else {
            return Box::pin(async { Err(error().invalid_input("issue title is required")) });
        };

        let mut draft = crate::issue().draft().repo(repo).title(title);

        if let Some(body) = self.body {
            draft = draft.body(body);
        }

        let issues = self.issues;
        let draft = draft.get();

        Box::pin(async move { Issues::create(&*issues, draft).await })
    }
}

pub struct IssueUpdateOperation {
    issues: Box<dyn Issues>,
    repo: Option<Repo>,
    id: Option<String>,
    title: Option<String>,
    body: Option<String>,
    closed: Option<bool>,
}

impl IssueUpdateOperation {
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

    pub fn update(self) -> BoxFuture<'static, VcsResult<Issue>> {
        let Some(repo) = self.repo else {
            return Box::pin(async { Err(error().invalid_input("repository is required")) });
        };

        let Some(id) = self.id else {
            return Box::pin(async { Err(error().invalid_input("issue id is required")) });
        };

        let issue = Issue::make(repo, IssueId::make(id));
        let mut patch = IssuePatchBuilder::make(issue);

        if let Some(title) = self.title {
            patch = patch.title(title);
        }

        if let Some(body) = self.body {
            patch = patch.body(body);
        }

        if let Some(closed) = self.closed {
            patch = apply_issue_closed_state(patch, closed);
        }

        let issues = self.issues;
        let patch = patch.get();

        Box::pin(async move { Issues::update(&*issues, patch).await })
    }
}

pub struct IssueCloseOperation {
    issues: Box<dyn Issues>,
    repo: Option<Repo>,
    id: Option<String>,
}

impl IssueCloseOperation {
    pub fn location(mut self, repo: Repo) -> Self {
        self.repo = Some(repo);
        self
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn close(self) -> BoxFuture<'static, VcsResult<Issue>> {
        let Some(repo) = self.repo else {
            return Box::pin(async { Err(error().invalid_input("repository is required")) });
        };

        let Some(id) = self.id else {
            return Box::pin(async { Err(error().invalid_input("issue id is required")) });
        };

        let issues = self.issues;
        let issue = Issue::make(repo, IssueId::make(id));
        let patch = IssuePatchBuilder::make(issue).closed().get();

        Box::pin(async move { Issues::close(&*issues, patch).await })
    }
}

fn apply_issue_closed_state(patch: IssuePatchBuilder, closed: bool) -> IssuePatchBuilder {
    if closed {
        return patch.closed();
    }

    patch.open()
}
