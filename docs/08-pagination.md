# Pagination

Pagination is provider-neutral in core and provider-specific in adapters.

Providers must map their native paging model into an opaque cursor:

- GitHub REST `Link` headers with `rel="next"` expose the next page value as `PageCursor`.
- GitLab `x-next-page`, `x-next-cursor`, or `Link` headers expose the next value as `PageCursor`.
- Bitbucket `next` links are preserved as opaque `PageCursor` values.

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
let repositories = github()
    .repos()
    .list(
        github()
            .repo()
            .query()
            .pagination(pagination().request().limit(50).build())
            .list(),
    )
    .await?;

let next_page = repositories.next();
```

`PageCursor` is intentionally opaque to application code. Pass it back through the provider query builder instead of parsing it.

```rust
let next_repositories = github()
    .repos()
    .list(
        github()
            .repo()
            .query()
            .pagination(
                pagination()
                    .request()
                    .limit(50)
                    .cursor(next_page.ok_or(VcsError::InvalidInput("missing next cursor".into()))?.as_str())
                    .build(),
            )
            .list(),
    )
    .await?;
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

## Provider Behavior

| Provider | Native source | Cursor stored by `Page<T>` | Next request behavior |
| --- | --- | --- | --- |
| GitHub | `Link` header | `page` query value from the `rel="next"` URL | provider builders send it as `page` |
| GitLab | `x-next-cursor`, `x-next-page`, or `Link` header | cursor or page value | provider builders send it as `page` for offset pagination |
| Bitbucket | paginated body `next` field | full `next` URL | provider builders use the URL directly |

Bitbucket is different because its documentation treats `next` as an opaque location. The provider therefore preserves the URL and avoids reconstructing the next request from a page number.
