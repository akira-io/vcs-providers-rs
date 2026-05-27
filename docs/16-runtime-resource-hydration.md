# Runtime Resource Hydration

Provider request builders create provider-specific HTTP requests. Runtime-backed provider clients execute those requests through a configured HTTP transport and hydrate provider-neutral resources.

```rust
let repository = vcs(github())
    .transport(my_http_transport)
    .repos()
    .get(repo().owner("akira-io").name("vcs-providers-rs").get())
    .await?;
```

The returned value is a universal `Repository`. GitHub, GitLab, and Bitbucket response fields are mapped inside their provider crates.

Application code should not need to construct a response transport. The transport is an infrastructure dependency: production clients use a real HTTP transport, and tests can replace it with a deterministic fixture.

## Core Boundary

Core owns the reusable execution contract:

```rust
pub trait RepositoryResponseMapper {
    fn repository(&self, requested_repo: &Repo, response: &Response) -> VcsResult<Repository>;
    fn repositories(&self, response: &Response) -> VcsResult<Page<Repository>>;
    fn branches(&self, response: &Response) -> VcsResult<Page<Branch>>;
    fn commits(&self, response: &Response) -> VcsResult<Page<Commit>>;
}
```

Core does not parse provider JSON. `TransportBackedRepos` only sends requests, maps HTTP status errors, and delegates resource hydration to the provider mapper.

## Provider Boundary

Provider crates own response shapes:

| Provider | Repository identity | Visibility | Lifecycle |
| --- | --- | --- | --- |
| GitHub | `full_name` | `private` | `archived`, `disabled` |
| GitLab | `path_with_namespace` | `visibility` | `archived` |
| Bitbucket | `full_name` | `is_private` | always active |

Provider payload structs remain private. Public APIs expose only universal resources and typed errors.

## Response Body Fixtures

`Response` carries an optional `ResponseBody`. Tests can use `provider_response()` to provide one response without exposing the low-level transport implementation in each test:

```rust
run_async_test(async {
    let repository = vcs(github())
        .transport(
            provider_response()
                .body(r#"{"full_name":"akira-io/vcs-providers-rs"}"#)
                .get(),
        )
        .repos()
        .get(repo().owner("akira-io").name("vcs-providers-rs").get())
        .await?;

    Ok(())
})?;
```

The body is plain text at the client boundary. Providers choose the parser privately and must map parse failures into `VcsError`.
