# git-cognition

Universal VCS provider contracts and a local Git read surface for Rust, in one umbrella crate.
Re-exports [`git-cognition-core`](https://crates.io/crates/git-cognition-core) unconditionally and
exposes provider drivers (GitHub, GitLab, Bitbucket) behind feature flags.

## Install

```toml
# Default: GitHub provider + local Git plane
git-cognition = "0.1"

# GitLab only
git-cognition = { version = "0.1", default-features = false, features = ["gitlab"] }

# All providers
git-cognition = { version = "0.1", features = ["all"] }

# Local Git plane only, no remote provider
git-cognition = { version = "0.1", default-features = false }
```

Or pull a single driver crate directly when umbrella indirection is not wanted:

```toml
git-cognition-gitlab = "0.1"
```

## Features

| Feature             | Pulls                          | Use case                            |
| ------------------- | ------------------------------ | ----------------------------------- |
| `github` (default)  | `git-cognition-github`         | GitHub.com + GitHub Enterprise      |
| `gitlab`            | `git-cognition-gitlab`         | GitLab.com + self-managed           |
| `bitbucket`         | `git-cognition-bitbucket`      | Bitbucket Cloud                     |
| `all`               | github + gitlab + bitbucket    | Every provider                      |
| `testing`           | `git-cognition-core/testing`   | Conformance fixtures                |

`git-cognition-core` is always included. The local Git plane (`cognition().local()`) needs no feature.

## Quickstart

```rust
use git_cognition::{cognition, http, repo};
use git_cognition::github::github;

let repository = cognition().provider(github())
    .transport(http().transport().get()?)
    .repos()
    .get(repo().owner("akira-io").name("git-cognition-rs").get())
    .await?;

let graph = cognition().local()
    .repo("/workspace/git-cognition-rs")
    .log()
    .limit(50)
    .graph()?;
```

See [the documentation](https://github.com/akira-io/git-cognition-rs/tree/main/docs) for the full
surface (log, diff, blame, merge preview, worktree, status, plus the multi-provider end-to-end flow).

## Versioning

All crates in this workspace ship at the same version on every release. `git-cognition 0.1.x` always
pulls `git-cognition-core 0.1.x` and each enabled driver at `0.1.x`.

## License

Dual-licensed under MIT or Apache-2.0.
