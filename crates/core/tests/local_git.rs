mod local_git_support;

use vcs_provider_core::{VcsResult, git};

use local_git_support::local_git_fixture;

#[test]
fn local_git_reads_repository_metadata() -> VcsResult<()> {
    let source = local_git_fixture()
        .workspace("metadata")
        .repo("source")
        .create()?;

    let repository = git().repo(&source);

    assert!(repository.is_repository());
    assert!(repository.is_valid_clone());
    assert_eq!(repository.name()?, "source");
    assert_eq!(repository.default_branch()?, "main");

    Ok(())
}

#[test]
fn local_git_clones_and_operates_on_remote_refs() -> VcsResult<()> {
    let workspace = local_git_fixture().workspace("clone");
    let source = workspace.repo("source").create()?;
    let destination = workspace.repo("destination");
    source.branch("feature").commit()?;
    let commit_sha = git().repo(&source).branch("main").sha()?;

    git().clone_from(&source).to(&destination).clone()?;

    let repository = git().repo(&destination);
    let origin = repository.remote("origin");

    assert!(repository.is_repository());
    assert!(repository.is_valid_clone());
    assert_eq!(repository.default_branch()?, "main");
    assert_eq!(origin.url(), Some(source.value()));

    repository.branch("local-feature").create()?;
    origin.branch("feature").fetch()?;
    origin.branch("feature").checkout()?;
    origin.reference("refs/heads/main").fetch()?;
    repository.fetch_head().checkout()?;
    origin.commit(commit_sha).fetch()?;
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
