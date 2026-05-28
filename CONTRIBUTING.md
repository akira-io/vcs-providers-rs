# Contributing

Thanks for considering a contribution to `git-cognition`.

## Getting started

1. Fork the repository and create a topic branch from `main`.
2. Install Rust 1.89 or newer (`rustup toolchain install 1.89.0`).
3. Run the full test matrix locally before opening a PR:

   ```sh
   cargo fmt --check
   cargo test --all-features
   cargo clippy --all-targets --all-features -- -D warnings
   ```

## Commits

This repository follows [Conventional Commits](https://www.conventionalcommits.org/).
Allowed types: `feat`, `fix`, `chore`, `docs`, `style`, `refactor`, `perf`, `test`,
`build`, `ci`, `revert`. The changelog is generated from these types on every release.

Examples:

```
feat(github): add organization listing
fix(local-git): correct merge preview region paths
docs(readme): document cargo add install paths
```

## Coding standards

- Rust edition 2024, MSRV 1.89.
- `unsafe_code = forbid`, `unwrap_used`/`expect_used`/`panic`/`todo` deny in production code.
- Public API items carry rustdoc (`///`). Internal helpers stay un-documented unless non-obvious.
- Per-feature changes must build under `--no-default-features --features <name>`.

## Adding a provider

1. Create `src/<provider>/` with its modules and gate it on `#[cfg(feature = "<provider>")]`
   in `src/lib.rs`.
2. Implement the contracts from `src/core` for the new provider.
3. Add `tests/<provider>.rs` plus `tests/<provider>/` files (gated with
   `#![cfg(feature = "<provider>")]`).
4. Update `docs/23-unified-dev-adapter-coverage.md` and the matrix in `.github/workflows/test.yml`.

## Pull requests

Keep PRs focused on a single concern. Reference issues with `Fixes #N` or `Refs #N` in the body.
Tests must pass under `--all-features` and clippy must stay clean before merge.

## License

By submitting a PR, you agree your contribution is dual-licensed under MIT and Apache-2.0,
matching the crate license.
