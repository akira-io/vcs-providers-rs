use git_cognition_core::{Repo, repo};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub(super) struct GitHubRepository {
    pub(super) full_name: Option<String>,
    pub(super) private: Option<bool>,
    pub(super) archived: Option<bool>,
    pub(super) disabled: Option<bool>,
}

impl GitHubRepository {
    pub(super) fn repo(&self) -> Option<Repo> {
        parse_repository_path(self.full_name.as_deref())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub(super) struct GitHubBranch {
    pub(super) name: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub(super) struct GitHubReference {
    #[serde(rename = "ref")]
    reference: String,
}

impl GitHubReference {
    pub(super) fn name(&self) -> &str {
        self.reference
            .strip_prefix("refs/heads/")
            .unwrap_or(self.reference.as_str())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub(super) struct GitHubOrganization {
    pub(super) id: u64,
    pub(super) login: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub(super) struct GitHubCommit {
    pub(super) sha: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub(super) struct GitHubIssue {
    pub(super) number: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub(super) struct GitHubCodeReview {
    pub(super) number: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub(super) struct GitHubRelease {
    pub(super) id: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub(super) struct GitHubPipelinePage {
    pub(super) workflow_runs: Vec<GitHubPipeline>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub(super) struct GitHubPipeline {
    pub(super) id: u64,
}

fn parse_repository_path(repository_path: Option<&str>) -> Option<Repo> {
    let (owner_name, repository_name) = repository_path?.split_once('/')?;

    Some(repo().owner(owner_name).name(repository_name).get())
}
