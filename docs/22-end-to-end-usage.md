# End-to-End Usage

This guide shows the main application flow for `vcs-providers-rs`: choose a provider, configure transport, add auth when needed, call a provider-neutral contract, and receive a typed resource.

## Provider Selection

Applications select the provider at the edge. Core does not import provider crates.

```rust
use vcs_provider_core::vcs;
use vcs_provider_gitlab::gitlab;

let repository = vcs(gitlab())
    .repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
    .get();

let url = repository.url();
```

Use the same shape for GitHub and Bitbucket:

```rust
let github_repository = vcs_provider_core::vcs(vcs_provider_github::github())
    .repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
    .get();

let bitbucket_repository = vcs_provider_core::vcs(vcs_provider_bitbucket::bitbucket())
    .repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
    .get();
```

## Runtime Client

Provider clients execute requests through the universal transport contract. The provider owns URL construction, default headers, auth mapping, and response mapping.

```rust
use vcs_provider_core::{http, repo};
use vcs_provider_github::github;

let repository = github()
    .client(http().transport().get()?)
    .repos()
    .get(repo().owner("akira-io").name("vcs-providers-rs").get())
    .await?;
```

The return type is `Repository`, not a provider payload.

## Middleware

Middleware wraps transport, not domain logic. Each request still enters the same provider client and mapper path.

```rust
use vcs_provider_core::{HeaderMiddleware, http, middleware};

let transport = middleware()
    .with(HeaderMiddleware::make("x-request-id", "request-1"))
    .transport(http().transport().get()?)
    .build();

let repository = vcs_provider_gitlab::gitlab()
    .client(transport)
    .repos()
    .get(repo().owner("akira-io").name("vcs-providers-rs").get())
    .await?;
```

## Repository Operations

Repositories use the same contract across GitHub, GitLab, and Bitbucket.

```rust
let location = repo().owner("akira-io").name("vcs-providers-rs").get();

let repository = github().client(transport).repos().get(location.clone()).await?;

let page = github()
    .client(transport)
    .repos()
    .branches(location.clone())
    .await?;
```

Create, update, and delete operations use explicit terminal verbs:

```rust
let created = gitlab()
    .client(transport)
    .repos()
    .create()
    .location(location.clone())
    .visibility(vcs_provider_core::Visibility::Private)
    .create()
    .await?;

gitlab().client(transport).repos().delete(location).await?;
```

## Collaboration Operations

Issues, code reviews, and releases use provider-neutral resources.

```rust
let issue = github()
    .client(transport)
    .issues()
    .location(location.clone())
    .title("Fix payment state")
    .body("Details")
    .create()
    .await?;

let code_review = gitlab()
    .client(transport)
    .code_reviews()
    .location(location.clone())
    .title("Add provider contract checks")
    .source("feature")
    .target("main")
    .create()
    .await?;

let release = github()
    .client(transport)
    .releases()
    .location(location)
    .tag("v1.0.0")
    .name("v1.0.0")
    .body("Release notes")
    .create()
    .await?;
```

Bitbucket supports code reviews and pipelines in the current universal capability set. Issues and releases remain unsupported there until they can be represented without provider-specific leakage.

## Testing Without Network

Provider crates can use the shared test transport to verify hydration without real HTTP:

```rust
use vcs_provider_core::{provider_response, repo, run_async_test};
use vcs_provider_github::github;

run_async_test(async {
    let repository = github()
        .body(r#"{"full_name":"akira-io/vcs-providers-rs","private":false}"#)
        .repos()
        .get(repo().owner("akira-io").name("vcs-providers-rs").get())
        .await?;

    assert_eq!(repository.repo().owner().as_str(), "akira-io");

    Ok(())
})?;
```

Tests that need to inspect the final outbound request should use `RecordingTransport`.

## Capability Checks

Do not assume every provider supports every resource.

```rust
use vcs_provider_core::Capability;

if github().capabilities().supports(&Capability::Releases) {
    // Build or execute release operations.
}
```

Runtime capability checks are part of the public contract. Provider-specific features should stay in provider crates or extensions.
