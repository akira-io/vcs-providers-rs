# Repository Requests

Repository request builders translate universal repository concepts into provider-specific REST endpoints.

They live inside provider crates because endpoint shapes are provider knowledge:

```rust
let repo = github()
    .repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
    .get();

let url = repo.branches(None);
```

Collections use the provider repository namespace without a specific repository:

```rust
let repo = github().repo();
let query = repo.query().search("vcs provider", None);
let url = repo.collection().search(&query);
```

Use `vcs(driver)` when the driver is selected dynamically or passed as a dependency:

```rust
let repo = vcs(gitlab())
    .repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
    .get();
```

The core crate only provides neutral primitives: `Repo`, pagination requests, request URLs, and transport requests.

## Create, Update, Delete

Use `RepositoryDraft` to create repositories and `RepositoryPatch` to update repository settings:

```rust
let repo = github()
    .repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
    .get();

let create_request = github()
    .repo()
    .draft(repo.clone())
    .visibility(Visibility::Private)
    .description("Universal VCS provider abstraction")
    .create();

let repository_patch = repo
    .patch()
    .visibility(Visibility::Public)
    .description("Universal VCS provider abstraction")
    .get();

let update_request = repo.update(&repository_patch);
let delete_request = repo.delete();
```

Provider support:

| Provider | Create | Update | Delete |
| --- | --- | --- | --- |
| GitHub | supported | supported | supported |
| GitLab | supported | supported | supported |
| Bitbucket | supported | supported | supported |

## URL Access

Provider repository URL methods return `RequestUrl`, not `String`. Use `value()` for the full URL and component accessors when routing, telemetry, or assertions need structured parts:

```rust
let url = bitbucket()
    .repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
    .get()
    .url();

assert_eq!(url.value(), "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs");
assert_eq!(url.scheme(), Some("https"));
assert_eq!(url.domain(), Some("api.bitbucket.org"));
assert_eq!(url.path(), "/2.0/repositories/akira-io/vcs-providers-rs");
```

## Provider Shapes

GitHub repository requests use owner and repository name as separate path segments:

```text
/repos/{owner}/{repo}
/repos/{owner}/{repo}/branches
/repos/{owner}/{repo}/commits
```

GitLab project requests use a URL-encoded project path under `/api/v4/projects`:

```text
/api/v4/projects/{owner%2Frepo}
/api/v4/projects/{owner%2Frepo}/repository/branches
/api/v4/projects/{owner%2Frepo}/repository/commits
```

Bitbucket Cloud repository requests use workspace and repository slug:

```text
/2.0/repositories/{workspace}/{repo_slug}
/2.0/repositories/{workspace}/{repo_slug}/refs/branches
/2.0/repositories/{workspace}/{repo_slug}/commits
```

## Pagination

The universal `PageRequest` remains provider-neutral. Each provider request builder maps it to the provider's query naming:

| Provider | Limit | Cursor |
| --- | --- | --- |
| GitHub | `per_page` | `page` |
| GitLab | `per_page` | `page` |
| Bitbucket | `pagelen` | `page` |

Response pagination hydration is intentionally separate from request construction. That keeps request routing, transport execution, and response mapping independent.
