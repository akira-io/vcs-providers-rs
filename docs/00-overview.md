# Overview

`git-cognition-rs` is a universal abstraction layer for Version Control System providers in Rust.

It is not a GitHub SDK. GitHub, GitLab, Bitbucket, and future providers are provider implementations behind provider-neutral Rust contracts.

The public model uses universal terminology:

| Universal concept | GitHub | GitLab | Bitbucket |
| --- | --- | --- | --- |
| Code review | Pull request | Merge request | Pull request |
| Organization | Organization | Group | Workspace |
| Pipeline | Actions workflow run | CI/CD pipeline | Pipeline |

The framework must keep provider-specific behavior inside provider crates. Core contracts must remain provider-neutral and stable.

## Goals

- Provider isolation.
- Transport isolation.
- Async-first APIs.
- Contract-first provider behavior.
- Runtime capability negotiation.
- Immutable resource modeling.
- Middleware pipelines.
- Observable transport and provider health.
- Explicit APIs with no hidden behavior.

## Non-goals

- Exposing provider HTTP clients as public API.
- Exposing raw provider payloads as public API.
- Reusing provider terminology in universal contracts.
- Central switch statements that require editing core when adding providers.

## Two Planes

`cognition()` is the single entry point. It exposes two planes that compose independently:

- **Remote**: `cognition().provider(driver)` talks to GitHub, GitLab or Bitbucket through
  `Provider` contracts over HTTP transport.
- **Local**: `cognition().local()` reads the local Git object database through the `git` CLI.
  Use it for repository cognition: log, diff, blame, merge preview, worktrees, status, show.

Each plane has its own capability namespace (`Capability` vs `LocalGitCapability`); they never
share contracts or transport.

## Quickstart

```rust
use git_cognition_core::{cognition, repo};
use git_cognition_github::github;

// Remote plane
let repository = cognition().provider(github())
    .transport(http().transport().get()?)
    .repos()
    .get(repo().owner("akira-io").name("git-cognition-rs").get())
    .await?;

// Local plane
let graph = cognition().local()
    .repo("/workspace/git-cognition-rs")
    .log()
    .limit(50)
    .graph()?;
```

See `22-end-to-end-usage.md` for the full provider flow and `24-local-git-cognition-reads.md`
for the full local surface.
