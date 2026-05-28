# git-cognition-github

GitHub provider driver for [`git-cognition`](https://crates.io/crates/git-cognition). Implements
the `Provider` contract from
[`git-cognition-core`](https://crates.io/crates/git-cognition-core) over the GitHub REST API.

Supports GitHub.com and GitHub Enterprise Server.

## Install

```toml
# Direct install (minimal dep tree)
git-cognition-github = "0.1"
git-cognition-core = "0.1"
```

Or via the umbrella with the default feature:

```toml
git-cognition = "0.1"
```

## Usage

```rust
use git_cognition_core::{cognition, http, repo};
use git_cognition_github::github;

let repository = cognition().provider(github())
    .transport(http().transport().get()?)
    .repos()
    .get(repo().owner("akira-io").name("git-cognition-rs").get())
    .await?;
```

GitHub Enterprise:

```rust
let provider = github().base_url("https://github.enterprise.test/api/v3");
```

## Coverage

See
[`docs/23-unified-dev-adapter-coverage.md`](https://github.com/akira-io/git-cognition-rs/blob/main/docs/23-unified-dev-adapter-coverage.md)
for the implemented capabilities (auth, organizations, repositories, branches, issues, code
reviews, pipelines, releases). Gate UI on `Capability` to handle unsupported operations cleanly.

## Versioning

Always ships at the same version as `git-cognition-core` and the other driver crates.

## License

Dual-licensed under MIT or Apache-2.0.
