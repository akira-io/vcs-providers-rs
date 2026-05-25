# Provider Runtime

Provider runtime composes a provider, transport, optional telemetry, request building, and universal error mapping.

```rust
let runtime = runtime()
    .with_provider(vcs_provider_github::github())
    .transport(transport)
    .build();

let request = runtime.request().get("/repos/akira-io/core").build();
let response = runtime.execute(request).await?;
```

The runtime is provider-neutral. It depends only on the `Provider`, `Transport`, `TelemetrySink`, and error contracts from core.

## Provider Configuration

Use `with_provider(...)` when the provider already exists and you want the shortest explicit runtime setup:

```rust
let response = runtime()
    .with_provider(vcs_provider_github::github())
    .transport(transport)
    .request()
    .get("/repos/akira-io/core")
    .send()
    .await?;
```

Use `provider().from(...)` when you want the full fluent provider configuration shape:

```rust
let response = runtime()
    .provider()
    .from(vcs_provider_github::github())
    .transport(transport)
    .request()
    .get("/repos/akira-io/core")
    .send()
    .await?;
```

Both forms use the same provider contract. `with_provider(...)` is the preferred form for concrete providers because it says directly that the runtime receives an existing provider.

Use `provider()` without `from(...)` when configuring a lightweight runtime provider fluently:

```rust
let request = runtime()
    .provider()
    .base_url("https://api.example.test")
    .bearer_auth()
    .transport(transport)
    .request()
    .get("/repos")
    .build();
```

## Responsibilities

- Build requests from provider base URLs.
- Apply provider auth header behavior.
- Execute requests through transport.
- Optionally wrap execution with telemetry.
- Map failed response statuses to universal errors.

Providers still own endpoint paths and resource hydration. Core does not know GitHub, GitLab, or Bitbucket payloads.
