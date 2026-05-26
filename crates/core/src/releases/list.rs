use crate::{
    BoxFuture, Page, PageRequest, PageRequestBuilder, Release, Releases, Repo, VcsResult, error,
    release,
};

pub struct ReleaseListOperation {
    releases: Box<dyn Releases>,
    repo: Option<Repo>,
    page: Option<PageRequest>,
}

impl ReleaseListOperation {
    pub fn make(releases: Box<dyn Releases>) -> Self {
        Self {
            releases,
            repo: None,
            page: None,
        }
    }

    pub fn location(mut self, repo: Repo) -> Self {
        self.repo = Some(repo);
        self
    }

    pub fn pagination(self) -> ReleaseListPaginationOperation {
        ReleaseListPaginationOperation {
            releases: self.releases,
            repo: self.repo,
            page: PageRequestBuilder::default(),
        }
    }

    pub fn list(self) -> BoxFuture<'static, VcsResult<Page<Release>>> {
        let Some(repo) = self.repo else {
            return Box::pin(async { Err(error().invalid_input("repository is required")) });
        };

        let releases = self.releases;
        let query = release()
            .query()
            .location(repo)
            .optional_pagination(self.page)
            .list();

        Box::pin(async move { Releases::list(&*releases, query).await })
    }
}

pub struct ReleaseListPaginationOperation {
    releases: Box<dyn Releases>,
    repo: Option<Repo>,
    page: PageRequestBuilder,
}

impl ReleaseListPaginationOperation {
    pub fn limit(mut self, limit: u16) -> Self {
        self.page = self.page.limit(limit);
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.page = self.page.cursor(cursor);
        self
    }

    pub fn list(self) -> ReleaseListOperation {
        ReleaseListOperation {
            releases: self.releases,
            repo: self.repo,
            page: Some(self.page.build()),
        }
    }
}
