#[path = "support/local_git_conflict.rs"]
mod local_git_conflict_support;
mod local_git_support;

use git_cognition_core::{
    ChangeKind, CognitionResult, FileState, LineOrigin, LocalGitCapability, cognition,
};

use local_git_conflict_support::local_git_conflict;
use local_git_support::local_git_fixture;

#[test]
fn local_git_exposes_cognition_capabilities() -> CognitionResult<()> {
    let source = local_git_fixture()
        .workspace("capabilities")
        .repo("source")
        .create()?;

    let capabilities = cognition().local().repo(&source).capabilities();

    assert!(capabilities.supports(&LocalGitCapability::Log));
    assert!(capabilities.supports(&LocalGitCapability::Diff));
    assert!(capabilities.supports(&LocalGitCapability::Blame));
    assert!(capabilities.supports(&LocalGitCapability::Worktree));

    Ok(())
}

#[test]
fn local_git_reads_log_graph_and_merge_base() -> CognitionResult<()> {
    let source = local_git_fixture()
        .workspace("log")
        .repo("source")
        .create()?;
    source.branch("feature").commit()?;
    let repository = cognition().local().repo(&source);
    let main = repository.branch("main").sha()?;
    let feature = repository.branch("feature").sha()?;

    let commits = repository
        .log()
        .range()
        .base(&main)
        .head(&feature)
        .commits()?;
    let graph = repository.log().limit(5).graph()?;
    let merge_base = repository
        .merge_base()
        .reference(&main)
        .and(&feature)
        .get()?;
    let commit = repository.commit_meta(&feature)?;

    assert_eq!(commits.len(), 1);
    assert_eq!(commit.id(), feature);
    assert_eq!(merge_base, main);
    assert!(!graph.rows.is_empty());

    Ok(())
}

#[test]
fn local_git_reads_status_show_diff_and_blame() -> CognitionResult<()> {
    let source = local_git_fixture()
        .workspace("reads")
        .repo("source")
        .create()?;
    source.file("README.md").write("test\nchanged\n")?;
    source.file("new.txt").write("new\n")?;
    let repository = cognition().local().repo(&source);

    let status = repository.status()?;
    let show = repository.show("HEAD").file("README.md")?;
    let diff = repository.diff().working().context_lines(1).compute()?;
    let blame = repository.blame("README.md").compute()?;

    assert!(
        status
            .iter()
            .any(|entry| entry.unstaged == FileState::Modified)
    );
    assert!(show.contains("test"));
    assert_eq!(diff.files[0].change, ChangeKind::Modified);
    assert!(
        diff.files[0]
            .hunks
            .iter()
            .flat_map(|hunk| hunk.lines.iter())
            .any(|line| line.origin == LineOrigin::Addition)
    );
    assert_eq!(blame.path.to_string_lossy(), "README.md");
    assert!(!blame.spans.is_empty());

    Ok(())
}

#[test]
fn local_git_manages_ephemeral_worktrees() -> CognitionResult<()> {
    let workspace = local_git_fixture().workspace("worktree");
    let source = workspace.repo("source").create()?;
    let sandbox = workspace.repo("sandbox");
    let sandbox_path = std::path::PathBuf::from(sandbox.clone());
    let repository = cognition().local().repo(&source);
    let head = repository.reference("HEAD").sha()?;
    let worktrees = repository.worktree();

    let worktree = worktrees
        .add(&sandbox)
        .from(&head)
        .detached(true)
        .create()?;
    let list = worktrees.list()?;
    worktrees.remove(sandbox_path.clone())?;
    worktrees.prune()?;

    assert_eq!(worktree.path, sandbox_path);
    assert!(list.len() >= 2);

    Ok(())
}

#[test]
fn local_git_previews_merge_conflicts_without_applying() -> CognitionResult<()> {
    let source = local_git_fixture()
        .workspace("merge")
        .repo("source")
        .create()?;
    let repository = cognition().local().repo(&source);
    let (base, ours, theirs) = local_git_conflict(&source).create()?;

    let preview = repository
        .merge()
        .base(base)
        .ours(ours)
        .theirs(theirs)
        .preview()?;

    assert!(!preview.clean);
    assert!(!preview.conflicts.is_empty());

    Ok(())
}
