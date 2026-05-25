# vcs-providers-rs

Universal async-first VCS provider abstraction layer for Rust.

This repository is the Rust implementation. It is not a GitHub SDK; GitHub, GitLab, Bitbucket, and future providers are driver implementations behind provider-neutral contracts.

## Layout

```text
vcs-providers-rs/
├── docs/
├── crates/
│   ├── core/
│   ├── github/
│   ├── gitlab/
│   └── bitbucket/
├── examples/
└── .github/
```

## Workspace

`vcs-provider-core` owns contracts, resource primitives, capabilities, errors, auth primitives, pagination primitives, transport contracts, middleware contracts, telemetry contracts, and driver registration contracts.

Provider crates implement those contracts. They do not define universal behavior and they do not require changes inside `core` when a new provider is added.

## Development

```sh
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```
