# Architecture

The Rust implementation is centered on one provider-neutral `core` crate and one crate per provider.

```text
Application
    |
    v
git-cognition-core contracts
    |
    v
provider crates
```

## Core

`git-cognition-core` owns shared contracts and domain primitives:

- Provider contracts.
- Capability negotiation.
- Provider-neutral errors.
- Auth primitives.
- Resource primitives.
- Pagination primitives.
- Transport contracts.
- Middleware contracts.
- Telemetry contracts.

Core never depends on provider crates.

## Providers

Provider crates implement core contracts:

- `git-cognition-github`
- `git-cognition-gitlab`
- `git-cognition-bitbucket`

Providers depend on `git-cognition-core`. Provider crates own provider-specific defaults, terminology mapping, endpoint behavior, payload mapping, and extensions.

Adding a new provider must not require editing `git-cognition-core`.

## Provider Contract

Every provider crate exposes a type implementing `Provider`.

The provider describes:

- Provider identity.
- Display name.
- Capabilities.
- Repos contract.
- Default endpoint.
- Supported authentication modes.

Applications and managers consume providers through `Provider`, not through concrete provider types.

## Repos Contract

`Repos` is the provider-neutral contract for repository operations.

It exposes:

- `get`
- `list`
- `search`
- `branches`
- `commits`

The contract is async-first and object-safe. Provider crates return futures through the shared `BoxFuture` type, so applications can consume repository operations through trait objects without depending on provider-specific types.

Provider crates own the mapping from provider endpoints to universal `Repository`, `Branch`, and `Commit` resources. Until transport is configured, repos return `CognitionError::TransportNotConfigured` instead of generating placeholder data.

## Registry Contract

`ProviderRegistry` stores providers by provider identity.

Applications compose the registry explicitly:

```rust
let registry = provider()
    .register(git_cognition_github::github())?
    .register(git_cognition_gitlab::gitlab())?
    .register(git_cognition_bitbucket::bitbucket())?
    .build();
```

The registry lives in `core`, but it never imports provider crates. Applications decide which providers to register.

## Dependency Rules

- `core` does not depend on providers.
- Providers depend on `core`.
- Provider-specific logic stays inside provider crates.
- Transport contracts live in `core`; concrete HTTP transport must not leak provider payloads or HTTP client types.
- Resources, errors, capabilities, auth, middleware, pagination, and telemetry stay provider-neutral.

## Local Git Plane

`git-cognition-core` also exposes a local Git plane through `cognition().local()` (and the
`git()` shortcut). This plane is independent of the provider plane:

```text
Application
    |
    +-----> cognition().provider(github())   --> HTTP transport --> remote API
    |
    +-----> cognition().local().repo(path)   --> git CLI         --> local object DB
```

It does not use HTTP transport, auth middleware, or `Provider` drivers. It shells out to the local
`git` binary and returns the same provider-neutral resources where applicable (`Commit`, `Branch`).
The capability namespace is `LocalGitCapability`, separate from `Capability`. See
`24-local-git-cognition-reads.md` for the read surface and `22-end-to-end-usage.md` for the
combined provider + local flow.

```rust
use git_cognition_core::cognition;

let repository = cognition().local().repo("/workspace/project");
let head = repository.show("HEAD").file("README.md")?;
let graph = repository.log().limit(50).graph()?;
```
