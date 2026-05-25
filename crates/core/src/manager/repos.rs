use crate::{
    ManagedCodeReviewBuilder, ManagedCodeReviewProvider, ManagedIssueBuilder, ManagedIssueProvider,
    ManagedProvider, ManagedReleaseBuilder, ManagedReleaseProvider, ManagedRepoCodeReviews,
    ManagedRepoIssues, ManagedRepoReleases, MissingOwnerName, MissingRepositoryName, PageRequest,
    ProvidedCodeReviewId, ProvidedCodeReviewRepo, ProvidedIssueId, ProvidedIssueRepo,
    ProvidedOwnerName, ProvidedReleaseId, ProvidedReleaseRepo, ProvidedRepositoryName, Repo,
    RepoBuilder, RepositoryDraftBuilder, RepositoryListQuery, RepositoryPatch,
    RepositorySearchQuery, RequestUrl, VcsManager, Visibility, code_review, issue, release,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedRepoBuilder<Driver, OwnerNameState, RepositoryNameState> {
    pub(crate) manager: VcsManager<Driver>,
    pub(crate) repo: RepoBuilder<OwnerNameState, RepositoryNameState>,
}

impl<Driver> ManagedRepoBuilder<Driver, MissingOwnerName, MissingRepositoryName>
where
    Driver: ManagedProvider,
{
    pub fn collection(&self) -> ManagedRepoCollection<Driver> {
        ManagedRepoCollection {
            manager: self.manager.clone(),
        }
    }

    pub fn query(&self) -> crate::RepoQueryBuilder {
        crate::RepoQueryBuilder
    }

    pub fn draft(&self, repo: impl Into<Repo>) -> ManagedRepositoryDraftBuilder<Driver> {
        ManagedRepositoryDraftBuilder {
            manager: self.manager.clone(),
            draft: RepositoryDraftBuilder::make(repo.into()),
        }
    }
}

impl<Driver, RepositoryNameState> ManagedRepoBuilder<Driver, MissingOwnerName, RepositoryNameState>
where
    Driver: ManagedProvider,
{
    pub fn owner(
        self,
        owner_name: impl Into<String>,
    ) -> ManagedRepoBuilder<Driver, ProvidedOwnerName, RepositoryNameState> {
        ManagedRepoBuilder {
            manager: self.manager,
            repo: self.repo.owner(owner_name),
        }
    }
}

impl<Driver, OwnerNameState> ManagedRepoBuilder<Driver, OwnerNameState, MissingRepositoryName>
where
    Driver: ManagedProvider,
{
    pub fn name(
        self,
        repository_name: impl Into<String>,
    ) -> ManagedRepoBuilder<Driver, OwnerNameState, ProvidedRepositoryName> {
        ManagedRepoBuilder {
            manager: self.manager,
            repo: self.repo.name(repository_name),
        }
    }
}

impl<Driver> ManagedRepoBuilder<Driver, ProvidedOwnerName, ProvidedRepositoryName>
where
    Driver: ManagedProvider,
{
    pub fn build(self) -> ManagedRepo<Driver> {
        self.get()
    }

    pub fn get(self) -> ManagedRepo<Driver> {
        ManagedRepo {
            manager: self.manager,
            repo: self.repo.build(),
        }
    }
}

impl<Driver> ManagedRepoBuilder<Driver, ProvidedOwnerName, ProvidedRepositoryName>
where
    Driver: ManagedIssueProvider,
{
    pub fn issue(
        self,
        id: impl Into<String>,
    ) -> ManagedIssueBuilder<Driver, ProvidedIssueRepo, ProvidedIssueId> {
        ManagedIssueBuilder {
            manager: self.manager,
            issue: issue().repo(self.repo.build()).id(id),
        }
    }

    pub fn issues(self) -> ManagedRepoIssues<Driver> {
        ManagedRepoIssues {
            manager: self.manager,
            repo: self.repo.build(),
            page: None,
        }
    }
}

impl<Driver> ManagedRepoBuilder<Driver, ProvidedOwnerName, ProvidedRepositoryName>
where
    Driver: ManagedCodeReviewProvider,
{
    pub fn code_review(
        self,
        id: impl Into<String>,
    ) -> ManagedCodeReviewBuilder<Driver, ProvidedCodeReviewRepo, ProvidedCodeReviewId> {
        ManagedCodeReviewBuilder {
            manager: self.manager,
            code_review: code_review().repo(self.repo.build()).id(id),
        }
    }

    pub fn code_reviews(self) -> ManagedRepoCodeReviews<Driver> {
        ManagedRepoCodeReviews {
            manager: self.manager,
            repo: self.repo.build(),
            page: None,
        }
    }
}

impl<Driver> ManagedRepoBuilder<Driver, ProvidedOwnerName, ProvidedRepositoryName>
where
    Driver: ManagedReleaseProvider,
{
    pub fn release(
        self,
        id: impl Into<String>,
    ) -> ManagedReleaseBuilder<Driver, ProvidedReleaseRepo, ProvidedReleaseId> {
        ManagedReleaseBuilder {
            manager: self.manager,
            release: release().repo(self.repo.build()).id(id),
        }
    }

    pub fn releases(self) -> ManagedRepoReleases<Driver> {
        ManagedRepoReleases {
            manager: self.manager,
            repo: self.repo.build(),
            page: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedRepositoryDraftBuilder<Driver> {
    manager: VcsManager<Driver>,
    draft: RepositoryDraftBuilder,
}

impl<Driver> ManagedRepositoryDraftBuilder<Driver>
where
    Driver: ManagedProvider,
{
    pub fn visibility(mut self, visibility: Visibility) -> Self {
        self.draft = self.draft.visibility(visibility);
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.draft = self.draft.description(description);
        self
    }

    pub fn create(self) -> crate::Request {
        self.manager.driver.repo_create_request(&self.draft.get())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedRepo<Driver> {
    manager: VcsManager<Driver>,
    repo: Repo,
}

impl<Driver> ManagedRepo<Driver>
where
    Driver: ManagedProvider,
{
    pub fn url(&self) -> RequestUrl {
        self.manager.driver.repo_url(&self.repo)
    }

    pub fn branches(&self, page: Option<&PageRequest>) -> RequestUrl {
        self.manager.driver.repo_branches_url(&self.repo, page)
    }

    pub fn commits(&self, page: Option<&PageRequest>) -> RequestUrl {
        self.manager.driver.repo_commits_url(&self.repo, page)
    }

    pub fn update(&self, patch: &RepositoryPatch) -> crate::Request {
        self.manager.driver.repo_update_request(patch)
    }

    pub fn delete(&self) -> crate::Request {
        self.manager.driver.repo_delete_request(&self.repo)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedRepoCollection<Driver> {
    manager: VcsManager<Driver>,
}

impl<Driver> ManagedRepoCollection<Driver>
where
    Driver: ManagedProvider,
{
    pub fn list(&self, query: &RepositoryListQuery) -> RequestUrl {
        self.manager.driver.repo_list_url(query)
    }

    pub fn search(&self, query: &RepositorySearchQuery) -> RequestUrl {
        self.manager.driver.repo_search_url(query)
    }
}

impl<Driver> From<ManagedRepo<Driver>> for Repo {
    fn from(managed_repo: ManagedRepo<Driver>) -> Self {
        managed_repo.repo
    }
}
