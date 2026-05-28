use std::path::PathBuf;
use std::process::Command;

use vcs_provider_core::{VcsResult, error, git};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalGitConflictFixture {
    path: PathBuf,
}

impl LocalGitConflictFixture {
    pub fn repo(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn create(&self) -> VcsResult<(String, String, String)> {
        let repository = git().repo(&self.path);
        let base = repository.reference("HEAD").sha()?;
        self.git().branch("ours").checkout_new()?;
        self.file("README.md").write("ours\n")?;
        self.git().commit().all("test(core): ours")?;
        let ours = repository.reference("HEAD").sha()?;
        self.git().branch("theirs").from(&base).checkout_new()?;
        self.file("README.md").write("theirs\n")?;
        self.git().commit().all("test(core): theirs")?;
        let theirs = repository.reference("HEAD").sha()?;

        Ok((base, ours, theirs))
    }

    fn file(&self, name: impl Into<PathBuf>) -> LocalGitConflictFile {
        LocalGitConflictFile {
            fixture: self.clone(),
            name: name.into(),
        }
    }

    fn git(&self) -> LocalGitConflictCommand {
        LocalGitConflictCommand {
            fixture: self.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct LocalGitConflictFile {
    fixture: LocalGitConflictFixture,
    name: PathBuf,
}

impl LocalGitConflictFile {
    fn write(&self, content: &str) -> VcsResult<()> {
        std::fs::write(self.fixture.path.join(&self.name), content)
            .map_err(|io_error| error().invalid_input(io_error.to_string()))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct LocalGitConflictCommand {
    fixture: LocalGitConflictFixture,
}

impl LocalGitConflictCommand {
    fn branch(&self, name: impl Into<String>) -> LocalGitConflictBranch {
        LocalGitConflictBranch {
            command: self.clone(),
            name: name.into(),
            base: None,
        }
    }

    fn commit(&self) -> LocalGitConflictCommit {
        LocalGitConflictCommit {
            command: self.clone(),
        }
    }

    fn run<const SIZE: usize>(&self, args: [&str; SIZE]) -> VcsResult<()> {
        let output = Command::new("git")
            .current_dir(&self.fixture.path)
            .args(args)
            .output()
            .map_err(|io_error| error().invalid_input(io_error.to_string()))?;

        if output.status.success() {
            return Ok(());
        }

        Err(error().invalid_input(String::from_utf8_lossy(&output.stderr).trim().to_owned()))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct LocalGitConflictBranch {
    command: LocalGitConflictCommand,
    name: String,
    base: Option<String>,
}

impl LocalGitConflictBranch {
    fn from(mut self, base: impl Into<String>) -> Self {
        self.base = Some(base.into());
        self
    }

    fn checkout_new(&self) -> VcsResult<()> {
        if let Some(base) = &self.base {
            return self.command.run(["checkout", "-b", &self.name, base]);
        }

        self.command.run(["checkout", "-b", &self.name])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct LocalGitConflictCommit {
    command: LocalGitConflictCommand,
}

impl LocalGitConflictCommit {
    fn all(&self, message: &str) -> VcsResult<()> {
        self.command.run(["commit", "-am", message])
    }
}

pub fn local_git_conflict(path: impl Into<PathBuf>) -> LocalGitConflictFixture {
    LocalGitConflictFixture::repo(path)
}
