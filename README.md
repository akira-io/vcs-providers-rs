# git-cognition-rs

Universal VCS provider contracts and a local Git read surface for Rust, in one crate.

This repository is the Rust implementation. It is not a GitHub SDK; GitHub, GitLab, Bitbucket, and
future providers are driver modules behind provider-neutral contracts, gated by feature flags.

## Layout

```text
git-cognition-rs/
├── docs/
├── src/
│   ├── lib.rs                # crate root: cognition(), Provider, types, local_git, ...
│   ├── github/               # #[cfg(feature = "github")]    (default)
│   ├── gitlab/               # #[cfg(feature = "gitlab")]
│   ├── bitbucket/            # #[cfg(feature = "bitbucket")]
│   └── local_git/            # local Git read surface (always available)
├── tests/                    # integration tests
├── Cargo.toml
└── .github/
```

## Install

From the CLI:

```sh
# Default: GitHub provider + local Git plane
cargo add git-cognition

# GitLab only
cargo add git-cognition --no-default-features --features gitlab

# All providers
cargo add git-cognition --features all

# Local Git plane only, no remote provider
cargo add git-cognition --no-default-features
```

Or by hand in `Cargo.toml`:

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

## Crate surface

`git-cognition` owns contracts, resource primitives, capabilities, errors, auth primitives,
pagination primitives, transport contracts, middleware contracts, telemetry contracts, and driver
registration contracts. Driver modules implement those contracts. Adding a new provider goes in as
a new module behind a new feature flag; the core surface stays untouched.

## Local Git Requirement

Local cognition APIs shell out to `git`. Merge preview requires Git 2.38 or newer because it uses
`git merge-tree --write-tree`. Use Git 2.50.1 or newer in CI and development to match the tested
environment.

## Development

```sh
cargo fmt --check
cargo test --all-features
cargo clippy --all-targets --all-features -- -D warnings
```
