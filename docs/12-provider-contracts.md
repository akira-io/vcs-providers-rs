# Provider Contracts

Every provider exposes the same universal contract surface:

```rust
let provider = github();

let repos = provider.repos();
let issues = provider.issues();
let code_reviews = provider.code_reviews();
let pipelines = provider.pipelines();
let releases = provider.releases();
let capabilities = provider.capabilities();
```

These contracts are provider-neutral. GitHub pull requests, GitLab merge requests, and Bitbucket pull requests all map to `CodeReview`.

## Contract Shape

The core `Provider` trait exposes small domain contracts instead of one large interface:

| Contract | Responsibility |
| --- | --- |
| `Repos` | Repository lookup, listing, search, mutation, branches, and commits. |
| `Issues` | Issue lookup, listing, creation, update, close, and delete where supported. |
| `CodeReviews` | Code review lookup, listing, creation, update, merge, close, and delete where supported. |
| `Pipelines` | Pipeline lookup, listing, rerun, and cancel. |
| `Releases` | Release lookup, listing, creation, update, and delete where supported. |

Provider crates implement the universal trait surface and keep provider-specific endpoint routing inside their own crates.

## Resource Builders

Universal resources use fluent builders so construction order stays explicit and predictable:

```rust
let repo = repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
    .get();

let issue = issue()
    .repo(repo.clone())
    .id("42")
    .get();

let code_review = code_review()
    .repo(repo.clone())
    .id("17")
    .get();

let draft = code_review()
    .draft()
    .repo(repo.clone())
    .title("Add provider contracts")
    .get();

let pipeline = pipeline()
    .repo(repo.clone())
    .id("build-100")
    .get();

let release = release()
    .repo(repo)
    .id("v1.0.0")
    .get();
```

List operations use query objects. That keeps the contracts stable when filters are added later:

```rust
let repo = repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
    .get();

let page = pagination()
    .request()
    .limit(50)
    .build();

let issue_query = issue()
    .query()
    .location(repo.clone())
    .pagination(page.clone())
    .list();

let code_review_query = code_review()
    .query()
    .location(repo.clone())
    .pagination(page.clone())
    .list();

let pipeline_query = pipeline()
    .query()
    .location(repo.clone())
    .pagination(page.clone())
    .list();

let release_query = release()
    .query()
    .location(repo)
    .pagination(page)
    .list();
```

## Capabilities

Capabilities are runtime information, not compile-time assumptions:

```rust
if provider.capabilities().supports(&Capability::Pipelines) {
    let pipelines = provider.pipelines();
}
```

Calling a contract that is not transport-backed returns `VcsError::TransportNotConfigured` until the provider wires transport and response mapping for that domain.

A provider must only advertise capabilities backed by the current universal contract surface. Provider APIs can expose more concepts than the core crate models today. GitHub organizations, GitHub discussions, GitLab groups and provider webhooks are real provider APIs, but they are not advertised as `Capability::Organizations`, `Capability::Discussions` or `Capability::Webhooks` until the framework has typed contracts, request builders, hydration and conformance coverage for them.

## Provider Isolation

The core crate defines only universal contracts and resources. It does not import GitHub, GitLab, or Bitbucket crates, and it does not know provider endpoint shapes.

Provider crates depend on core and implement the shared contracts:

```text
core -> universal contracts
github -> GitHub implementation
gitlab -> GitLab implementation
bitbucket -> Bitbucket implementation
```

Adding a provider means implementing `Provider` and returning each contract object from that provider crate. Core does not change for new providers.
