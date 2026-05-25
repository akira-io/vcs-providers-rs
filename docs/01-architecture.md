# Architecture

The Rust implementation is centered on one provider-neutral `core` crate and one crate per provider.

```text
Application
    |
    v
vcs-provider-core contracts
    |
    v
provider crates
```

## Core

`vcs-provider-core` owns shared contracts and domain primitives:

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

- `vcs-provider-github`
- `vcs-provider-gitlab`
- `vcs-provider-bitbucket`

Providers depend on `vcs-provider-core`. Provider crates own provider-specific defaults, terminology mapping, endpoint behavior, payload mapping, and extensions.

Adding a new provider must not require editing `vcs-provider-core`.

## Provider Contract

Every provider crate exposes a type implementing `Provider`.

The provider describes:

- Provider identity.
- Display name.
- Capabilities.
- Repositories contract.
- Default endpoint.
- Supported authentication modes.

Applications and managers consume providers through `Provider`, not through concrete provider types.

## Repositories Contract

`Repositories` is the provider-neutral contract for repository operations.

It exposes:

- `get`
- `list`
- `search`
- `branches`
- `commits`

The contract is async-first and object-safe. Provider crates return futures through the shared `BoxFuture` type, so applications can consume repository operations through trait objects without depending on provider-specific types.

Provider crates own the mapping from provider endpoints to universal `Repository`, `Branch`, and `Commit` resources. Until transport is configured, repositories return `VcsError::TransportNotConfigured` instead of generating placeholder data.

## Registry Contract

`ProviderRegistry` stores providers by provider identity.

Applications compose the registry explicitly:

```rust
let registry = ProviderRegistry::builder()
    .register(vcs_provider_github::provider())?
    .register(vcs_provider_gitlab::provider())?
    .register(vcs_provider_bitbucket::provider())?
    .build();
```

The registry lives in `core`, but it never imports provider crates. Applications decide which providers to register.

## Dependency Rules

- `core` does not depend on providers.
- Providers depend on `core`.
- Provider-specific logic stays inside provider crates.
- Transport contracts live in `core`; concrete HTTP transport must not leak provider payloads or HTTP client types.
- Resources, errors, capabilities, auth, middleware, pagination, and telemetry stay provider-neutral.
