use crate::{
    BoxFuture, Issue, Issues, Page, PageRequest, PageRequestBuilder, Repo, VcsResult, error, issue,
};

pub struct IssueListOperation {
    issues: Box<dyn Issues>,
    repo: Option<Repo>,
    page: Option<PageRequest>,
}

impl IssueListOperation {
    pub fn make(issues: Box<dyn Issues>) -> Self {
        Self {
            issues,
            repo: None,
            page: None,
        }
    }

    pub fn location(mut self, repo: Repo) -> Self {
        self.repo = Some(repo);
        self
    }

    pub fn pagination(self) -> IssueListPaginationOperation {
        IssueListPaginationOperation {
            issues: self.issues,
            repo: self.repo,
            page: PageRequestBuilder::default(),
        }
    }

    pub fn list(self) -> BoxFuture<'static, VcsResult<Page<Issue>>> {
        let Some(repo) = self.repo else {
            return Box::pin(async { Err(error().invalid_input("repository is required")) });
        };

        let issues = self.issues;
        let query = issue()
            .query()
            .location(repo)
            .optional_pagination(self.page)
            .list();

        Box::pin(async move { Issues::list(&*issues, query).await })
    }
}

pub struct IssueListPaginationOperation {
    issues: Box<dyn Issues>,
    repo: Option<Repo>,
    page: PageRequestBuilder,
}

impl IssueListPaginationOperation {
    pub fn limit(mut self, limit: u16) -> Self {
        self.page = self.page.limit(limit);
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.page = self.page.cursor(cursor);
        self
    }

    pub fn list(self) -> IssueListOperation {
        IssueListOperation {
            issues: self.issues,
            repo: self.repo,
            page: Some(self.page.build()),
        }
    }
}
