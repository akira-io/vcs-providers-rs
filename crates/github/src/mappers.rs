use serde::Deserialize;
use vcs_provider_core::{
    Branch, CodeReview, CodeReviewId, CodeReviewResponseMapper, Commit, Issue, IssueId,
    IssueResponseMapper, LifecycleState, Page, Release, ReleaseId, ReleaseResponseMapper, Repo,
    Repository, RepositoryResponseMapper, Response, VcsError, VcsResult, Visibility, error,
    pipeline, repo,
};
use vcs_provider_core::{Pipeline, PipelineResponseMapper};

use crate::PROVIDER_ID;

#[derive(Clone, Copy, Debug, Default)]
pub struct GitHubRepositoryMapper;

#[derive(Clone, Copy, Debug, Default)]
pub struct GitHubIssueMapper;

#[derive(Clone, Copy, Debug, Default)]
pub struct GitHubCodeReviewMapper;

#[derive(Clone, Copy, Debug, Default)]
pub struct GitHubReleaseMapper;

#[derive(Clone, Copy, Debug, Default)]
pub struct GitHubPipelineMapper;

impl RepositoryResponseMapper for GitHubRepositoryMapper {
    fn repository(&self, requested_repo: &Repo, response: &Response) -> VcsResult<Repository> {
        let repository_response = github_repository(response)?;
        let repository_repo = repository_response
            .repo()
            .unwrap_or_else(|| requested_repo.clone());

        Ok(repository(repository_repo, repository_response))
    }

    fn repositories(&self, response: &Response) -> VcsResult<Page<Repository>> {
        let repositories = github_repositories(response)?
            .into_iter()
            .filter_map(|repository_response| {
                repository_response
                    .repo()
                    .map(|repository_repo| repository(repository_repo, repository_response))
            })
            .collect();

        Ok(Page::make(repositories))
    }

    fn branches(&self, response: &Response) -> VcsResult<Page<Branch>> {
        let branches = parse_body::<Vec<GitHubBranch>>(response, "invalid github branch response")?
            .into_iter()
            .map(|branch| Branch::make(branch.name))
            .collect();

        Ok(Page::make(branches))
    }

    fn commits(&self, response: &Response) -> VcsResult<Page<Commit>> {
        let commits = parse_body::<Vec<GitHubCommit>>(response, "invalid github commit response")?
            .into_iter()
            .map(|commit| Commit::make(commit.sha))
            .collect();

        Ok(Page::make(commits))
    }
}

impl IssueResponseMapper for GitHubIssueMapper {
    fn issue(&self, requested_issue: &Issue, response: &Response) -> VcsResult<Issue> {
        let issue = parse_body::<GitHubIssue>(response, "invalid github issue response")?;

        Ok(Issue::make(
            requested_issue.repo().clone(),
            IssueId::make(issue.number.to_string()),
        ))
    }

    fn issues(&self, requested_repo: &Repo, response: &Response) -> VcsResult<Page<Issue>> {
        let issues =
            parse_body::<Vec<GitHubIssue>>(response, "invalid github issue list response")?
                .into_iter()
                .map(|issue| {
                    Issue::make(
                        requested_repo.clone(),
                        IssueId::make(issue.number.to_string()),
                    )
                })
                .collect();

        Ok(Page::make(issues))
    }
}

impl CodeReviewResponseMapper for GitHubCodeReviewMapper {
    fn code_review(
        &self,
        requested_code_review: &CodeReview,
        response: &Response,
    ) -> VcsResult<CodeReview> {
        let code_review =
            parse_body::<GitHubCodeReview>(response, "invalid github code review response")?;

        Ok(CodeReview::make(
            requested_code_review.repo().clone(),
            CodeReviewId::make(code_review.number.to_string()),
        ))
    }

    fn code_reviews(
        &self,
        requested_repo: &Repo,
        response: &Response,
    ) -> VcsResult<Page<CodeReview>> {
        let code_reviews = parse_body::<Vec<GitHubCodeReview>>(
            response,
            "invalid github code review list response",
        )?
        .into_iter()
        .map(|code_review| {
            CodeReview::make(
                requested_repo.clone(),
                CodeReviewId::make(code_review.number.to_string()),
            )
        })
        .collect();

        Ok(Page::make(code_reviews))
    }
}

impl ReleaseResponseMapper for GitHubReleaseMapper {
    fn release(&self, requested_release: &Release, response: &Response) -> VcsResult<Release> {
        let release = parse_body::<GitHubRelease>(response, "invalid github release response")?;

        Ok(Release::make(
            requested_release.repo().clone(),
            ReleaseId::make(release.id.to_string()),
        ))
    }

    fn releases(&self, requested_repo: &Repo, response: &Response) -> VcsResult<Page<Release>> {
        let releases =
            parse_body::<Vec<GitHubRelease>>(response, "invalid github release list response")?
                .into_iter()
                .map(|release| {
                    Release::make(
                        requested_repo.clone(),
                        ReleaseId::make(release.id.to_string()),
                    )
                })
                .collect();

        Ok(Page::make(releases))
    }
}

impl PipelineResponseMapper for GitHubPipelineMapper {
    fn pipeline(&self, requested_pipeline: &Pipeline, response: &Response) -> VcsResult<Pipeline> {
        let pipeline = parse_body::<GitHubPipeline>(response, "invalid github pipeline response")?;

        Ok(vcs_provider_core::Pipeline::make(
            requested_pipeline.repo().clone(),
            vcs_provider_core::PipelineId::make(pipeline.id.to_string()),
        ))
    }

    fn pipelines(&self, requested_repo: &Repo, response: &Response) -> VcsResult<Page<Pipeline>> {
        let pipelines =
            parse_body::<GitHubPipelinePage>(response, "invalid github pipeline list response")?
                .workflow_runs
                .into_iter()
                .map(|pipeline_response| {
                    pipeline()
                        .repo(requested_repo.clone())
                        .id(pipeline_response.id.to_string())
                        .get()
                })
                .collect();

        Ok(Page::make(pipelines))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct GitHubRepository {
    full_name: Option<String>,
    private: Option<bool>,
    archived: Option<bool>,
    disabled: Option<bool>,
}

impl GitHubRepository {
    fn repo(&self) -> Option<Repo> {
        parse_repository_path(self.full_name.as_deref())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct GitHubBranch {
    name: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct GitHubCommit {
    sha: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct GitHubIssue {
    number: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct GitHubCodeReview {
    number: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct GitHubRelease {
    id: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct GitHubPipelinePage {
    workflow_runs: Vec<GitHubPipeline>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct GitHubPipeline {
    id: u64,
}

fn github_repository(response: &Response) -> VcsResult<GitHubRepository> {
    parse_body(response, "invalid github repository response")
}

fn github_repositories(response: &Response) -> VcsResult<Vec<GitHubRepository>> {
    parse_body(response, "invalid github repository list response")
}

fn repository(repository_repo: Repo, repository_response: GitHubRepository) -> Repository {
    repo()
        .owner(repository_repo.owner().as_str())
        .name(repository_repo.name().as_str())
        .provider(PROVIDER_ID)
        .visibility(visibility(repository_response.private.unwrap_or_default()))
        .lifecycle(lifecycle_state(
            repository_response.archived.unwrap_or_default(),
            repository_response.disabled.unwrap_or_default(),
        ))
        .get()
}

fn visibility(is_private: bool) -> Visibility {
    if is_private {
        return Visibility::Private;
    }

    Visibility::Public
}

fn lifecycle_state(is_archived: bool, is_disabled: bool) -> LifecycleState {
    if is_disabled {
        return LifecycleState::Disabled;
    }

    if is_archived {
        return LifecycleState::Archived;
    }

    LifecycleState::Active
}

fn parse_repository_path(repository_path: Option<&str>) -> Option<Repo> {
    let (owner_name, repository_name) = repository_path?.split_once('/')?;

    Some(repo().owner(owner_name).name(repository_name).get())
}

fn parse_body<'a, T>(response: &'a Response, message: &str) -> VcsResult<T>
where
    T: Deserialize<'a>,
{
    let response_body = response.body().ok_or_else(|| invalid_response(message))?;

    serde_json::from_str(response_body.as_str()).map_err(|_parse_error| invalid_response(message))
}

fn invalid_response(message: &str) -> VcsError {
    error().invalid_input(message)
}
