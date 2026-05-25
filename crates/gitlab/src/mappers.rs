use serde::Deserialize;
use vcs_provider_core::{
    Branch, Commit, LifecycleState, Page, Repo, Repository, RepositoryResponseMapper, Response,
    VcsError, VcsResult, Visibility, error, repo,
};

use crate::PROVIDER_ID;

#[derive(Clone, Copy, Debug, Default)]
pub struct GitLabRepositoryMapper;

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

        Ok(Page::make(repositories))
    }

    fn branches(&self, response: &Response) -> VcsResult<Page<Branch>> {
        let branches = parse_body::<Vec<GitLabBranch>>(response, "invalid gitlab branch response")?
            .into_iter()
            .map(|branch| Branch::make(branch.name))
            .collect();

        Ok(Page::make(branches))
    }

    fn commits(&self, response: &Response) -> VcsResult<Page<Commit>> {
        let commits = parse_body::<Vec<GitLabCommit>>(response, "invalid gitlab commit response")?
            .into_iter()
            .map(|commit| Commit::make(commit.id))
            .collect();

        Ok(Page::make(commits))
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
