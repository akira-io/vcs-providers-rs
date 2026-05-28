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

A file with multiple conflict regions surfaces every region, each carrying the file path and the
base text from stage 1 when Git recorded a base blob:

```rust
let preview = cognition().local()
    .repo("/workspace/project")
    .merge()
    .base(&base_sha)
    .ours(&ours_sha)
    .theirs(&theirs_sha)
    .preview()?;

for region in &preview.conflicts {
    println!("{} ({:?})", region.path.display(), region.kind);
    println!("ours:\n{}", region.ours);
    println!("theirs:\n{}", region.theirs);
    if let Some(base) = &region.base {
        println!("base:\n{base}");
    }
}
```

Conflict classification is conservative. `ConflictKind::AddAdd` and `DeleteModify` come from Git's
own messages for the path; `Whitespace` and `ImportOrder` are detected from the textual content of
the region. Everything else stays `Overlap`; callers that need richer semantic classification
should layer their own analysis above the returned regions.

## Sandbox A Merge Preview In A Worktree

Worktrees give you an ephemeral checkout so a merge preview never touches the user's working tree.
Pair `worktree().add()` with `merge().preview()` and reap the sandbox when done:

```rust
let repository = cognition().local().repo("/workspace/project");

let sandbox = repository
    .worktree()
    .add("/tmp/project-merge-preview")
    .from(&ours_sha)
    .detached(true)
    .create()?;

let preview = repository
    .merge()
    .base(&base_sha)
    .ours(&ours_sha)
    .theirs(&theirs_sha)
    .preview()?;

repository.worktree().remove(sandbox.path)?;
repository.worktree().prune()?;

println!("clean: {}, files: {}", preview.clean, preview.merged_files.len());
```

## End-To-End: Review A Branch Locally

A single flow combining log, diff, blame and merge-base to inspect what a feature branch
introduced relative to the trunk:

```rust
use git_cognition_core::{LineOrigin, cognition};

let repository = cognition().local().repo("/workspace/project");

let main = repository.branch("main").sha()?;
let feature = repository.branch("feature").sha()?;
let merge_base = repository.merge_base().reference(&main).and(&feature).get()?;

let commits = repository
    .log()
    .range()
    .base(&merge_base)
    .head(&feature)
    .limit(50)
    .commits()?;

let diff = repository
    .diff()
    .range(&merge_base, &feature)
    .detect_renames(true)
    .compute()?;

for file in &diff.files {
    let path = file.new_path.as_deref().or(file.old_path.as_deref());
    let additions = file
        .hunks
        .iter()
        .flat_map(|hunk| hunk.lines.iter())
        .filter(|line| line.origin == LineOrigin::Addition)
        .count();

    println!("{:?} +{additions} ({:?})", path, file.change);
}

if let Some(top_file) = diff.files.first() {
    if let Some(path) = top_file.new_path.as_ref() {
        let blame = repository.blame(path).at(&feature).compute()?;
        for span in blame.spans.iter().take(3) {
            println!("{} {} L{}-{}", span.commit, span.author, span.start_line, span.start_line + span.line_count - 1);
        }
    }
}

println!("{} commits ahead", commits.len());
```

## `commits()` Versus `commit_meta()`

`log().commits()` is a cheap history scan: it returns `Commit` resources populated with the
identifier only, suitable for counting, paginating, or feeding back into other operations. When you
need the author, message and timestamps for a specific commit, hydrate it with `commit_meta()`:

```rust
let repository = cognition().local().repo("/workspace/project");

let ids = repository.log().limit(100).commits()?;          // cheap: only ids
let head = ids.first().map(Commit::id).unwrap_or_default();
let full = repository.commit_meta(head)?;                   // full hydration

println!("{} by {}", full.message(), full.author());
```

`graph()` returns the same `RawGraphRow` topology hydrated as `Commit`-by-id plus precomputed lane,
parent shas and ref tips for rendering.

## Capability Gating

`capabilities()` is intentionally static today: it advertises every surface this crate implements
against any supported `git` install. Callers that want to gate UI on real availability should still
check the set so future drivers (for example a `gix`-backed engine) can advertise extra capabilities
without breaking consumers:

```rust
let capabilities = cognition().local().repo("/workspace/project").capabilities();

if capabilities.supports(&LocalGitCapability::MergePreview) {
    // show merge preview affordance
}

if !capabilities.supports(&LocalGitCapability::Blame) {
    // hide blame UI
}
```

`DiffRename` is advertised but only activates when callers opt in with
`diff().detect_renames(true)`. The default keeps `git diff` rename detection off to match Git's own
default.
