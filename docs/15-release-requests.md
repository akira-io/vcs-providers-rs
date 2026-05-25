# Release Requests

Release request builders translate the universal `Release` resource into provider-specific release endpoints.

Use the provider facade when the provider is known:

```rust
let release = github()
    .repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
    .release("123")
    .build();

let url = release.url();
```

Use the repository collection builder for list URLs:

```rust
let releases = gitlab()
    .repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
    .releases()
    .pagination()
    .limit(50)
    .get();

let url = releases.url();
```

If the repo already exists as a variable, pass it into the release builder:

```rust
let repo = github()
    .repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
    .build();

let release = github()
    .release()
    .repo(repo)
    .id("123")
    .build();
```

Use `vcs(driver)` when the provider is injected:

```rust
let provider = vcs(gitlab());

let release = provider
    .repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
    .release("v1.0.0")
    .build();
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

Pagination remains provider-neutral in the caller. Providers map it to their own query names.

## Mutations

Use `ReleaseDraft` to create releases and `ReleasePatch` to update release notes:

```rust
let repo = github()
    .repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
    .build();

let draft = release()
    .draft()
    .repo(repo.clone())
    .tag("v1.0.0")
    .name("v1.0.0")
    .body("Release notes")
    .build();

let create_request = github().release().collection().create(&draft);

let release = github().release().repo(repo).id("123").build();
let patch = ReleasePatchBuilder::make(release.release().clone())
    .body("Updated release notes")
    .build();

let update_request = release.update(&patch);
let delete_request = release.delete();
```

Provider support:

| Provider | Create | Update | Delete |
| --- | --- | --- | --- |
| GitHub | supported | supported | supported |
| GitLab | supported | supported | supported |
| Bitbucket | unsupported | unsupported | unsupported |
