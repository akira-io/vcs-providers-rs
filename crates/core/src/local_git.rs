use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use crate::{CognitionResult, error};

#[path = "local_git/blame.rs"]
mod blame;
#[path = "local_git/capabilities.rs"]
mod capabilities;
#[path = "local_git/clone.rs"]
mod clone;
#[path = "local_git/commands.rs"]
mod commands;
#[path = "local_git/diff.rs"]
mod diff;
#[path = "local_git/log.rs"]
mod log;
#[path = "local_git/merge.rs"]
mod merge;
#[path = "local_git/merge_base.rs"]
mod merge_base;
#[path = "local_git/reference.rs"]
mod reference;
#[path = "local_git/remote.rs"]
mod remote;
#[path = "local_git/show.rs"]
mod show;
#[path = "local_git/status.rs"]
mod status;
#[path = "local_git/url.rs"]
mod url;
#[path = "local_git/worktree.rs"]
mod worktree;

pub use blame::{Blame, BlameSpan, LocalGitBlame};
pub use capabilities::{LocalGitCapability, LocalGitCapabilitySet};
pub use clone::{LocalGitCloneBuilder, MissingCloneDestination, ProvidedCloneDestination};
use commands::{git_stdout, git_stdout_optional, run_git};
pub use diff::{ChangeKind, DiffFile, DiffLine, DiffModel, Hunk, LineOrigin, LocalGitDiff};
pub use log::{CommitGraph, GraphRow, LocalGitLog, LocalGitLogRange};
pub use merge::{
    ConflictKind, ConflictRegion, LocalGitMergeBuilder, MergeOutcome, MergePlan, MergePreview,
    MissingBase, MissingOurs, MissingTheirs, ProvidedBase, ProvidedOurs, ProvidedTheirs,
};
pub use merge_base::LocalGitMergeBase;
pub use reference::LocalGitReference;
pub use remote::{
    LocalGitBranch, LocalGitFetchHead, LocalGitRemote, LocalGitRemoteBranch, LocalGitRemoteCommit,
    LocalGitRemoteReference,
};
pub use show::LocalGitShow;
pub use status::{FileState, StatusEntry};
pub use url::LocalGitUrl;
pub use worktree::{LocalGitWorktreeAdd, LocalGitWorktrees, Worktree};

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

    pub fn name(&self) -> CognitionResult<String> {
        let top_level = git_stdout(&self.path, ["rev-parse", "--show-toplevel"])?;
        let name = Path::new(top_level.trim())
            .file_name()
            .and_then(OsStr::to_str)
            .ok_or_else(|| error().invalid_input("missing repository name"))?;

        Ok(name.to_owned())
    }

    pub fn default_branch(&self) -> CognitionResult<String> {
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

    pub fn reference(&self, reference: impl Into<String>) -> LocalGitReference {
        LocalGitReference::make(self.clone(), reference)
    }

    pub fn fetch_head(&self) -> LocalGitFetchHead {
        LocalGitFetchHead {
            repository: self.clone(),
        }
    }

    pub fn capabilities(&self) -> LocalGitCapabilitySet {
        LocalGitCapabilitySet::make([
            LocalGitCapability::Log,
            LocalGitCapability::LogGraph,
            LocalGitCapability::Diff,
            LocalGitCapability::DiffRename,
            LocalGitCapability::Blame,
            LocalGitCapability::MergePreview,
            LocalGitCapability::Worktree,
            LocalGitCapability::Status,
            LocalGitCapability::Show,
            LocalGitCapability::MergeBase,
        ])
    }

    pub fn log(&self) -> LocalGitLog {
        LocalGitLog::make(self.clone())
    }

    pub fn commit_meta(&self, sha: impl Into<String>) -> CognitionResult<crate::Commit> {
        self.reference(sha).commit()
    }

    pub fn merge_base(&self) -> LocalGitMergeBase {
        LocalGitMergeBase::make(self.clone())
    }

    pub fn diff(&self) -> LocalGitDiff {
        LocalGitDiff::make(self.clone())
    }

    pub fn blame(&self, path: impl Into<PathBuf>) -> LocalGitBlame {
        LocalGitBlame::make(self.clone(), path)
    }

    pub fn merge(&self) -> LocalGitMergeBuilder<MissingBase, MissingOurs, MissingTheirs> {
        LocalGitMergeBuilder::make(self.clone())
    }

    pub fn worktree(&self) -> LocalGitWorktrees {
        LocalGitWorktrees::make(self.clone())
    }

    pub fn status(&self) -> CognitionResult<Vec<StatusEntry>> {
        status::status(self)
    }

    pub fn show(&self, revision: impl Into<String>) -> LocalGitShow {
        LocalGitShow::make(self.clone(), revision)
    }

    fn origin_head_branch(&self) -> CognitionResult<Option<String>> {
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

    fn current_branch(&self) -> CognitionResult<Option<String>> {
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

    fn remote_head_branch(&self) -> CognitionResult<Option<String>> {
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

    fn local_head_branch(&self) -> CognitionResult<Option<String>> {
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
