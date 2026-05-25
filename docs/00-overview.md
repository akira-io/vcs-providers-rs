# Overview

`vcs-providers-rs` is a universal abstraction layer for Version Control System providers in Rust.

It is not a GitHub SDK. GitHub, GitLab, Bitbucket, and future providers are driver implementations behind provider-neutral Rust contracts.

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
