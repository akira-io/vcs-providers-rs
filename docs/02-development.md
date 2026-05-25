# Development

Work in focused PRs that introduce complete contracts across all supported providers.

When a new core contract is introduced, the same PR must apply it to GitHub, GitLab, and Bitbucket unless the contract is explicitly provider-specific.

## Required Checks

Run from the repository root:

```sh
cargo fmt --check
cargo test
cargo clippy --all-targets --all-features -- -D warnings
```

## API Rules

- Do not expose HTTP client implementation types publicly.
- Do not expose untyped JSON payloads publicly.
- Do not expose raw provider payloads publicly.
- Do not add provider-specific branches inside core.
- Do not add chained conditional branches.
- Do not add comments that restate obvious code.
- Keep public APIs explicit and typed.

## Provider Work

Provider behavior should be added through driver implementations, adapters, builders, middleware, and contract implementations.

If a new provider needs a different behavior, define the variation as a provider-owned implementation behind a core contract.
