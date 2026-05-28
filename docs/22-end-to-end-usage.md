# End-to-End Usage

This guide shows the main application flow for `git-cognition-rs`: choose a provider, configure transport, add auth when needed, call a provider-neutral contract, and receive a typed resource.

## Provider Selection

Applications select the provider at the edge. Core does not import provider crates.

```rust
use git_cognition_core::cognition;
use git_cognition_gitlab::gitlab;

let repository = cognition().provider(gitlab())
    .repo()
    .owner("akira-io")
    .name("git-cognition-rs")
    .get();

let url = repository.url();
```

Use the same shape for GitHub and Bitbucket:

```rust
let github_repository = git_cognition_core::cognition().provider(git_cognition_github::github())
    .repo()
    .owner("akira-io")
    .name("git-cognition-rs")
    .get();

let bitbucket_repository = git_cognition_core::cognition().provider(git_cognition_bitbucket::bitbucket())
    .repo()
    .owner("akira-io")
    .name("git-cognition-rs")
    .get();
```

## Runtime Client

Provider clients execute requests through the universal transport contract. The provider owns URL construction, default headers, auth mapping, and response mapping.

```rust
use git_cognition_core::{auth, cognition, http, repo};
use git_cognition_github::github;

let repository = cognition().provider(github())
    .transport(http().transport().get()?)
    .auth(auth().personal_access_token("token"))
    .repos()
    .get(repo().owner("akira-io").name("git-cognition-rs").get())
    .await?;
```

The return type is `Repository`, not a provider payload.

Enterprise and self-managed installations are configured on the provider before it enters the facade:

```rust
let repository = cognition().provider(gitlab().base_url("https://gitlab.internal.example"))
    .transport(http().transport().get()?)
    .auth(auth().personal_access_token("token"))
    .repos()
    .get(repo().owner("akira-io").name("git-cognition-rs").get())
    .await?;
```

Use `github().base_url("https://github.enterprise.test/api/v3")` for GitHub Enterprise Server. GitLab expects the instance origin, for example `https://gitlab.internal.example`, and the provider builds REST paths under `/api/v4`.

## Middleware

Middleware wraps transport, not domain logic. Each request still enters the same provider client and mapper path.

```rust
use git_cognition_core::{cognition, http, middleware, repo};

let transport = middleware()
    .header("x-request-id", "request-1")
    .transport(http().transport().get()?)
    .build();

let repository = cognition().provider(git_cognition_gitlab::gitlab())
    .transport(transport)
    .repos()
    .get(repo().owner("akira-io").name("git-cognition-rs").get())
    .await?;
```

## Repository Operations

Repositories use the same contract across GitHub, GitLab, and Bitbucket.

```rust
let location = repo().owner("akira-io").name("git-cognition-rs").get();

let repository = cognition().provider(github())
    .transport(transport)
    .repos()
    .get(location.clone())
    .await?;

let page = cognition().provider(github())
    .transport(transport)
    .repos()
    .branches(location.clone())
    .await?;
```

Create, update, and delete operations use explicit terminal verbs:

```rust
let created = cognition().provider(gitlab())
    .transport(transport.clone())
    .repos()
    .create()
    .location(location.clone())
    .visibility(git_cognition_core::Visibility::Private)
    .create()
    .await?;

cognition().provider(gitlab())
    .transport(transport)
    .repos()
    .delete(location)
    .await?;
```

## Collaboration Operations

Issues, code reviews, and releases use provider-neutral resources.

```rust
let issue = cognition().provider(github())
    .transport(transport.clone())
    .issues()
    .location(location.clone())
    .title("Fix payment state")
    .body("Details")
    .create()
    .await?;

let code_review = cognition().provider(gitlab())
    .transport(transport.clone())
    .code_reviews()
    .location(location.clone())
    .title("Add provider contract checks")
    .source("feature")
    .target("main")
    .create()
    .await?;

let release = cognition().provider(github())
    .transport(transport)
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

Provider crates can attach response fixtures directly to the provider facade to verify hydration without real HTTP:

```rust
use git_cognition_core::{repo, run_async_test};
use git_cognition_github::github;

run_async_test(async {
    let repository = github()
        .body(r#"{"full_name":"akira-io/git-cognition-rs","private":false}"#)
        .repos()
        .get(repo().owner("akira-io").name("git-cognition-rs").get())
        .await?;

    assert_eq!(repository.repo().owner().as_str(), "akira-io");

    Ok(())
})?;
```

Tests that need to inspect the final outbound request should record through the provider fixture:

```rust
let transport = github()
    .body(r#"{"full_name":"akira-io/git-cognition-rs","private":false}"#)
    .record();

let repository = cognition().provider(github())
    .transport(transport.clone())
    .repos()
    .get(repo().owner("akira-io").name("git-cognition-rs").get())
    .await?;

assert_eq!(transport.requests().len(), 1);
assert_eq!(repository.repo().owner().as_str(), "akira-io");
```

Retry tests can provide multiple provider responses without constructing transports directly:

```rust
let transport = github()
    .responses()
    .status(500)
    .body(r#"{"full_name":"akira-io/git-cognition-rs","private":false}"#)
    .record();
```

## Capability Checks

Do not assume every provider supports every resource.

```rust
use git_cognition_core::Capability;

if github().capabilities().supports(&Capability::Releases) {
    github().releases();
}
```

Runtime capability checks are part of the public contract. Provider-specific features should stay in provider crates or extensions.

Capabilities describe framework-supported universal contracts, not every upstream provider endpoint. A provider can have native webhooks or organization APIs without exposing those capabilities until `git-cognition-core` has typed contracts for them.
