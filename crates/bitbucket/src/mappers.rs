use serde::Deserialize;
use vcs_provider_core::{
    Branch, Commit, LifecycleState, Page, Repo, Repository, RepositoryResponseMapper, Response,
    VcsError, VcsResult, Visibility, error, repo,
};

use crate::PROVIDER_ID;

#[derive(Clone, Copy, Debug, Default)]
pub struct BitbucketRepositoryMapper;

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
