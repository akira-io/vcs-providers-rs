# HTTP Auth Middleware Runtime

The core crate owns HTTP execution through the universal `Transport` trait. `HttpTransport` is the real HTTP implementation, but it does not expose `reqwest` in public function signatures.

```rust
let repository = vcs(github())
    .transport(http().transport().get()?)
    .auth(auth().personal_access_token("token"))
    .repos()
    .get(repo().owner("akira-io").name("vcs-providers-rs").get())
    .await?;
```

Provider crates choose their own default headers and auth mapping. The core transport only sends a typed `Request` and returns a typed `Response`.

## Provider Auth Rules

| Provider | Token style | Default headers | Source |
| --- | --- | --- | --- |
| GitHub | `Authorization: Bearer <token>` | `Accept: application/vnd.github+json`, `X-GitHub-Api-Version: 2022-11-28` | [GitHub REST headers and auth](https://docs.github.com/en/rest/using-the-rest-api/getting-started-with-the-rest-api), [GitHub rate limits](https://docs.github.com/en/rest/using-the-rest-api/rate-limits-for-the-rest-api) |
| GitLab | `PRIVATE-TOKEN: <token>` for personal access tokens, Bearer for OAuth-style tokens | `Accept: application/json` | [GitLab REST auth](https://docs.gitlab.com/api/rest/authentication/), [GitLab REST API](https://docs.gitlab.com/api/rest/) |
| Bitbucket | Bearer for OAuth tokens | `Accept: application/json` | [Bitbucket OAuth](https://support.atlassian.com/bitbucket-cloud/docs/use-oauth-on-bitbucket-cloud/), [Bitbucket request limits](https://support.atlassian.com/bitbucket-cloud/docs/api-request-limits/) |

Bitbucket app passwords require username plus password credentials and are being removed by Atlassian. They are not modeled as a provider-neutral token in this version.

## Middleware Boundary

Middleware remains transport-level and provider-neutral:

```rust
let transport = middleware()
    .with(HeaderMiddleware::make("x-request-id", "request-1"))
    .transport(http().transport().get()?)
    .build();
```

Retries are provider-neutral and can be applied through the facade:

```rust
let repository = vcs(github())
    .retry(http().transport().get()?)
    .attempts(3)
    .on_statuses([429, 500, 502, 503, 504])
    .repos()
    .get(repo().owner("akira-io").name("vcs-providers-rs").get())
    .await?;
```

Provider clients can then use that composed transport:

```rust
let repository = vcs(gitlab())
    .transport(transport)
    .auth(auth().personal_access_token("token"))
    .repos()
    .get(repo().owner("akira-io").name("vcs-providers-rs").get())
    .await?;
```

The request path is:

```text
Provider client -> provider headers -> auth header -> middleware -> retry -> HttpTransport -> typed Response -> mapper
```

Retry decisions use response status codes only. Provider-specific rate-limit headers remain observable through `rate_limit().headers()`, and callers decide which statuses are retryable for their workload.
