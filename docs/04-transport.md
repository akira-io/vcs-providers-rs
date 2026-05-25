# Transport

The core crate defines transport contracts without exposing a concrete HTTP client.

```rust
let request = request()
    .get("https://api.example.test/repos")
    .auth_header(github().auth_header(&credential))
    .build();
```

## Request Model

Requests contain:

- HTTP method
- URL
- headers
- optional opaque body

The core request model does not expose `reqwest` or any provider payload type.

Mutation requests can carry an opaque body:

```rust
let request = request()
    .post("https://api.example.test/repos")
    .body(RequestBody::make("{}"))
    .build();
```

Provider crates map typed drafts and patches into transport bodies without exposing raw provider payload structures.

## Transport Contract

Transport implementations implement `Transport`:

```rust
fn send(&self, request: Request) -> BoxFuture<'_, VcsResult<Response>>;
```

The transport trait is async-first and object-safe. Concrete HTTP clients belong behind transport implementations, not in public provider contracts.

## Auth Integration

Providers map credentials into `AuthHeader` values. Requests accept those values through `auth_header(...)`, keeping authentication separate from transport execution.
