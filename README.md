# git-cognition-rs

Universal async-first VCS provider abstraction layer for Rust.

This repository is the Rust implementation. It is not a GitHub SDK; GitHub, GitLab, Bitbucket, and future providers are driver implementations behind provider-neutral contracts.

## Layout

```text
git-cognition-rs/
├── docs/
├── crates/
│   ├── core/
│   ├── github/
│   ├── gitlab/
│   └── bitbucket/
└── .github/
```

## Workspace

`git-cognition-core` owns contracts, resource primitives, capabilities, errors, auth primitives, pagination primitives, transport contracts, middleware contracts, telemetry contracts, and driver registration contracts.

Provider crates implement those contracts. They do not define universal behavior and they do not require changes inside `core` when a new provider is added.

## Local Git Requirement

Local cognition APIs shell out to `git`. Merge preview requires Git 2.38 or newer because it uses `git merge-tree --write-tree`. Use Git 2.50.1 or newer in CI and development to match the tested environment.

## Development

```sh
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```
