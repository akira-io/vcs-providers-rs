# Middleware

Middleware composes request mutation before transport execution.

```rust
let pipeline = middleware()
    .header("accept", "application/json")
    .transport(transport)
    .build();
```

## Pipeline Contract

`Middleware` receives a provider-neutral `Request` and returns a provider-neutral `Request`.

The pipeline runs middleware in registration order and then calls the configured `Transport`.

## Transport Isolation

Middleware does not know providers and does not know concrete HTTP clients. It only works with core request and response types.

Concrete transports remain behind the `Transport` trait.
