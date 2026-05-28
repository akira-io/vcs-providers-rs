# Release Requests

Release request builders translate the universal `Release` resource into provider-specific release endpoints.

Use the provider facade when the provider is known:

```rust
let release = github()
    .repo()
    .owner("akira-io")
    .name("git-cognition-rs")
    .release("123")
    .get();

let url = release.url();
```

Use the repository collection builder for list URLs:

```rust
let releases = gitlab()
    .repo()
    .owner("akira-io")
    .name("git-cognition-rs")
    .releases()
    .pagination()
    .limit(50)
    .list();

let url = releases.url();
```

If the repo already exists as a variable, pass it into the release builder:

```rust
let repo = github()
    .repo()
    .owner("akira-io")
    .name("git-cognition-rs")
    .get();

let release = github()
    .release()
    .repo(repo)
    .id("123")
    .get();
```

Use `cognition().provider(driver)` when the provider is injected:

```rust
let provider = cognition().provider(gitlab());

let release = provider
    .repo()
    .owner("akira-io")
    .name("git-cognition-rs")
    .release("v1.0.0")
    .get();
```

## Provider Support

GitHub releases are created with `tag_name`, but resource operations address the GitHub release id:

```text
/repos/{owner}/{repo}/releases/{release_id}
/repos/{owner}/{repo}/releases
```

GitLab releases are addressed by tag name through the project releases endpoint:

```text
/api/v4/projects/{owner%2Frepo}/releases/{release}
/api/v4/projects/{owner%2Frepo}/releases
```

Bitbucket Cloud is intentionally not exposed through this facade. Bitbucket Cloud has repository downloads, but downloads are not equivalent to provider-neutral releases. A Bitbucket downloads extension can model that behavior without weakening the release contract.

When a provider does not advertise `Capability::Releases`, release operations must return
`CognitionError::UnsupportedOperation`. They must not return `TransportNotConfigured`, because that
would imply the provider supports releases and only lacks an HTTP client.

Pagination remains provider-neutral in the caller. Providers map it to their own query names.

## Create, Update, Delete

Create and update requests should stay fluent at the call site:

```rust
let repo = github()
    .repo()
    .owner("akira-io")
    .name("git-cognition-rs")
    .get();

let create_request = github()
    .release()
    .draft()
    .repo(repo.clone())
    .tag("v1.0.0")
    .name("v1.0.0")
    .body("Release notes")
    .create();

let update_request = github()
    .release()
    .repo(repo.clone())
    .id("123")
    .body("Updated release notes")
    .update();

let release = github().release().repo(repo).id("123").get();
let delete_request = release.delete();
```

Provider support:

| Provider | Create | Update | Delete |
| --- | --- | --- | --- |
| GitHub | supported | supported | supported |
| GitLab | supported | supported | supported |
| Bitbucket | unsupported | unsupported | unsupported |
