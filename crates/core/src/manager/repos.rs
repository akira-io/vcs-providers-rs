use crate::{
    CognitionManager, ManagedCodeReviewBuilder, ManagedCodeReviewProvider, ManagedIssueBuilder,
    ManagedIssueProvider, ManagedPipelineBuilder, ManagedPipelineProvider, ManagedProvider,
    ManagedReleaseBuilder, ManagedReleaseProvider, ManagedRepoCodeReviews, ManagedRepoIssues,
    ManagedRepoPipelines, ManagedRepoReleases, MissingOwnerName, MissingRepositoryName,
    PageRequest, ProvidedCodeReviewId, ProvidedCodeReviewRepo, ProvidedIssueId, ProvidedIssueRepo,
    ProvidedOwnerName, ProvidedPipelineId, ProvidedPipelineRepo, ProvidedReleaseId,
    ProvidedReleaseRepo, ProvidedRepositoryName, Repo, RepoBuilder, RepositoryDraftBuilder,
    RepositoryListQuery, RepositorySearchQuery, RequestUrl, Visibility, code_review, issue,
    pipeline, release,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedRepoBuilder<Driver, OwnerNameState, RepositoryNameState> {
    pub(crate) manager: CognitionManager<Driver>,
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

impl<Driver> ManagedRepoBuilder<Driver, ProvidedOwnerName, ProvidedRepositoryName>
where
    Driver: ManagedPipelineProvider,
{
    pub fn pipeline(
        self,
        id: impl Into<String>,
    ) -> ManagedPipelineBuilder<Driver, ProvidedPipelineRepo, ProvidedPipelineId> {
        ManagedPipelineBuilder {
            manager: self.manager,
            pipeline: pipeline().repo(self.repo.build()).id(id),
        }
    }

    pub fn pipelines(self) -> ManagedRepoPipelines<Driver> {
        ManagedRepoPipelines {
            manager: self.manager,
            repo: self.repo.build(),
            page: None,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedRepositoryDraftBuilder<Driver> {
    manager: CognitionManager<Driver>,
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
    pub(super) manager: CognitionManager<Driver>,
    pub(super) repo: Repo,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedRepoBranchBuilder<Driver> {
    manager: CognitionManager<Driver>,
    repo: Repo,
    name: Option<String>,
    sha: Option<String>,
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

    pub fn delete(&self) -> crate::Request {
        self.manager.driver.repo_delete_request(&self.repo)
    }

    pub fn branch(&self) -> ManagedRepoBranchBuilder<Driver> {
        ManagedRepoBranchBuilder {
            manager: self.manager.clone(),
            repo: self.repo.clone(),
            name: None,
            sha: None,
        }
    }
}

impl<Driver> ManagedRepoBranchBuilder<Driver>
where
    Driver: ManagedProvider,
{
    pub fn name(mut self, branch_name: impl Into<String>) -> Self {
        self.name = Some(branch_name.into());
        self
    }

    pub fn sha(mut self, sha: impl Into<String>) -> Self {
        self.sha = Some(sha.into());
        self
    }

    pub fn create(self) -> crate::CognitionResult<crate::Request> {
        let Some(name) = self.name else {
            return Err(crate::error().invalid_input("branch name is required"));
        };

        let Some(sha) = self.sha else {
            return Err(crate::error().invalid_input("branch sha is required"));
        };

        let draft = crate::BranchDraft::make(self.repo, name, sha);

        self.manager.driver.repo_branch_create_request(&draft)
    }

    pub fn delete(self) -> crate::CognitionResult<crate::Request> {
        let Some(name) = self.name else {
            return Err(crate::error().invalid_input("branch name is required"));
        };

        self.manager
            .driver
            .repo_branch_delete_request(&self.repo, &name)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagedRepoCollection<Driver> {
    manager: CognitionManager<Driver>,
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
