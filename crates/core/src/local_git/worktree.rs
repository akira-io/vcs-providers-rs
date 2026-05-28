use std::path::PathBuf;

use crate::CognitionResult;

use super::LocalGitRepository;
use super::commands::{git_stdout_arguments, run_git_arguments};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalGitWorktrees {
    repository: LocalGitRepository,
}

impl LocalGitWorktrees {
    pub(super) fn make(repository: LocalGitRepository) -> Self {
        Self { repository }
    }

    pub fn list(&self) -> CognitionResult<Vec<Worktree>> {
        let output = git_stdout_arguments(
            &self.repository.path,
            &["worktree".into(), "list".into(), "--porcelain".into()],
        )?;

        Ok(worktrees(&output))
    }

    pub fn add(&self, path: impl Into<PathBuf>) -> LocalGitWorktreeAdd {
        LocalGitWorktreeAdd {
            repository: self.repository.clone(),
            path: path.into(),
            reference: "HEAD".into(),
            detached: false,
        }
    }

    pub fn remove(&self, path: impl Into<PathBuf>) -> CognitionResult<()> {
        run_git_arguments(
            &self.repository.path,
            &[
                "worktree".into(),
                "remove".into(),
                path.into().to_string_lossy().to_string(),
            ],
        )?;

        Ok(())
    }

    pub fn prune(&self) -> CognitionResult<()> {
        run_git_arguments(&self.repository.path, &["worktree".into(), "prune".into()])?;

        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalGitWorktreeAdd {
    repository: LocalGitRepository,
    path: PathBuf,
    reference: String,
    detached: bool,
}

impl LocalGitWorktreeAdd {
    pub fn from(mut self, reference: impl Into<String>) -> Self {
        self.reference = reference.into();
        self
    }

    pub fn detached(mut self, detached: bool) -> Self {
        self.detached = detached;
        self
    }

    pub fn create(self) -> CognitionResult<Worktree> {
        run_git_arguments(&self.repository.path, &self.arguments())?;

        Ok(Worktree {
            path: self.path,
            branch: None,
            head: self.reference,
            locked: false,
        })
    }

    fn arguments(&self) -> Vec<String> {
        let mut arguments = vec!["worktree".into(), "add".into()];

        if self.detached {
            arguments.push("--detach".into());
        }

        arguments.push(self.path.to_string_lossy().to_string());
        arguments.push(self.reference.clone());
        arguments
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Worktree {
    pub path: PathBuf,
    pub branch: Option<String>,
    pub head: String,
    pub locked: bool,
}

fn worktrees(output: &str) -> Vec<Worktree> {
    let mut worktrees = Vec::new();
    let mut current = None;

    for line in output.lines() {
        if line.is_empty() {
            push_current(&mut worktrees, &mut current);
            continue;
        }

        parse_line(line, &mut current);
    }

    push_current(&mut worktrees, &mut current);
    worktrees
}

fn push_current(worktrees: &mut Vec<Worktree>, current: &mut Option<Worktree>) {
    if let Some(worktree) = current.take() {
        worktrees.push(worktree);
    }
}

fn parse_line(line: &str, current: &mut Option<Worktree>) {
    if let Some(path) = line.strip_prefix("worktree ") {
        *current = Some(Worktree {
            path: PathBuf::from(path),
            branch: None,
            head: String::new(),
            locked: false,
        });
        return;
    }

    let Some(worktree) = current.as_mut() else {
        return;
    };

    if let Some(head) = line.strip_prefix("HEAD ") {
        worktree.head = head.to_owned();
        return;
    }

    if let Some(branch) = line.strip_prefix("branch refs/heads/") {
        worktree.branch = Some(branch.to_owned());
        return;
    }

    if line == "locked" {
        worktree.locked = true;
    }
}
