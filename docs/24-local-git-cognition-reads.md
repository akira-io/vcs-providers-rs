# Local Git Cognition Reads

`git-cognition-core::cognition().local()` exposes local Git operations separately from HTTP providers. Use this
surface for repository cognition features that need commit history, diffs, blame, worktrees, status,
or merge previews.

This API does not use provider drivers, HTTP transport, auth middleware, or provider capabilities.
It shells out to the local `git` binary and returns normalized Rust resources. Merge preview requires Git 2.38 or newer because it uses `git merge-tree --write-tree`; the tested development version is Git 2.50.1.

## Repository

```rust
use git_cognition_core::cognition;

let repository = cognition().local().repo("/workspace/project");

let name = repository.name()?;
let default_branch = repository.default_branch()?;
let is_clone = repository.is_valid_clone();
```

## Capabilities

Local capabilities are separate from remote provider capabilities.

```rust
use git_cognition_core::{cognition, LocalGitCapability};

let capabilities = cognition().local().repo("/workspace/project").capabilities();

assert!(capabilities.supports(&LocalGitCapability::Diff));
assert!(capabilities.supports(&LocalGitCapability::Blame));
```

## Log And Graph

```rust
let repository = cognition().local().repo("/workspace/project");

let main = repository.branch("main").sha()?;
let feature = repository.branch("feature").sha()?;

let commits = repository
    .log()
    .range()
    .base(&main)
    .head(&feature)
    .limit(50)
    .commits()?;

let commit = repository.commit_meta(&feature)?;

let graph = repository
    .log()
    .since_ref("main")
    .graph()?;

let base = repository
    .merge_base()
    .reference(&main)
    .and(&feature)
    .get()?;
```

`commits()` returns commit identifiers for cheap history scans. Use `commit_meta()` when author, committer, message and timestamps are needed. `graph()` computes lane positions from parent topology for rendering.

## Diff

```rust
let diff = cognition().local()
    .repo("/workspace/project")
    .diff()
    .working()
    .detect_renames(true)
    .context_lines(3)
    .compute()?;
```

The returned `DiffModel` contains files, hunks, line origins, additions, deletions, binary markers,
and provider-neutral change kinds. Addition and deletion counts include hunk body lines only, not file headers.

## Blame

```rust
let blame = cognition().local()
    .repo("/workspace/project")
    .blame("src/lib.rs")
    .at("HEAD")
    .compute()?;
```

## Status And Show

```rust
let repository = cognition().local().repo("/workspace/project");

let status = repository.status()?;
let file = repository.show("HEAD").file("README.md")?;
```

`status()` uses porcelain output and returns normalized staged and unstaged file states.

## Worktrees

```rust
let repository = cognition().local().repo("/workspace/project");

let worktree = repository
    .worktree()
    .add("/tmp/project-preview")
    .from("HEAD")
    .detached(true)
    .create()?;

repository.worktree().remove(worktree.path)?;
repository.worktree().prune()?;
```

Use worktrees for sandboxed previews and dry-runs. Do not mutate the user's active working tree for
analysis.

## Merge Preview

```rust
let preview = cognition().local()
    .repo("/workspace/project")
    .merge()
    .base("base-sha")
    .ours("ours-sha")
    .theirs("theirs-sha")
    .preview()?;
```

`preview()` does not apply the merge. It returns whether the merge is clean, the files touched by Git's merge machinery, and conflict regions with paths plus base, ours and theirs content when Git reports conflicts. Applying a merge is intentionally separate from previewing it. `MergeApply` is not advertised until recovery refs and `MergePlan` are implemented.
