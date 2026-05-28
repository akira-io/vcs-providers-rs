# git-cognition-core

Provider-neutral contracts and the local Git read surface for
[`git-cognition`](https://crates.io/crates/git-cognition).

This crate is the substrate every provider driver implements. It owns:

- The `Provider` trait and resource ports (`Repos`, `Issues`, `CodeReviews`, `Pipelines`,
  `Releases`, `Organizations`).
- Universal resources (`Repository`, `Branch`, `Commit`, `Issue`, `CodeReview`, …) and
  capability negotiation (`Capability`, `CapabilitySet`).
- Transport contracts, middleware, retry, rate limit, telemetry, pagination.
- The local Git read surface exposed via `cognition().local()` (log, diff, blame,
  merge preview, worktree, status, show, merge_base).

`git-cognition-core` never imports a provider crate. Applications compose providers explicitly.

## Install

```toml
git-cognition-core = "0.1"
```

Most consumers should depend on the [`git-cognition`](https://crates.io/crates/git-cognition)
umbrella instead and pick provider features there. Reach for `git-cognition-core` directly when
implementing a new provider or when you only need the local Git plane.

## Local Git plane

```rust
use git_cognition_core::cognition;

let repository = cognition().local().repo("/workspace/project");
let graph = repository.log().limit(50).graph()?;
let diff = repository.diff().working().compute()?;
```

## Documentation

See [git-cognition-rs/docs](https://github.com/akira-io/git-cognition-rs/tree/main/docs) for the
full architecture, provider contracts, end-to-end flows, and the local Git read surface.

## License

Dual-licensed under MIT or Apache-2.0.
