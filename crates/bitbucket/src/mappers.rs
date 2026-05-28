use serde::Deserialize;
use vcs_provider_core::{
    Branch, CodeReview, CodeReviewResponseMapper, Commit, Issue, IssueResponseMapper,
    LifecycleState, Organization, OrganizationKind, OrganizationResponseMapper, Page, Repo,
    Repository, RepositoryResponseMapper, Response, VcsError, VcsResult, Visibility, error, issue,
    pipeline, repo,
};
use vcs_provider_core::{Pipeline, PipelineResponseMapper};

use crate::PROVIDER_ID;

#[derive(Clone, Copy, Debug, Default)]
pub struct BitbucketRepositoryMapper;

#[derive(Clone, Copy, Debug, Default)]
pub struct BitbucketCodeReviewMapper;

#[derive(Clone, Copy, Debug, Default)]
pub struct BitbucketPipelineMapper;

#[derive(Clone, Copy, Debug, Default)]
pub struct BitbucketIssueMapper;

#[derive(Clone, Copy, Debug, Default)]
pub struct BitbucketOrganizationMapper;

impl RepositoryResponseMapper for BitbucketRepositoryMapper {
    fn repository(&self, requested_repo: &Repo, response: &Response) -> VcsResult<Repository> {
        let repository_response = bitbucket_repository(response)?;
        let repository_repo = repository_response
            .repo()
            .unwrap_or_else(|| requested_repo.clone());

        Ok(repository(repository_repo, repository_response))
    }

    fn repositories(&self, response: &Response) -> VcsResult<Page<Repository>> {
        let provider_page = bitbucket_repositories(response)?;
        let repositories = provider_page
            .values
            .into_iter()
            .filter_map(|repository_response| {
                repository_response
                    .repo()
                    .map(|repository_repo| repository(repository_repo, repository_response))
            })
            .collect();

        Ok(page(repositories, provider_page.next.as_deref()))
    }

    fn branches(&self, response: &Response) -> VcsResult<Page<Branch>> {
        let provider_page = parse_body::<BitbucketPage<BitbucketBranch>>(
            response,
            "invalid bitbucket branch response",
        )?;
        let branches = provider_page
            .values
            .into_iter()
            .map(|branch| Branch::make(branch.name))
            .collect();

        Ok(page(branches, provider_page.next.as_deref()))
    }

    fn branch(&self, response: &Response) -> VcsResult<Branch> {
        let branch = parse_body::<BitbucketBranch>(response, "invalid bitbucket branch response")?;

        Ok(Branch::make(branch.name))
    }

    fn commits(&self, response: &Response) -> VcsResult<Page<Commit>> {
        let provider_page = parse_body::<BitbucketPage<BitbucketCommit>>(
            response,
            "invalid bitbucket commit response",
        )?;
        let commits = provider_page
            .values
            .into_iter()
            .map(|commit| Commit::make(commit.hash))
            .collect();

        Ok(page(commits, provider_page.next.as_deref()))
    }
}

impl CodeReviewResponseMapper for BitbucketCodeReviewMapper {
    fn code_review(
        &self,
        requested_code_review: &CodeReview,
        response: &Response,
    ) -> VcsResult<CodeReview> {
        let code_review =
            parse_body::<BitbucketCodeReview>(response, "invalid bitbucket code review response")?;

        Ok(vcs_provider_core::code_review()
            .repo(requested_code_review.repo().clone())
            .id(code_review.id.to_string())
            .get())
    }

    fn code_reviews(
        &self,
        requested_repo: &Repo,
        response: &Response,
    ) -> VcsResult<Page<CodeReview>> {
        let provider_page = parse_body::<BitbucketPage<BitbucketCodeReview>>(
            response,
            "invalid bitbucket code review list response",
        )?;
        let code_reviews = provider_page
            .values
            .into_iter()
            .map(|code_review| {
                vcs_provider_core::code_review()
                    .repo(requested_repo.clone())
                    .id(code_review.id.to_string())
                    .get()
            })
            .collect();

        Ok(page(code_reviews, provider_page.next.as_deref()))
    }
}

impl IssueResponseMapper for BitbucketIssueMapper {
    fn issue(&self, requested_issue: &Issue, response: &Response) -> VcsResult<Issue> {
        let issue_response =
            parse_body::<BitbucketIssue>(response, "invalid bitbucket issue response")?;

        Ok(issue()
            .repo(requested_issue.repo().clone())
            .id(issue_response.id.to_string())
            .get())
    }

    fn issues(&self, requested_repo: &Repo, response: &Response) -> VcsResult<Page<Issue>> {
        let provider_page =
            parse_body::<BitbucketPage<BitbucketIssue>>(response, "invalid bitbucket issue list")?;
        let issues = provider_page
            .values
            .into_iter()
            .map(|issue_response| {
                issue()
                    .repo(requested_repo.clone())
                    .id(issue_response.id.to_string())
                    .get()
            })
            .collect();

        Ok(page(issues, provider_page.next.as_deref()))
    }
}

impl OrganizationResponseMapper for BitbucketOrganizationMapper {
    fn organizations(&self, response: &Response) -> VcsResult<Page<Organization>> {
        let provider_page = parse_body::<BitbucketPage<BitbucketWorkspace>>(
            response,
            "invalid bitbucket workspace response",
        )?;
        let organizations = provider_page
            .values
            .into_iter()
            .map(|workspace| {
                Organization::make(
                    PROVIDER_ID,
                    workspace.uuid,
                    workspace.slug,
                    OrganizationKind::Organization,
                )
            })
            .collect();

        Ok(page(organizations, provider_page.next.as_deref()))
    }
}

impl PipelineResponseMapper for BitbucketPipelineMapper {
    fn pipeline(&self, requested_pipeline: &Pipeline, response: &Response) -> VcsResult<Pipeline> {
        let pipeline =
            parse_body::<BitbucketPipeline>(response, "invalid bitbucket pipeline response")?;

        Ok(vcs_provider_core::pipeline()
            .repo(requested_pipeline.repo().clone())
            .id(pipeline.uuid)
            .get())
    }

    fn pipelines(&self, requested_repo: &Repo, response: &Response) -> VcsResult<Page<Pipeline>> {
        let provider_page = parse_body::<BitbucketPage<BitbucketPipeline>>(
            response,
            "invalid bitbucket pipeline list response",
        )?;
        let pipelines = provider_page
            .values
            .into_iter()
            .map(|pipeline_response| {
                pipeline()
                    .repo(requested_repo.clone())
                    .id(pipeline_response.uuid)
                    .get()
            })
            .collect();

        Ok(page(pipelines, provider_page.next.as_deref()))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct BitbucketPage<T> {
    values: Vec<T>,
    next: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct BitbucketRepository {
    full_name: Option<String>,
    is_private: Option<bool>,
}

impl BitbucketRepository {
    fn repo(&self) -> Option<Repo> {
        parse_repository_path(self.full_name.as_deref())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct BitbucketBranch {
    name: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct BitbucketCommit {
    hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct BitbucketCodeReview {
    id: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct BitbucketIssue {
    id: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct BitbucketPipeline {
    uuid: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct BitbucketWorkspace {
    uuid: String,
    slug: String,
}

fn bitbucket_repository(response: &Response) -> VcsResult<BitbucketRepository> {
    parse_body(response, "invalid bitbucket repository response")
}

fn bitbucket_repositories(response: &Response) -> VcsResult<BitbucketPage<BitbucketRepository>> {
    parse_body(response, "invalid bitbucket repository list response")
}

fn repository(repository_repo: Repo, repository_response: BitbucketRepository) -> Repository {
    repo()
        .owner(repository_repo.owner().as_str())
        .name(repository_repo.name().as_str())
        .provider(PROVIDER_ID)
        .visibility(visibility(
            repository_response.is_private.unwrap_or_default(),
        ))
        .lifecycle(LifecycleState::Active)
        .get()
}

fn visibility(is_private: bool) -> Visibility {
    if is_private {
        return Visibility::Private;
    }

    Visibility::Public
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

fn page<T>(items: Vec<T>, next_url: Option<&str>) -> Page<T> {
    vcs_provider_core::pagination()
        .page(items)
        .optional_next(crate::pagination::next_cursor(next_url))
        .build()
}
