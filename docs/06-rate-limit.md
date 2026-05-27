# Rate Limit

Rate limit support is configured through header profiles instead of provider-specific functions.

```rust
let profile = rate_limit()
    .headers()
    .remaining(["x-ratelimit-remaining", "ratelimit-remaining"])
    .reset_at(["x-ratelimit-reset", "ratelimit-reset"])
    .retry_after(["retry-after"])
    .cost(["x-ratelimit-used", "ratelimit-used"])
    .build();
```

## Header Profiles

Header profiles describe which response headers carry rate limit information. Providers or applications can supply their own profiles without modifying core.

The profile can read:

- Remaining quota
- Reset value
- Retry-after value
- Request cost

Header matching is case-insensitive.

## Observation

`RateLimitObservation` is provider-neutral. It keeps values typed and optional because not every provider exposes every rate limit field for every endpoint.

## Transport Observation

Rate limit observation can run inside the provider transport flow. The core receives a configured header profile and records typed observations after every response.

```rust
let recorder = rate_limit().recorder();

let repository = vcs(github())
    .rate_limit(http().transport().get()?)
    .remaining(["x-ratelimit-remaining"])
    .reset_at(["x-ratelimit-reset"])
    .retry_after(["retry-after"])
    .cost(["x-ratelimit-used"])
    .recorder(recorder.clone())
    .repos()
    .get(repo().owner("akira-io").name("vcs-providers-rs").get())
    .await?;

let observations = recorder.observations();
```

Providers do not register rate limit profiles in core. Applications configure the headers that matter for the selected provider, self-hosted deployment, or proxy layer.
