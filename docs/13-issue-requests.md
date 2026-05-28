# Issue Requests

Issue request builders translate universal issue resources into provider-specific REST endpoints.

Use the provider facade when the provider is known:

```rust
let issue = github()
    .repo()
    .owner("akira-io")
    .name("git-cognition-rs")
    .issue("42")
    .get();

let url = issue.url();
```

Use the collection builder for list URLs:

```rust
let issues = gitlab()
    .repo()
    .owner("akira-io")
    .name("git-cognition-rs")
    .issues()
    .pagination()
    .limit(50)
    .list();

let url = issues.url();
```

If the provider uses cursor pagination, keep the pagination in the same chain:

```rust
let issues = github()
    .repo()
    .owner("akira-io")
    .name("git-cognition-rs")
    .issues()
    .pagination()
    .limit(50)
    .cursor("2")
    .list();

let url = issues.url();
```

If the repo already exists as a variable, pass it into the issue builder:

```rust
let repo = github()
    .repo()
    .owner("akira-io")
    .name("git-cognition-rs")
    .get();

let issue = github()
    .issue()
    .repo(repo)
    .id("42")
    .get();
```

Use `cognition().provider(driver)` when the provider is injected:

```rust
let provider = cognition().provider(gitlab());

let issue = provider
    .repo()
    .owner("akira-io")
    .name("git-cognition-rs")
    .issue("42")
    .get();
```

## Provider Support

GitHub issues use the repository path:

```text
/repos/{owner}/{repo}/issues/{issue}
/repos/{owner}/{repo}/issues
```

GitLab issues use the URL-encoded project path:

```text
/api/v4/projects/{owner%2Frepo}/issues/{issue}
/api/v4/projects/{owner%2Frepo}/issues
```

Pagination remains provider-neutral in the caller. Providers map it to their own query names.

Bitbucket Cloud is intentionally not exposed through this facade.
Bitbucket Cloud still has legacy issue tracker endpoints, but Atlassian has announced that
native Bitbucket Cloud Issues are being removed in August 2026. The Bitbucket provider does not
advertise `Capability::Issues` and does not implement `ManagedIssueProvider`. Jira-backed work
tracking should be modeled as a separate extension instead of leaking Jira behavior into the
provider-neutral issue contract.

When a provider does not advertise `Capability::Issues`, issue operations must return
`CognitionError::UnsupportedOperation`. They must not return `TransportNotConfigured`, because that
would imply the provider supports issues and only lacks an HTTP client.

## Create, Update, Close, Delete

Create, update, close and delete requests should stay fluent at the call site:

```rust
let repo = github()
    .repo()
    .owner("akira-io")
    .name("git-cognition-rs")
    .get();

let create_request = github()
    .issue()
    .draft()
    .repo(repo.clone())
    .title("Fix pagination")
    .body("The cursor should be opaque.")
    .create();

let update_request = github()
    .issue()
    .repo(repo.clone())
    .id("42")
    .title("Fix pagination safely")
    .body("The cursor should remain opaque.")
    .update();

let close_request = github()
    .issue()
    .repo(repo)
    .id("42")
    .closed()
    .close();

let delete_request = gitlab()
    .issue()
    .repo(repo)
    .id("42")
    .delete()?;
```

The async client follows the same terminal action names:

```rust
gitlab()
    .transport(transport)
    .issues()
    .location(repo)
    .id("42")
    .delete()
    .await?;
```

GitHub issues do not expose a universal delete endpoint. Use `close` for GitHub issue lifecycle changes. Calling async `delete` on GitHub returns `CognitionError::UnsupportedOperation`.

Provider support:

| Provider | Create | Update | Close | Delete |
| --- | --- | --- | --- | --- |
| GitHub | supported | supported | supported | unsupported |
| GitLab | supported | supported | supported | supported |
| Bitbucket | unsupported | unsupported | unsupported | unsupported |
