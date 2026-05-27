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
pub struct GitLabRepositoryMapper;

#[derive(Clone, Copy, Debug, Default)]
pub struct GitLabIssueMapper;

#[derive(Clone, Copy, Debug, Default)]
pub struct GitLabCodeReviewMapper;

#[derive(Clone, Copy, Debug, Default)]
pub struct GitLabReleaseMapper;

#[derive(Clone, Copy, Debug, Default)]
pub struct GitLabPipelineMapper;

impl RepositoryResponseMapper for GitLabRepositoryMapper {
    fn repository(&self, requested_repo: &Repo, response: &Response) -> VcsResult<Repository> {
        let repository_response = gitlab_repository(response)?;
        let repository_repo = repository_response
            .repo()
            .unwrap_or_else(|| requested_repo.clone());

        Ok(repository(repository_repo, repository_response))
    }

    fn repositories(&self, response: &Response) -> VcsResult<Page<Repository>> {
        let repositories = gitlab_repositories(response)?
            .into_iter()
            .filter_map(|repository_response| {
                repository_response
                    .repo()
                    .map(|repository_repo| repository(repository_repo, repository_response))
            })
            .collect();

        Ok(page(repositories, response))
    }

    fn branches(&self, response: &Response) -> VcsResult<Page<Branch>> {
        let branches = parse_body::<Vec<GitLabBranch>>(response, "invalid gitlab branch response")?
            .into_iter()
            .map(|branch| Branch::make(branch.name))
            .collect();

        Ok(page(branches, response))
    }

    fn commits(&self, response: &Response) -> VcsResult<Page<Commit>> {
        let commits = parse_body::<Vec<GitLabCommit>>(response, "invalid gitlab commit response")?
            .into_iter()
            .map(|commit| Commit::make(commit.id))
            .collect();

        Ok(page(commits, response))
    }
}

impl IssueResponseMapper for GitLabIssueMapper {
    fn issue(&self, requested_issue: &Issue, response: &Response) -> VcsResult<Issue> {
        let issue = parse_body::<GitLabIssue>(response, "invalid gitlab issue response")?;

        Ok(Issue::make(
            requested_issue.repo().clone(),
            IssueId::make(issue.iid.to_string()),
        ))
    }

    fn issues(&self, requested_repo: &Repo, response: &Response) -> VcsResult<Page<Issue>> {
        let issues =
            parse_body::<Vec<GitLabIssue>>(response, "invalid gitlab issue list response")?
                .into_iter()
                .map(|issue| {
                    Issue::make(requested_repo.clone(), IssueId::make(issue.iid.to_string()))
                })
                .collect();

        Ok(page(issues, response))
    }
}

impl CodeReviewResponseMapper for GitLabCodeReviewMapper {
    fn code_review(
        &self,
        requested_code_review: &CodeReview,
        response: &Response,
    ) -> VcsResult<CodeReview> {
        let code_review =
            parse_body::<GitLabCodeReview>(response, "invalid gitlab code review response")?;

        Ok(CodeReview::make(
            requested_code_review.repo().clone(),
            CodeReviewId::make(code_review.iid.to_string()),
        ))
    }

    fn code_reviews(
        &self,
        requested_repo: &Repo,
        response: &Response,
    ) -> VcsResult<Page<CodeReview>> {
        let code_reviews = parse_body::<Vec<GitLabCodeReview>>(
            response,
            "invalid gitlab code review list response",
        )?
        .into_iter()
        .map(|code_review| {
            CodeReview::make(
                requested_repo.clone(),
                CodeReviewId::make(code_review.iid.to_string()),
            )
        })
        .collect();

        Ok(page(code_reviews, response))
    }
}

impl ReleaseResponseMapper for GitLabReleaseMapper {
    fn release(&self, requested_release: &Release, response: &Response) -> VcsResult<Release> {
        let release = parse_body::<GitLabRelease>(response, "invalid gitlab release response")?;

        Ok(Release::make(
            requested_release.repo().clone(),
            ReleaseId::make(release.tag_name),
        ))
    }

    fn releases(&self, requested_repo: &Repo, response: &Response) -> VcsResult<Page<Release>> {
        let releases =
            parse_body::<Vec<GitLabRelease>>(response, "invalid gitlab release list response")?
                .into_iter()
                .map(|release| {
                    Release::make(requested_repo.clone(), ReleaseId::make(release.tag_name))
                })
                .collect();

        Ok(page(releases, response))
    }
}

impl PipelineResponseMapper for GitLabPipelineMapper {
    fn pipeline(&self, requested_pipeline: &Pipeline, response: &Response) -> VcsResult<Pipeline> {
        let pipeline = parse_body::<GitLabPipeline>(response, "invalid gitlab pipeline response")?;

        Ok(vcs_provider_core::Pipeline::make(
            requested_pipeline.repo().clone(),
            vcs_provider_core::PipelineId::make(pipeline.id.to_string()),
        ))
    }

    fn pipelines(&self, requested_repo: &Repo, response: &Response) -> VcsResult<Page<Pipeline>> {
        let pipelines =
            parse_body::<Vec<GitLabPipeline>>(response, "invalid gitlab pipeline list response")?
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct GitLabRepository {
    path_with_namespace: Option<String>,
    visibility: Option<String>,
    archived: Option<bool>,
}

impl GitLabRepository {
    fn repo(&self) -> Option<Repo> {
        parse_repository_path(self.path_with_namespace.as_deref())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct GitLabBranch {
    name: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct GitLabCommit {
    id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct GitLabIssue {
    iid: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct GitLabCodeReview {
    iid: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct GitLabRelease {
    tag_name: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct GitLabPipeline {
    id: u64,
}

fn gitlab_repository(response: &Response) -> VcsResult<GitLabRepository> {
    parse_body(response, "invalid gitlab repository response")
}

fn gitlab_repositories(response: &Response) -> VcsResult<Vec<GitLabRepository>> {
    parse_body(response, "invalid gitlab repository list response")
}

fn repository(repository_repo: Repo, repository_response: GitLabRepository) -> Repository {
    repo()
        .owner(repository_repo.owner().as_str())
        .name(repository_repo.name().as_str())
        .provider(PROVIDER_ID)
        .visibility(visibility(repository_response.visibility.as_deref()))
        .lifecycle(lifecycle_state(
            repository_response.archived.unwrap_or_default(),
        ))
        .get()
}

fn visibility(provider_visibility: Option<&str>) -> Visibility {
    match provider_visibility {
        Some("private") => Visibility::Private,
        Some("internal") => Visibility::Internal,
        _ => Visibility::Public,
    }
}

fn lifecycle_state(is_archived: bool) -> LifecycleState {
    if is_archived {
        return LifecycleState::Archived;
    }

    LifecycleState::Active
}

fn parse_repository_path(repository_path: Option<&str>) -> Option<Repo> {
    let (owner_name, repository_name) = repository_path?.rsplit_once('/')?;

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

fn page<T>(items: Vec<T>, response: &Response) -> Page<T> {
    vcs_provider_core::pagination()
        .page(items)
        .optional_next(crate::pagination::next_cursor(response))
        .build()
}
