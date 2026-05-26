use serde::Deserialize;
use vcs_provider_core::{
    Branch, CodeReview, CodeReviewId, CodeReviewResponseMapper, Commit, LifecycleState, Page, Repo,
    Repository, RepositoryResponseMapper, Response, VcsError, VcsResult, Visibility, error,
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

impl RepositoryResponseMapper for BitbucketRepositoryMapper {
    fn repository(&self, requested_repo: &Repo, response: &Response) -> VcsResult<Repository> {
        let repository_response = bitbucket_repository(response)?;
        let repository_repo = repository_response
            .repo()
            .unwrap_or_else(|| requested_repo.clone());

        Ok(repository(repository_repo, repository_response))
    }

    fn repositories(&self, response: &Response) -> VcsResult<Page<Repository>> {
        let repositories = bitbucket_repositories(response)?
            .values
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
        let branches = parse_body::<BitbucketPage<BitbucketBranch>>(
            response,
            "invalid bitbucket branch response",
        )?
        .values
        .into_iter()
        .map(|branch| Branch::make(branch.name))
        .collect();

        Ok(Page::make(branches))
    }

    fn commits(&self, response: &Response) -> VcsResult<Page<Commit>> {
        let commits = parse_body::<BitbucketPage<BitbucketCommit>>(
            response,
            "invalid bitbucket commit response",
        )?
        .values
        .into_iter()
        .map(|commit| Commit::make(commit.hash))
        .collect();

        Ok(Page::make(commits))
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

        Ok(CodeReview::make(
            requested_code_review.repo().clone(),
            CodeReviewId::make(code_review.id.to_string()),
        ))
    }

    fn code_reviews(
        &self,
        requested_repo: &Repo,
        response: &Response,
    ) -> VcsResult<Page<CodeReview>> {
        let code_reviews = parse_body::<BitbucketPage<BitbucketCodeReview>>(
            response,
            "invalid bitbucket code review list response",
        )?
        .values
        .into_iter()
        .map(|code_review| {
            CodeReview::make(
                requested_repo.clone(),
                CodeReviewId::make(code_review.id.to_string()),
            )
        })
        .collect();

        Ok(Page::make(code_reviews))
    }
}

impl PipelineResponseMapper for BitbucketPipelineMapper {
    fn pipeline(&self, requested_pipeline: &Pipeline, response: &Response) -> VcsResult<Pipeline> {
        let pipeline =
            parse_body::<BitbucketPipeline>(response, "invalid bitbucket pipeline response")?;

        Ok(vcs_provider_core::Pipeline::make(
            requested_pipeline.repo().clone(),
            vcs_provider_core::PipelineId::make(pipeline.uuid),
        ))
    }

    fn pipelines(&self, requested_repo: &Repo, response: &Response) -> VcsResult<Page<Pipeline>> {
        let pipelines = parse_body::<BitbucketPage<BitbucketPipeline>>(
            response,
            "invalid bitbucket pipeline list response",
        )?
        .values
        .into_iter()
        .map(|pipeline_response| {
            pipeline()
                .repo(requested_repo.clone())
                .id(pipeline_response.uuid)
                .get()
        })
        .collect();

        Ok(Page::make(pipelines))
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
struct BitbucketPage<T> {
    values: Vec<T>,
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
struct BitbucketPipeline {
    uuid: String,
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
