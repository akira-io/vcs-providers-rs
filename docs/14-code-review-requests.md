# Code Review Requests

Code review request builders translate the universal `CodeReview` resource into provider-specific review endpoints.

Use the provider facade when the provider is known:

```rust
let code_review = github()
    .repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
    .code_review("42")
    .get();

let url = code_review.url();
```

Use the repository collection builder for list URLs:

```rust
let code_reviews = gitlab()
    .repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
    .code_reviews()
    .pagination()
    .limit(50)
    .list();

let url = code_reviews.url();
```

If the provider uses cursor pagination, keep the cursor in the same chain:

```rust
let code_reviews = bitbucket()
    .repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
    .code_reviews()
    .pagination()
    .limit(50)
    .cursor("2")
    .list();

let url = code_reviews.url();
```

If the repo already exists as a variable, pass it into the code review builder:

```rust
let repo = github()
    .repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
    .get();

let code_review = github()
    .code_review()
    .repo(repo)
    .id("42")
    .get();
```

Use `vcs(driver)` when the provider is injected:

```rust
let provider = vcs(gitlab());

let code_review = provider
    .repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
    .code_review("42")
    .get();
```

## Provider Support

GitHub maps code reviews to pull requests:

```text
/repos/{owner}/{repo}/pulls/{code_review}
/repos/{owner}/{repo}/pulls
```

GitLab maps code reviews to merge requests:

```text
/api/v4/projects/{owner%2Frepo}/merge_requests/{code_review}
/api/v4/projects/{owner%2Frepo}/merge_requests
```

Bitbucket maps code reviews to pull requests:

```text
/2.0/repositories/{workspace}/{repo_slug}/pullrequests/{code_review}
/2.0/repositories/{workspace}/{repo_slug}/pullrequests
```

Pagination remains provider-neutral in the caller. Providers map it to their own query names.

## Create, Update, Close, Delete

Use `CodeReviewDraft` to create code reviews and `CodeReviewPatch` to update or close them:

```rust
let repo = gitlab()
    .repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
    .get();

let create_request = gitlab()
    .code_review()
    .draft()
    .repo(repo.clone())
    .title("Add release mutations")
    .source("feature/releases")
    .target("main")
    .body("Adds release request builders.")
    .create();

let code_review = gitlab().code_review().repo(repo).id("42").get();
let code_review_patch = CodeReviewPatchBuilder::make(code_review.code_review().clone())
    .closed()
    .get();

let update_request = code_review.update(&code_review_patch);
let close_request = code_review.close();
let delete_request = code_review.delete();
```

Provider support:

| Provider | Create | Update | Close | Delete |
| --- | --- | --- | --- | --- |
| GitHub | supported | supported | supported | unsupported |
| GitLab | supported | supported | supported | supported |
| Bitbucket | supported | supported | supported | unsupported |
