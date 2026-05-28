# git-cognition-bitbucket

Bitbucket Cloud provider driver for
[`git-cognition`](https://crates.io/crates/git-cognition). Implements the `Provider` contract from
[`git-cognition-core`](https://crates.io/crates/git-cognition-core) over the Bitbucket Cloud REST
API.

## Install

```toml
# Direct install (minimal dep tree)
git-cognition-bitbucket = "0.1"
git-cognition-core = "0.1"
```

Or via the umbrella with the `bitbucket` feature:

```toml
git-cognition = { version = "0.1", default-features = false, features = ["bitbucket"] }
```

## Usage

```rust
use git_cognition_core::{cognition, http, repo};
use git_cognition_bitbucket::bitbucket;

let repository = cognition().provider(bitbucket())
    .transport(http().transport().get()?)
    .repos()
    .get(repo().owner("akira-io").name("git-cognition-rs").get())
    .await?;
```

## Coverage

See
[`docs/23-unified-dev-adapter-coverage.md`](https://github.com/akira-io/git-cognition-rs/blob/main/docs/23-unified-dev-adapter-coverage.md)
for the implemented capabilities. Bitbucket Cloud exposes Downloads rather than a release resource
with equivalent semantics, so the `Releases` contract is unsupported there. Gate UI on `Capability`
to handle unsupported operations cleanly.

## Versioning

Always ships at the same version as `git-cognition-core` and the other driver crates.

## License

Dual-licensed under MIT or Apache-2.0.
