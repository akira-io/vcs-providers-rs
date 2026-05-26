# Provider Manager Facade

`vcs(driver)` is the provider manager facade for request construction. It keeps core provider-neutral while allowing application code to choose GitHub, GitLab, Bitbucket, or a future provider at the edge.

```rust
use vcs_provider_core::vcs;
use vcs_provider_gitlab::gitlab;

let repo = vcs(gitlab())
    .repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
    .get();
```

The facade owns the selected driver once and every chained resource operation uses that driver. Code should not repeat the provider inside nested resource calls.

## Repository Requests

```rust
let repository = vcs(gitlab())
    .repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
    .get();

let url = repository.url();
```

Provider-specific path rules stay inside the provider crate:

| Provider | Repository URL shape |
| --- | --- |
| GitHub | `/repos/{owner}/{repo}` |
| GitLab | `/api/v4/projects/{owner}%2F{repo}` |
| Bitbucket | `/repositories/{workspace}/{repo_slug}` |

## Nested Resources

Nested resource builders inherit the facade driver from the repository chain.

```rust
let issue = vcs(gitlab())
    .repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
    .issue("42")
    .get();

let code_review = vcs(gitlab())
    .repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
    .code_review("42")
    .get();
```

GitHub and GitLab expose issues, code reviews and releases through this facade. Bitbucket exposes repositories and code reviews through the same facade; issues and releases are not enabled in its universal capability set.

## Mutation Requests

Mutation builders are also reached from the selected facade.

```rust
let request = vcs(gitlab())
    .repo()
    .draft(repo)
    .visibility(vcs_provider_core::Visibility::Private)
    .create();
```

The terminal method describes the command being built: `create`, `update`, `delete`, or `close` where the provider supports it.

## Dependency Boundary

`vcs(driver)` lives in `vcs-provider-core`, but it receives the driver from the application. Core does not import provider crates and providers do not register themselves globally.

This keeps provider addition open-ended:

```rust
let repo = vcs(custom_provider)
    .repo()
    .owner("team")
    .name("project")
    .get();
```

The custom provider only needs to implement the same core manager traits as the built-in providers.
