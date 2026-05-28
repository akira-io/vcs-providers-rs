use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use crate::{VcsResult, error};

#[path = "local_git/clone.rs"]
mod clone;
#[path = "local_git/commands.rs"]
mod commands;
#[path = "local_git/remote.rs"]
mod remote;
#[path = "local_git/url.rs"]
mod url;

pub use clone::{LocalGitCloneBuilder, MissingCloneDestination, ProvidedCloneDestination};
use commands::{git_stdout, git_stdout_optional, run_git};
pub use remote::{
    LocalGitBranch, LocalGitFetchHead, LocalGitRemote, LocalGitRemoteBranch, LocalGitRemoteCommit,
    LocalGitRemoteReference,
};
pub use url::LocalGitUrl;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LocalGitBuilder;

impl LocalGitBuilder {
    pub fn repo(self, path: impl Into<PathBuf>) -> LocalGitRepository {
        LocalGitRepository { path: path.into() }
    }

    pub fn clone_from(
        self,
        source: impl Into<PathBuf>,
    ) -> LocalGitCloneBuilder<MissingCloneDestination> {
        LocalGitCloneBuilder {
            source: source.into(),
            destination: MissingCloneDestination,
        }
    }

    pub fn url(self, url: impl Into<String>) -> LocalGitUrl {
        LocalGitUrl { url: url.into() }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalGitRepository {
    pub(super) path: PathBuf,
}

impl LocalGitRepository {
    pub fn is_repository(&self) -> bool {
        if !self.path.exists() {
            return false;
        }

        run_git(&self.path, ["rev-parse", "--git-dir"]).is_ok()
    }

    pub fn is_valid_clone(&self) -> bool {
        if !self.path.exists() {
            return false;
        }

        run_git(&self.path, ["rev-parse", "HEAD"]).is_ok()
    }

    pub fn name(&self) -> VcsResult<String> {
        let top_level = git_stdout(&self.path, ["rev-parse", "--show-toplevel"])?;
        let name = Path::new(top_level.trim())
            .file_name()
            .and_then(OsStr::to_str)
            .ok_or_else(|| error().invalid_input("missing repository name"))?;

        Ok(name.to_owned())
    }

    pub fn default_branch(&self) -> VcsResult<String> {
        if let Some(branch) = self.origin_head_branch()? {
            return Ok(branch);
        }

        if let Some(branch) = self.current_branch()? {
            return Ok(branch);
        }

        if let Some(branch) = self.remote_head_branch()? {
            return Ok(branch);
        }

        if let Some(branch) = self.local_head_branch()? {
            return Ok(branch);
        }

        Err(error().invalid_input(format!(
            "failed to get default branch for {}",
            self.path.display()
        )))
    }

    pub fn branch(&self, name: impl Into<String>) -> LocalGitBranch {
        LocalGitBranch {
            repository: self.clone(),
            name: name.into(),
        }
    }

    pub fn remote(&self, name: impl Into<String>) -> LocalGitRemote {
        LocalGitRemote {
            repository: self.clone(),
            name: name.into(),
        }
    }

    pub fn fetch_head(&self) -> LocalGitFetchHead {
        LocalGitFetchHead {
            repository: self.clone(),
        }
    }

    fn origin_head_branch(&self) -> VcsResult<Option<String>> {
        let output = git_stdout_optional(
            &self.path,
            ["symbolic-ref", "--short", "refs/remotes/origin/HEAD"],
        )?;
        let Some(raw) = output else {
            return Ok(None);
        };
        let branch = raw.trim().strip_prefix("origin/").unwrap_or(raw.trim());

        Ok(non_empty_branch(branch))
    }

    fn current_branch(&self) -> VcsResult<Option<String>> {
        let output = git_stdout_optional(&self.path, ["rev-parse", "--abbrev-ref", "HEAD"])?;
        let Some(raw) = output else {
            return Ok(None);
        };
        let branch = raw.trim();

        if branch == "HEAD" {
            return Ok(None);
        }

        Ok(non_empty_branch(branch))
    }

    fn remote_head_branch(&self) -> VcsResult<Option<String>> {
        let output = git_stdout_optional(&self.path, ["ls-remote", "--symref", "origin", "HEAD"])?;
        let Some(raw) = output else {
            return Ok(None);
        };

        for line in raw.lines() {
            if let Some(rest) = line.strip_prefix("ref: refs/heads/") {
                return Ok(non_empty_branch(
                    rest.split_whitespace().next().unwrap_or_default(),
                ));
            }
        }

        Ok(None)
    }

    fn local_head_branch(&self) -> VcsResult<Option<String>> {
        let output = git_stdout_optional(&self.path, ["symbolic-ref", "HEAD"])?;
        let Some(raw) = output else {
            return Ok(None);
        };
        let Some(branch) = raw.trim().strip_prefix("refs/heads/") else {
            return Ok(None);
        };

        Ok(non_empty_branch(branch))
    }
}

fn non_empty_branch(branch: &str) -> Option<String> {
    if branch.is_empty() {
        return None;
    }

    Some(branch.to_owned())
}
