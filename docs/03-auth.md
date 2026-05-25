# Auth

The core crate exposes provider-neutral authentication primitives. Applications create credentials with the `auth()` helper and pass them to providers without depending on provider-specific token shapes.

```rust
let credential = auth().personal_access_token("token");
let header = github().auth_header(&credential);
```

## Credential Kinds

Supported credential kinds:

- Anonymous
- Personal access token
- OAuth token
- App installation token
- JWT

Each credential reports its `AuthKind`, so providers can decide the correct header style for the credential they receive.

## Header Mapping

Providers own header style decisions:

- GitHub personal access tokens use `Authorization: Bearer <token>`.
- GitLab personal access tokens use `private-token: <token>`.
- Bitbucket OAuth tokens use `Authorization: Bearer <token>`.

The core crate owns the neutral header model. It does not expose an HTTP client type and does not depend on a provider crate.

## Token Handling

`AuthToken` and `AuthHeaderValue` redact their debug output. The raw value remains available through `as_str()` because transports need to write outbound headers.

Do not log `AuthHeaderValue::as_str()` or `AuthToken::as_str()`.
