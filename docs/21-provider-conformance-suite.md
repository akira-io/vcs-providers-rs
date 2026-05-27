# Provider Conformance Suite

The common conformance suite verifies provider behavior through core contracts. It is used by GitHub, GitLab and Bitbucket tests with provider-specific expectations supplied by each provider crate.

```rust
use vcs_provider_core::{Capability, conformance};
use vcs_provider_github::github;

conformance()
    .provider(github())
    .id("github")
    .display_name("GitHub")
    .supports([Capability::Repos, Capability::CodeReviews])
    .check()?;
```

The suite validates:

- Provider descriptor id and display name.
- Supported and unsupported capabilities.
- Every known capability is explicitly classified by the provider test.
- Auth header style expectations.
- Object-safe access to universal contracts.
- Supported universal contracts fail with `TransportNotConfigured` when no client is attached.
- Unsupported universal contracts fail with `UnsupportedOperation`.
- Registry registration, duplicate rejection and capability filtering.

Provider crates own their expected capability list. Core does not import provider crates or hardcode provider names.

A supported capability means the framework exposes a typed contract for that provider today. Do not mark provider API features as supported just because the upstream provider has an endpoint. Organizations, discussions and webhooks stay unsupported in conformance until they have core contracts and provider implementations.

## Adding A Provider

New providers should add one conformance test before adding provider-specific request tests:

```rust
#[test]
fn custom_provider_passes_common_conformance_suite() -> vcs_provider_core::VcsResult<()> {
    conformance()
        .provider(custom())
        .id("custom")
        .display_name("Custom")
        .supports([Capability::Repos])
        .does_not_support([Capability::Issues])
        .check()
}
```

If the provider supports an auth mode, add it explicitly:

```rust
conformance()
    .provider(custom())
    .auth(AuthKind::PersonalAccessToken, AuthHeaderStyle::AuthorizationBearer)
    .check()?;
```

The conformance suite does not replace provider endpoint tests. It proves that every provider behaves consistently through shared contracts before provider-specific URL, payload and hydration tests run.
