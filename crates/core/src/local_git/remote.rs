use crate::CognitionResult;

use super::LocalGitRepository;
use super::commands::{git_stdout_arguments, git_stdout_optional, run_git};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalGitBranch {
    pub(super) repository: LocalGitRepository,
    pub(super) name: String,
}

impl LocalGitBranch {
    pub fn sha(&self) -> CognitionResult<String> {
        git_stdout_arguments(
            &self.repository.path,
            &["rev-parse".into(), self.name.clone()],
        )
    }

    pub fn create(&self) -> CognitionResult<()> {
        run_git(
            &self.repository.path,
            ["checkout", "-b", self.name.as_str()],
        )
        .map(|_| ())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalGitRemote {
    pub(super) repository: LocalGitRepository,
    pub(super) name: String,
}

impl LocalGitRemote {
    pub fn url(&self) -> Option<String> {
        git_stdout_optional(
            &self.repository.path,
            ["remote", "get-url", self.name.as_str()],
        )
        .ok()
        .flatten()
        .map(|url| url.trim().to_owned())
        .filter(|url| !url.is_empty())
    }

    pub fn set_url(&self, url: impl AsRef<str>) -> CognitionResult<()> {
        run_git(
            &self.repository.path,
            ["remote", "set-url", self.name.as_str(), url.as_ref()],
        )
        .map(|_| ())
    }

    pub fn branch(&self, name: impl Into<String>) -> LocalGitRemoteBranch {
        LocalGitRemoteBranch {
            remote: self.clone(),
            name: name.into(),
        }
    }

    pub fn reference(&self, name: impl Into<String>) -> LocalGitRemoteReference {
        LocalGitRemoteReference {
            remote: self.clone(),
            name: name.into(),
        }
    }

    pub fn commit(&self, sha: impl Into<String>) -> LocalGitRemoteCommit {
        LocalGitRemoteCommit {
            remote: self.clone(),
            sha: sha.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalGitRemoteBranch {
    remote: LocalGitRemote,
    name: String,
}

impl LocalGitRemoteBranch {
    pub fn fetch(&self) -> CognitionResult<()> {
        run_git(
            &self.remote.repository.path,
            ["fetch", self.remote.name.as_str(), self.name.as_str()],
        )
        .map(|_| ())
    }

    pub fn checkout(&self) -> CognitionResult<()> {
        let remote_branch = format!("{}/{}", self.remote.name, self.name);

        run_git(
            &self.remote.repository.path,
            ["checkout", "-B", self.name.as_str(), remote_branch.as_str()],
        )
        .map(|_| ())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalGitRemoteReference {
    remote: LocalGitRemote,
    name: String,
}

impl LocalGitRemoteReference {
    pub fn fetch(&self) -> CognitionResult<()> {
        run_git(
            &self.remote.repository.path,
            ["fetch", self.remote.name.as_str(), self.name.as_str()],
        )
        .map(|_| ())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalGitRemoteCommit {
    remote: LocalGitRemote,
    sha: String,
}

impl LocalGitRemoteCommit {
    pub fn fetch(&self) -> CognitionResult<()> {
        run_git(
            &self.remote.repository.path,
            ["fetch", self.remote.name.as_str(), self.sha.as_str()],
        )
        .map(|_| ())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalGitFetchHead {
    pub(super) repository: LocalGitRepository,
}

impl LocalGitFetchHead {
    pub fn checkout(&self) -> CognitionResult<()> {
        run_git(
            &self.repository.path,
            ["checkout", "--detach", "FETCH_HEAD"],
        )
        .map(|_| ())
    }
}
