use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use vcs_provider_core::{VcsResult, error};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LocalGitFixtureBuilder;

impl LocalGitFixtureBuilder {
    pub fn workspace(self, name: impl Into<String>) -> LocalGitFixtureWorkspace {
        LocalGitFixtureWorkspace {
            root: Arc::new(LocalGitFixtureRoot {
                path: fixture_root(name),
            }),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalGitFixtureWorkspace {
    root: Arc<LocalGitFixtureRoot>,
}

impl LocalGitFixtureWorkspace {
    pub fn repo(&self, name: impl Into<String>) -> LocalGitFixtureRepository {
        LocalGitFixtureRepository {
            root: Arc::clone(&self.root),
            path: self.root.path.join(name.into()),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct LocalGitFixtureRoot {
    path: PathBuf,
}

impl Drop for LocalGitFixtureRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalGitFixtureRepository {
    root: Arc<LocalGitFixtureRoot>,
    path: PathBuf,
}

impl LocalGitFixtureRepository {
    pub fn create(&self) -> VcsResult<Self> {
        fs::create_dir_all(&self.path)
            .map_err(|io_error| error().invalid_input(io_error.to_string()))?;
        self.git().init()?;
        self.git().config().hooks_path("/dev/null")?;
        self.git().branch("main").checkout_new()?;
        self.git().config().email("tests@example.test")?;
        self.git().config().user_name("Tests")?;
        self.file("README.md").write("test\n")?;
        self.git().add("README.md")?;
        self.git().commit().message("test(core): initial")?;

        Ok(self.clone())
    }

    pub fn branch(&self, name: impl Into<String>) -> LocalGitFixtureBranch {
        LocalGitFixtureBranch {
            repository: self.clone(),
            name: name.into(),
        }
    }

    pub fn file(&self, name: impl Into<PathBuf>) -> LocalGitFixtureFile {
        LocalGitFixtureFile {
            repository: self.clone(),
            name: name.into(),
        }
    }

    pub fn value(&self) -> String {
        self.path.to_string_lossy().to_string()
    }

    fn git(&self) -> LocalGitFixtureCommand {
        LocalGitFixtureCommand {
            repository: self.clone(),
        }
    }
}

impl From<LocalGitFixtureRepository> for PathBuf {
    fn from(repository: LocalGitFixtureRepository) -> Self {
        repository.path
    }
}

impl From<&LocalGitFixtureRepository> for PathBuf {
    fn from(repository: &LocalGitFixtureRepository) -> Self {
        repository.path.clone()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalGitFixtureBranch {
    repository: LocalGitFixtureRepository,
    name: String,
}

impl LocalGitFixtureBranch {
    pub fn commit(&self) -> VcsResult<()> {
        self.repository.git().branch(&self.name).checkout_new()?;
        self.repository.file("feature.txt").write("feature\n")?;
        self.repository.git().add("feature.txt")?;
        self.repository
            .git()
            .commit()
            .message("test(core): feature")?;
        self.repository.git().branch("main").checkout()?;

        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalGitFixtureFile {
    repository: LocalGitFixtureRepository,
    name: PathBuf,
}

impl LocalGitFixtureFile {
    pub fn write(&self, content: &str) -> VcsResult<()> {
        fs::write(self.repository.path.join(&self.name), content)
            .map_err(|io_error| error().invalid_input(io_error.to_string()))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct LocalGitFixtureCommand {
    repository: LocalGitFixtureRepository,
}

impl LocalGitFixtureCommand {
    fn init(&self) -> VcsResult<()> {
        self.run(["init"])
    }

    fn config(&self) -> LocalGitFixtureConfig {
        LocalGitFixtureConfig {
            command: self.clone(),
        }
    }

    fn branch(&self, name: impl Into<String>) -> LocalGitFixtureGitBranch {
        LocalGitFixtureGitBranch {
            command: self.clone(),
            name: name.into(),
        }
    }

    fn add(&self, path: &str) -> VcsResult<()> {
        self.run(["add", path])
    }

    fn commit(&self) -> LocalGitFixtureCommit {
        LocalGitFixtureCommit {
            command: self.clone(),
        }
    }

    fn run<const SIZE: usize>(&self, args: [&str; SIZE]) -> VcsResult<()> {
        let output = Command::new("git")
            .current_dir(&self.repository.path)
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
struct LocalGitFixtureConfig {
    command: LocalGitFixtureCommand,
}

impl LocalGitFixtureConfig {
    fn hooks_path(&self, path: &str) -> VcsResult<()> {
        self.command.run(["config", "core.hooksPath", path])
    }

    fn email(&self, email: &str) -> VcsResult<()> {
        self.command.run(["config", "user.email", email])
    }

    fn user_name(&self, name: &str) -> VcsResult<()> {
        self.command.run(["config", "user.name", name])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct LocalGitFixtureGitBranch {
    command: LocalGitFixtureCommand,
    name: String,
}

impl LocalGitFixtureGitBranch {
    fn checkout(&self) -> VcsResult<()> {
        self.command.run(["checkout", &self.name])
    }

    fn checkout_new(&self) -> VcsResult<()> {
        self.command.run(["checkout", "-b", &self.name])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct LocalGitFixtureCommit {
    command: LocalGitFixtureCommand,
}

impl LocalGitFixtureCommit {
    fn message(&self, message: &str) -> VcsResult<()> {
        self.command.run(["commit", "-m", message])
    }
}

pub fn local_git_fixture() -> LocalGitFixtureBuilder {
    LocalGitFixtureBuilder
}

fn fixture_root(name: impl Into<String>) -> PathBuf {
    std::env::temp_dir().join(format!(
        "vcs-provider-local-git-{}-{}-{}",
        name.into(),
        std::process::id(),
        unique_suffix()
    ))
}

fn unique_suffix() -> u128 {
    let Ok(duration) = SystemTime::now().duration_since(UNIX_EPOCH) else {
        return 0;
    };

    duration.as_nanos()
}
