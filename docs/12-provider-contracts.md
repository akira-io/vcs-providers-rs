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
| `Repos` | Repository lookup, listing, search, branches, and commits. |
| `Issues` | Issue lookup and listing. |
| `CodeReviews` | Code review lookup, listing, creation, merge, and close. |
| `Pipelines` | Pipeline lookup, listing, rerun, and cancel. |
| `Releases` | Release lookup and listing. |

Provider crates implement the universal trait surface and keep provider-specific endpoint routing inside their own crates.

## Capabilities

Capabilities are runtime information, not compile-time assumptions:

```rust
if provider.capabilities().supports(&Capability::Pipelines) {
    let pipelines = provider.pipelines();
}
```

Calling a contract that is not transport-backed returns `VcsError::TransportNotConfigured` until the provider wires transport and response mapping for that domain.

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
