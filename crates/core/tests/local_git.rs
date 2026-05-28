use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use vcs_provider_core::{VcsResult, error, git};

#[test]
fn local_git_reads_repository_metadata() -> VcsResult<()> {
    let workspace = TestWorkspace::make("metadata")?;
    let source = workspace.path("source");
    create_repository(&source)?;

    let repository = git().repo(&source);

    assert!(repository.is_repository());
    assert!(repository.is_valid_clone());
    assert_eq!(repository.name()?, "source");
    assert_eq!(repository.default_branch()?, "main");

    Ok(())
}

#[test]
fn local_git_clones_and_operates_on_remote_refs() -> VcsResult<()> {
    let workspace = TestWorkspace::make("clone")?;
    let source = workspace.path("source");
    let destination = workspace.path("destination");
    create_repository(&source)?;
    create_branch_with_commit(&source, "feature")?;
    let commit_sha = git_stdout(&source, ["rev-parse", "main"])?;

    git().clone_from(&source).to(&destination).clone()?;

    let repository = git().repo(&destination);
    let origin = repository.remote("origin");

    assert!(repository.is_repository());
    assert!(repository.is_valid_clone());
    assert_eq!(repository.default_branch()?, "main");
    assert_eq!(origin.url(), Some(source.to_string_lossy().to_string()));

    repository.branch("local-feature").create()?;
    origin.branch("feature").fetch()?;
    origin.branch("feature").checkout()?;
    origin.reference("refs/heads/main").fetch()?;
    repository.fetch_head().checkout()?;
    origin.commit(commit_sha.trim()).fetch()?;
    origin.set_url("https://example.test/akira-io/vcs-providers-rs.git")?;

    assert_eq!(
        repository.remote("origin").url(),
        Some("https://example.test/akira-io/vcs-providers-rs.git".into())
    );

    Ok(())
}

#[test]
fn local_git_parses_repository_urls() {
    let repository_url = git().url("https://github.com/akira-io/vcs-providers-rs.git");

    assert!(repository_url.is_github());
    assert_eq!(repository_url.repo_name(), Some("vcs-providers-rs".into()));
}

struct TestWorkspace {
    root: PathBuf,
}

impl TestWorkspace {
    fn make(name: &str) -> VcsResult<Self> {
        let root = std::env::temp_dir().join(format!(
            "vcs-provider-local-git-{}-{}-{}",
            name,
            std::process::id(),
            unique_suffix()?
        ));
        fs::create_dir_all(&root)
            .map_err(|io_error| error().invalid_input(io_error.to_string()))?;

        Ok(Self { root })
    }

    fn path(&self, name: &str) -> PathBuf {
        self.root.join(name)
    }
}

impl Drop for TestWorkspace {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn create_repository(path: &Path) -> VcsResult<()> {
    fs::create_dir_all(path).map_err(|io_error| error().invalid_input(io_error.to_string()))?;
    git_command(path, ["init"])?;
    git_command(path, ["config", "core.hooksPath", "/dev/null"])?;
    git_command(path, ["checkout", "-b", "main"])?;
    git_command(path, ["config", "user.email", "tests@example.test"])?;
    git_command(path, ["config", "user.name", "Tests"])?;
    fs::write(path.join("README.md"), "test\n")
        .map_err(|io_error| error().invalid_input(io_error.to_string()))?;
    git_command(path, ["add", "README.md"])?;
    git_command(path, ["commit", "-m", "test(core): initial"])?;

    Ok(())
}

fn create_branch_with_commit(path: &Path, branch: &str) -> VcsResult<()> {
    git_command(path, ["checkout", "-b", branch])?;
    fs::write(path.join("feature.txt"), "feature\n")
        .map_err(|io_error| error().invalid_input(io_error.to_string()))?;
    git_command(path, ["add", "feature.txt"])?;
    git_command(path, ["commit", "-m", "test(core): feature"])?;
    git_command(path, ["checkout", "main"])?;

    Ok(())
}

fn git_stdout<const SIZE: usize>(path: &Path, args: [&str; SIZE]) -> VcsResult<String> {
    let output = git_command(path, args)?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn git_command<const SIZE: usize>(
    path: &Path,
    args: [&str; SIZE],
) -> VcsResult<std::process::Output> {
    let output = Command::new("git")
        .current_dir(path)
        .args(args)
        .output()
        .map_err(|io_error| error().invalid_input(io_error.to_string()))?;

    if output.status.success() {
        return Ok(output);
    }

    Err(error().invalid_input(String::from_utf8_lossy(&output.stderr).trim().to_owned()))
}

fn unique_suffix() -> VcsResult<u128> {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|time_error| error().invalid_input(time_error.to_string()))?;

    Ok(duration.as_nanos())
}
