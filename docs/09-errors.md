# Errors

Errors are provider-neutral contracts owned by `vcs-provider-core`.

Provider crates must map native provider failures into `VcsError` before returning results to applications. They must not expose raw HTTP client errors, provider payloads, or provider-specific error names through public contracts.

```rust
let result = error().from_status(&response_status);
```

## Error Kinds

`VcsError::kind()` exposes stable classification for callers that need branching without parsing messages.

```rust
let kind = error().provider_not_registered("github").kind();
```

## Status Mapping

Core provides a small universal status mapper:

- `401` maps to unauthorized.
- `403` maps to forbidden.
- `404` maps to not found.
- `409` maps to conflict.
- `429` maps to rate limited.
- `500..=599` maps to provider unavailable.

Providers may add more context internally, but public errors must remain provider-neutral.
