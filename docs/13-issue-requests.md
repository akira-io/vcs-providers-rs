# Issue Requests

Issue request builders translate universal issue resources into provider-specific REST endpoints.

Use the provider facade when the provider is known:

```rust
let issue = github()
    .repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
    .issue("42")
    .get();

let url = issue.url();
```

Use the collection builder for list URLs:

```rust
let issues = gitlab()
    .repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
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
    .name("vcs-providers-rs")
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
    .name("vcs-providers-rs")
    .get();

let issue = github()
    .issue()
    .repo(repo)
    .id("42")
    .get();
```

Use `vcs(driver)` when the provider is injected:

```rust
let provider = vcs(gitlab());

let issue = provider
    .repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
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

## Create, Update, Close

Use `IssueDraft` to create issues and `IssuePatch` to update or close them:

```rust
let repo = github()
    .repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
    .get();

let create_request = github()
    .issue()
    .draft()
    .repo(repo.clone())
    .title("Fix pagination")
    .body("The cursor should be opaque.")
    .create();

let issue = github().issue().repo(repo).id("42").get();
let issue_patch = IssuePatchBuilder::make(issue.issue().clone())
    .closed()
    .get();

let update_request = issue.update(&issue_patch);
let close_request = issue.close(&issue_patch);
```

Provider support:

| Provider | Create | Update | Close | Delete |
| --- | --- | --- | --- | --- |
| GitHub | supported | supported | supported | unsupported |
| GitLab | supported | supported | supported | supported |
| Bitbucket | unsupported | unsupported | unsupported | unsupported |
