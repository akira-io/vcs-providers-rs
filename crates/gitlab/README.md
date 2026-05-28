# git-cognition-gitlab

GitLab provider driver for [`git-cognition`](https://crates.io/crates/git-cognition). Implements
the `Provider` contract from
[`git-cognition-core`](https://crates.io/crates/git-cognition-core) over the GitLab REST API.

Supports GitLab.com and self-managed GitLab instances.

## Install

```toml
# Direct install (minimal dep tree)
git-cognition-gitlab = "0.1"
git-cognition-core = "0.1"
```

Or via the umbrella with the `gitlab` feature:

```toml
git-cognition = { version = "0.1", default-features = false, features = ["gitlab"] }
```

## Usage

```rust
use git_cognition_core::{cognition, http, repo};
use git_cognition_gitlab::gitlab;

let repository = cognition().provider(gitlab())
    .transport(http().transport().get()?)
    .repos()
    .get(repo().owner("akira-io").name("git-cognition-rs").get())
    .await?;
```

Self-managed instance:

```rust
let provider = gitlab().base_url("https://gitlab.internal.example");
```

The driver builds REST paths under `/api/v4` automatically.

## Coverage

See
[`docs/23-unified-dev-adapter-coverage.md`](https://github.com/akira-io/git-cognition-rs/blob/main/docs/23-unified-dev-adapter-coverage.md)
for the implemented capabilities. Gate UI on `Capability` to handle unsupported operations cleanly.

## Versioning

Always ships at the same version as `git-cognition-core` and the other driver crates.

## License

Dual-licensed under MIT or Apache-2.0.
