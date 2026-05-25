# Pagination

Pagination is provider-neutral in core and provider-specific in adapters.

Providers must map their native paging model into an opaque cursor:

- GitHub REST `Link` headers become a `PageCursor`.
- GitLab page headers or keyset cursors become a `PageCursor`.
- Bitbucket `next` links become a `PageCursor`.

Core does not expose provider pagination headers, page URLs, or response payloads.

```rust
let page_request = pagination()
    .request()
    .limit(100)
    .cursor("opaque-provider-cursor")
    .build();
```

Resources that return lists use `Page<T>`.

```rust
let page = pagination()
    .page(repositories)
    .next("next-provider-cursor")
    .build();
```

## Request URLs

Providers should use the core URL builder instead of manually concatenating query strings.

```rust
let request_url = url("https://api.example.test/items")
    .query_param("per_page", "100")
    .optional_query_param("cursor", next_cursor)
    .build();
```

This keeps query parameter handling explicit and reusable without exposing transport internals.
