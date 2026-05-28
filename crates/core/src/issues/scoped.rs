use crate::{
    BoxFuture, CognitionResult, Issue, IssueId, IssuePatchBuilder, Issues, Page, Repo, error, issue,
};

pub struct ScopedIssueOperation {
    issues: Box<dyn Issues>,
    repo: Repo,
    id: Option<String>,
    title: Option<String>,
    body: Option<String>,
    closed: Option<bool>,
}

impl ScopedIssueOperation {
    pub fn make(issues: Box<dyn Issues>, repo: Repo) -> Self {
        Self {
            issues,
            repo,
            id: None,
            title: None,
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

    pub fn get(self) -> BoxFuture<'static, CognitionResult<Issue>> {
        let Some(id) = self.id else {
            return Box::pin(async { Err(error().invalid_input("issue id is required")) });
        };

        let issues = self.issues;
        let repo = self.repo;

        Box::pin(async move { Issues::get(&*issues, repo, IssueId::make(id)).await })
    }

    pub fn list(self) -> BoxFuture<'static, CognitionResult<Page<Issue>>> {
        let issues = self.issues;
        let query = issue().query().location(self.repo).list();

        Box::pin(async move { Issues::list(&*issues, query).await })
    }

    pub fn create(self) -> BoxFuture<'static, CognitionResult<Issue>> {
        let Some(title) = self.title else {
            return Box::pin(async { Err(error().invalid_input("issue title is required")) });
        };

        let mut draft = issue().draft().repo(self.repo).title(title);

        if let Some(body) = self.body {
            draft = draft.body(body);
        }

        let issues = self.issues;
        let draft = draft.get();

        Box::pin(async move { Issues::create(&*issues, draft).await })
    }

    pub fn update(self) -> BoxFuture<'static, CognitionResult<Issue>> {
        let Some(id) = self.id else {
            return Box::pin(async { Err(error().invalid_input("issue id is required")) });
        };

        let issue = crate::issue().repo(self.repo).id(id).get();
        let mut patch = issue.patch();

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

    pub fn close(self) -> BoxFuture<'static, CognitionResult<Issue>> {
        let Some(id) = self.id else {
            return Box::pin(async { Err(error().invalid_input("issue id is required")) });
        };

        let issue = crate::issue().repo(self.repo).id(id).get();
        let patch = issue.patch().closed().get();
        let issues = self.issues;

        Box::pin(async move { Issues::close(&*issues, patch).await })
    }

    pub fn delete(self) -> BoxFuture<'static, CognitionResult<()>> {
        let Some(id) = self.id else {
            return Box::pin(async { Err(error().invalid_input("issue id is required")) });
        };

        let issues = self.issues;
        let issue = crate::issue().repo(self.repo).id(id).get();

        Box::pin(async move { Issues::delete(&*issues, issue).await })
    }
}

fn apply_issue_closed_state(patch: IssuePatchBuilder, closed: bool) -> IssuePatchBuilder {
    if closed {
        return patch.closed();
    }

    patch.open()
}
