use serde::Deserialize;
use vcs_provider_core::{
    Branch, Commit, LifecycleState, Page, Repo, Repository, RepositoryResponseMapper, Response,
    VcsError, VcsResult, Visibility, error, repo,
};

use crate::PROVIDER_ID;

#[derive(Clone, Copy, Debug, Default)]
pub struct GitHubRepositoryMapper;

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
