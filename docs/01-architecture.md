# Architecture

The Rust implementation is centered on one provider-neutral `core` crate and one crate per provider.

```text
Application
    |
    v
vcs-provider-core contracts
    |
    v
provider driver crates
```

## Core

`vcs-provider-core` owns shared contracts and domain primitives:

- Provider driver contracts.
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

## Driver Contract

Every provider exposes a driver implementing `ProviderDriver`.

The driver describes:

- Provider identity.
- Display name.
- Capabilities.
- Default endpoint.
- Supported authentication modes.

Applications and managers consume drivers through `ProviderDriver`, not through concrete provider types.

## Registry Contract

`ProviderRegistry` stores provider drivers by provider identity.

Applications compose the registry explicitly:

```rust
let registry = ProviderRegistry::builder()
    .register(vcs_provider_github::driver())?
    .register(vcs_provider_gitlab::driver())?
    .register(vcs_provider_bitbucket::driver())?
    .build();
```

The registry lives in `core`, but it never imports provider crates. Each provider owns its driver and applications decide which drivers to register.

## Dependency Rules

- `core` does not depend on providers.
- Providers depend on `core`.
- Provider-specific logic stays inside provider crates.
- Transport contracts live in `core`; concrete HTTP transport must not leak provider payloads or HTTP client types.
- Resources, errors, capabilities, auth, middleware, pagination, and telemetry stay provider-neutral.
