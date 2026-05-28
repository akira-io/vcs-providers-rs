use git_cognition_core::{
    Branch, CodeReview, CodeReviewResponseMapper, CognitionError, CognitionResult, Commit, Issue,
    IssueResponseMapper, LifecycleState, Organization, OrganizationKind,
    OrganizationResponseMapper, Page, Release, ReleaseResponseMapper, Repo, Repository,
    RepositoryResponseMapper, Response, Visibility, error, pipeline, repo,
};
use git_cognition_core::{Pipeline, PipelineResponseMapper};
use serde::Deserialize;

use crate::PROVIDER_ID;
use response_types::{
    GitHubBranch, GitHubCodeReview, GitHubCommit, GitHubIssue, GitHubOrganization, GitHubPipeline,
    GitHubPipelinePage, GitHubReference, GitHubRelease, GitHubRepository,
};

mod response_types;

#[derive(Clone, Copy, Debug, Default)]
pub struct GitHubRepositoryMapper;

#[derive(Clone, Copy, Debug, Default)]
pub struct GitHubOrganizationMapper;

#[derive(Clone, Copy, Debug, Default)]
pub struct GitHubIssueMapper;

#[derive(Clone, Copy, Debug, Default)]
pub struct GitHubCodeReviewMapper;

#[derive(Clone, Copy, Debug, Default)]
pub struct GitHubReleaseMapper;

#[derive(Clone, Copy, Debug, Default)]
pub struct GitHubPipelineMapper;

impl RepositoryResponseMapper for GitHubRepositoryMapper {
    fn repository(
        &self,
        requested_repo: &Repo,
        response: &Response,
    ) -> CognitionResult<Repository> {
        let repository_response = github_repository(response)?;
        let repository_repo = repository_response
            .repo()
            .unwrap_or_else(|| requested_repo.clone());

        Ok(repository(repository_repo, repository_response))
    }

    fn repositories(&self, response: &Response) -> CognitionResult<Page<Repository>> {
        let repositories = github_repositories(response)?
            .into_iter()
            .filter_map(|repository_response| {
                repository_response
                    .repo()
                    .map(|repository_repo| repository(repository_repo, repository_response))
            })
            .collect();

        Ok(page(repositories, response))
    }

    fn branches(&self, response: &Response) -> CognitionResult<Page<Branch>> {
        let branches = parse_body::<Vec<GitHubBranch>>(response, "invalid github branch response")?
            .into_iter()
            .map(|branch| Branch::make(branch.name))
            .collect();

        Ok(page(branches, response))
    }

    fn branch(&self, response: &Response) -> CognitionResult<Branch> {
        let branch = parse_body::<GitHubReference>(response, "invalid github branch response")?;

        Ok(Branch::make(branch.name()))
    }

    fn commits(&self, response: &Response) -> CognitionResult<Page<Commit>> {
        let commits = parse_body::<Vec<GitHubCommit>>(response, "invalid github commit response")?
            .into_iter()
            .map(|commit| Commit::make(commit.sha))
            .collect();

        Ok(page(commits, response))
    }
}

impl OrganizationResponseMapper for GitHubOrganizationMapper {
    fn organizations(&self, response: &Response) -> CognitionResult<Page<Organization>> {
        let organizations = parse_body::<Vec<GitHubOrganization>>(
            response,
            "invalid github organization response",
        )?
        .into_iter()
        .map(|organization| {
            Organization::make(
                PROVIDER_ID,
                organization.id.to_string(),
                organization.login,
                OrganizationKind::Organization,
            )
        })
        .collect();

        Ok(page(organizations, response))
    }
}

impl IssueResponseMapper for GitHubIssueMapper {
    fn issue(&self, requested_issue: &Issue, response: &Response) -> CognitionResult<Issue> {
        let issue = parse_body::<GitHubIssue>(response, "invalid github issue response")?;

        Ok(git_cognition_core::issue()
            .repo(requested_issue.repo().clone())
            .id(issue.number.to_string())
            .get())
    }

    fn issues(&self, requested_repo: &Repo, response: &Response) -> CognitionResult<Page<Issue>> {
        let issues =
            parse_body::<Vec<GitHubIssue>>(response, "invalid github issue list response")?
                .into_iter()
                .map(|issue| {
                    git_cognition_core::issue()
                        .repo(requested_repo.clone())
                        .id(issue.number.to_string())
                        .get()
                })
                .collect();

        Ok(page(issues, response))
    }
}

impl CodeReviewResponseMapper for GitHubCodeReviewMapper {
    fn code_review(
        &self,
        requested_code_review: &CodeReview,
        response: &Response,
    ) -> CognitionResult<CodeReview> {
        let code_review =
            parse_body::<GitHubCodeReview>(response, "invalid github code review response")?;

        Ok(git_cognition_core::code_review()
            .repo(requested_code_review.repo().clone())
            .id(code_review.number.to_string())
            .get())
    }

    fn code_reviews(
        &self,
        requested_repo: &Repo,
        response: &Response,
    ) -> CognitionResult<Page<CodeReview>> {
        let code_reviews = parse_body::<Vec<GitHubCodeReview>>(
            response,
            "invalid github code review list response",
        )?
        .into_iter()
        .map(|code_review| {
            git_cognition_core::code_review()
                .repo(requested_repo.clone())
                .id(code_review.number.to_string())
                .get()
        })
        .collect();

        Ok(page(code_reviews, response))
    }
}

impl ReleaseResponseMapper for GitHubReleaseMapper {
    fn release(
        &self,
        requested_release: &Release,
        response: &Response,
    ) -> CognitionResult<Release> {
        let release = parse_body::<GitHubRelease>(response, "invalid github release response")?;

        Ok(git_cognition_core::release()
            .repo(requested_release.repo().clone())
            .id(release.id.to_string())
            .get())
    }

    fn releases(
        &self,
        requested_repo: &Repo,
        response: &Response,
    ) -> CognitionResult<Page<Release>> {
        let releases =
            parse_body::<Vec<GitHubRelease>>(response, "invalid github release list response")?
                .into_iter()
                .map(|release| {
                    git_cognition_core::release()
                        .repo(requested_repo.clone())
                        .id(release.id.to_string())
                        .get()
                })
                .collect();

        Ok(page(releases, response))
    }
}

impl PipelineResponseMapper for GitHubPipelineMapper {
    fn pipeline(
        &self,
        requested_pipeline: &Pipeline,
        response: &Response,
    ) -> CognitionResult<Pipeline> {
        let pipeline = parse_body::<GitHubPipeline>(response, "invalid github pipeline response")?;

        Ok(git_cognition_core::pipeline()
            .repo(requested_pipeline.repo().clone())
            .id(pipeline.id.to_string())
            .get())
    }

    fn pipelines(
        &self,
        requested_repo: &Repo,
        response: &Response,
    ) -> CognitionResult<Page<Pipeline>> {
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

        Ok(page(pipelines, response))
    }
}

fn github_repository(response: &Response) -> CognitionResult<GitHubRepository> {
    parse_body(response, "invalid github repository response")
}

fn github_repositories(response: &Response) -> CognitionResult<Vec<GitHubRepository>> {
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

fn parse_body<'a, T>(response: &'a Response, message: &str) -> CognitionResult<T>
where
    T: Deserialize<'a>,
{
    let response_body = response.body().ok_or_else(|| invalid_response(message))?;

    serde_json::from_str(response_body.as_str()).map_err(|_parse_error| invalid_response(message))
}

fn invalid_response(message: &str) -> CognitionError {
    error().invalid_input(message)
}

fn page<T>(items: Vec<T>, response: &Response) -> Page<T> {
    git_cognition_core::pagination()
        .page(items)
        .optional_next(crate::pagination::next_cursor(response))
        .build()
}
