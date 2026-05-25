use crate::{
    CodeReview, CodeReviewBuilder, CodeReviewDraftBuilder, CodeReviewListQuery, CodeReviewPatch,
    CodeReviewQueryBuilder, MissingCodeReviewDraftRepo, MissingCodeReviewId, MissingCodeReviewRepo,
    MissingCodeReviewTitle, PageRequest, PageRequestBuilder, ProvidedCodeReviewDraftRepo,
    ProvidedCodeReviewId, ProvidedCodeReviewRepo, ProvidedCodeReviewTitle, Repo, Request,
    RequestUrl,
};

use super::{ManagedCodeReviewDeleteProvider, ManagedCodeReviewProvider, VcsManager};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedCodeReviewBuilder<Driver, RepoState, CodeReviewIdState> {
    pub(super) manager: VcsManager<Driver>,
    pub(super) code_review: CodeReviewBuilder<RepoState, CodeReviewIdState>,
}

impl<Driver> ManagedCodeReviewBuilder<Driver, MissingCodeReviewRepo, MissingCodeReviewId>
where
    Driver: ManagedCodeReviewProvider,
{
    pub fn collection(&self) -> ManagedCodeReviewCollection<Driver> {
        ManagedCodeReviewCollection {
            manager: self.manager.clone(),
        }
    }

    pub fn query(&self) -> CodeReviewQueryBuilder {
        CodeReviewQueryBuilder
    }

    pub fn draft(
        &self,
    ) -> ManagedCodeReviewDraftBuilder<Driver, MissingCodeReviewDraftRepo, MissingCodeReviewTitle>
    {
        ManagedCodeReviewDraftBuilder {
            manager: self.manager.clone(),
            draft: crate::code_review().draft(),
        }
    }
}

impl<Driver, CodeReviewIdState>
    ManagedCodeReviewBuilder<Driver, MissingCodeReviewRepo, CodeReviewIdState>
where
    Driver: ManagedCodeReviewProvider,
{
    pub fn repo(
        self,
        repo: impl Into<Repo>,
    ) -> ManagedCodeReviewBuilder<Driver, ProvidedCodeReviewRepo, CodeReviewIdState> {
        ManagedCodeReviewBuilder {
            manager: self.manager,
            code_review: self.code_review.repo(repo),
        }
    }
}

impl<Driver, RepoState> ManagedCodeReviewBuilder<Driver, RepoState, MissingCodeReviewId>
where
    Driver: ManagedCodeReviewProvider,
{
    pub fn id(
        self,
        id: impl Into<String>,
    ) -> ManagedCodeReviewBuilder<Driver, RepoState, ProvidedCodeReviewId> {
        ManagedCodeReviewBuilder {
            manager: self.manager,
            code_review: self.code_review.id(id),
        }
    }
}

impl<Driver> ManagedCodeReviewBuilder<Driver, ProvidedCodeReviewRepo, ProvidedCodeReviewId>
where
    Driver: ManagedCodeReviewProvider,
{
    pub fn build(self) -> ManagedCodeReview<Driver> {
        self.get()
    }

    pub fn get(self) -> ManagedCodeReview<Driver> {
        ManagedCodeReview {
            manager: self.manager,
            code_review: self.code_review.build(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedCodeReviewDraftBuilder<Driver, RepoState, TitleState> {
    manager: VcsManager<Driver>,
    draft: CodeReviewDraftBuilder<RepoState, TitleState>,
}

impl<Driver, TitleState>
    ManagedCodeReviewDraftBuilder<Driver, MissingCodeReviewDraftRepo, TitleState>
where
    Driver: ManagedCodeReviewProvider,
{
    pub fn repo(
        self,
        repo: impl Into<Repo>,
    ) -> ManagedCodeReviewDraftBuilder<Driver, ProvidedCodeReviewDraftRepo, TitleState> {
        ManagedCodeReviewDraftBuilder {
            manager: self.manager,
            draft: self.draft.repo(repo),
        }
    }
}

impl<Driver, RepoState> ManagedCodeReviewDraftBuilder<Driver, RepoState, MissingCodeReviewTitle>
where
    Driver: ManagedCodeReviewProvider,
{
    pub fn title(
        self,
        title: impl Into<String>,
    ) -> ManagedCodeReviewDraftBuilder<Driver, RepoState, ProvidedCodeReviewTitle> {
        ManagedCodeReviewDraftBuilder {
            manager: self.manager,
            draft: self.draft.title(title),
        }
    }
}

impl<Driver, RepoState, TitleState> ManagedCodeReviewDraftBuilder<Driver, RepoState, TitleState>
where
    Driver: ManagedCodeReviewProvider,
{
    pub fn source(mut self, source: impl Into<String>) -> Self {
        self.draft = self.draft.source(source);
        self
    }

    pub fn target(mut self, target: impl Into<String>) -> Self {
        self.draft = self.draft.target(target);
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.draft = self.draft.body(body);
        self
    }
}

impl<Driver>
    ManagedCodeReviewDraftBuilder<Driver, ProvidedCodeReviewDraftRepo, ProvidedCodeReviewTitle>
where
    Driver: ManagedCodeReviewProvider,
{
    pub fn create(self) -> Request {
        self.manager
            .driver
            .code_review_create_request(&self.draft.get())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedCodeReview<Driver> {
    manager: VcsManager<Driver>,
    code_review: CodeReview,
}

impl<Driver> ManagedCodeReview<Driver>
where
    Driver: ManagedCodeReviewProvider,
{
    pub fn url(&self) -> RequestUrl {
        self.manager.driver.code_review_url(&self.code_review)
    }

    pub fn code_review(&self) -> &CodeReview {
        &self.code_review
    }

    pub fn repo(&self) -> &Repo {
        self.code_review.repo()
    }

    pub fn update(&self, patch: &CodeReviewPatch) -> Request {
        self.manager.driver.code_review_update_request(patch)
    }

    pub fn close(&self) -> Request {
        self.manager
            .driver
            .code_review_close_request(&self.code_review)
    }
}

impl<Driver> ManagedCodeReview<Driver>
where
    Driver: ManagedCodeReviewDeleteProvider,
{
    pub fn delete(&self) -> Request {
        self.manager
            .driver
            .code_review_delete_request(&self.code_review)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedCodeReviewCollection<Driver> {
    manager: VcsManager<Driver>,
}

impl<Driver> ManagedCodeReviewCollection<Driver>
where
    Driver: ManagedCodeReviewProvider,
{
    pub fn list(&self, query: &CodeReviewListQuery) -> RequestUrl {
        self.manager.driver.code_review_list_url(query)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedRepoCodeReviews<Driver> {
    pub(super) manager: VcsManager<Driver>,
    pub(super) repo: Repo,
    pub(super) page: Option<PageRequest>,
}

impl<Driver> ManagedRepoCodeReviews<Driver>
where
    Driver: ManagedCodeReviewProvider,
{
    pub fn url(&self) -> RequestUrl {
        let query = self.query();
        self.manager.driver.code_review_list_url(&query)
    }

    pub fn pagination(self) -> ManagedRepoCodeReviewsPagination<Driver> {
        ManagedRepoCodeReviewsPagination {
            manager: self.manager,
            repo: self.repo,
            page: PageRequestBuilder::default(),
        }
    }

    pub fn repo(&self) -> &Repo {
        &self.repo
    }

    fn query(&self) -> CodeReviewListQuery {
        CodeReviewQueryBuilder.list(self.repo.clone(), self.page.clone())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedRepoCodeReviewsPagination<Driver> {
    manager: VcsManager<Driver>,
    repo: Repo,
    page: PageRequestBuilder,
}

impl<Driver> ManagedRepoCodeReviewsPagination<Driver>
where
    Driver: ManagedCodeReviewProvider,
{
    pub fn limit(mut self, limit: u16) -> Self {
        self.page = self.page.limit(limit);
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.page = self.page.cursor(cursor);
        self
    }

    pub fn build(self) -> ManagedRepoCodeReviews<Driver> {
        self.list()
    }

    pub fn get(self) -> ManagedRepoCodeReviews<Driver> {
        self.list()
    }

    pub fn list(self) -> ManagedRepoCodeReviews<Driver> {
        ManagedRepoCodeReviews {
            manager: self.manager,
            repo: self.repo,
            page: Some(self.page.build()),
        }
    }

    pub fn url(self) -> RequestUrl {
        self.list().url()
    }
}
