# Collaboration Resource Hydration

This document covers hydrated client behavior for issues, code reviews and releases.

The low-level request builders remain available for URL and request inspection. Application code should prefer the client contracts when it wants hydrated resources.

## Provider Coverage

| Resource | GitHub | GitLab | Bitbucket |
| --- | --- | --- | --- |
| Issues | Supported | Supported | Not enabled in core capabilities |
| Code reviews | Pull requests | Merge requests | Pull requests |
| Releases | Supported | Supported | Not enabled in core capabilities |

Bitbucket pull requests are modeled as universal code reviews. Bitbucket issues and releases are intentionally not exposed as first-class supported capabilities in this version because current Bitbucket Cloud workflows commonly depend on Jira or repository issue tracker settings, and downloads are not equivalent to universal releases.

## Issues

GitHub and GitLab issue clients hydrate provider responses into provider-neutral `Issue` resources.

```rust
use vcs_provider_core::IssuesFluent;
use vcs_provider_github::github;

let repo = github()
    .repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
    .get();

let created_issue = github()
    .transport(transport)
    .issues()
    .location(repo.clone().into())
    .title("Fix payment state")
    .body("Details")
    .create()
    .await?;

let issue_page = github()
    .transport(transport)
    .issues()
    .location(repo.clone().into())
    .list()
    .await?;

let closed_issue = github()
    .transport(transport)
    .issues()
    .location(repo.into())
    .id(created_issue.id().as_str())
    .close()
    .await?;
```

The fluent operation names are the action names: `create`, `update`, `close` and `delete`. GitLab issue deletion maps to the GitLab issue delete endpoint. GitHub issue deletion is intentionally reported as `VcsError::UnsupportedOperation`; GitHub callers should use `close`.

## Code Reviews

Code reviews normalize GitHub pull requests, GitLab merge requests and Bitbucket pull requests behind the `CodeReview` resource.

```rust
use vcs_provider_core::CodeReviewsFluent;
use vcs_provider_gitlab::gitlab;

let repo = gitlab()
    .repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
    .get();

let created_code_review = gitlab()
    .transport(transport)
    .code_reviews()
    .location(repo.clone().into())
    .title("Add collaboration hydration")
    .source("feature")
    .target("main")
    .body("Details")
    .create()
    .await?;

let code_review_page = gitlab()
    .transport(transport)
    .code_reviews()
    .location(repo.clone().into())
    .list()
    .await?;

let updated_code_review = gitlab()
    .transport(transport)
    .code_reviews()
    .location(repo.into())
    .id(created_code_review.id().as_str())
    .title("Add hydrated collaboration resources")
    .update()
    .await?;

gitlab()
    .transport(transport)
    .code_reviews()
    .location(repo.into())
    .id(created_code_review.id().as_str())
    .delete()
    .await?;
```

GitLab merge request deletion is available through `delete`. GitHub and Bitbucket code reviews use `close` for lifecycle changes and report `VcsError::UnsupportedOperation` for async `delete`.

## Releases

GitHub and GitLab releases hydrate into the universal `Release` resource.

```rust
use vcs_provider_core::ReleasesFluent;
use vcs_provider_github::github;

let repo = github()
    .repo()
    .owner("akira-io")
    .name("vcs-providers-rs")
    .get();

let created_release = github()
    .transport(transport)
    .releases()
    .location(repo.clone().into())
    .tag("v1.0.0")
    .name("v1.0.0")
    .body("Release notes")
    .create()
    .await?;

let release_page = github()
    .transport(transport)
    .releases()
    .location(repo.clone().into())
    .list()
    .await?;

github()
    .transport(transport)
    .releases()
    .location(repo.into())
    .id(created_release.id().as_str())
    .delete()
    .await?;
```

## Response Mapping

Provider mappers keep payload details out of the public API:

| Provider | Issue ID source | Code review ID source | Release ID source |
| --- | --- | --- | --- |
| GitHub | `number` | `number` | `id` |
| GitLab | `iid` | `iid` | `tag_name` |
| Bitbucket | Not mapped | `id` | Not mapped |

The core contracts expose only `Issue`, `CodeReview`, `Release`, `Page<T>` and universal errors.

## Research References

This implementation was checked against current official provider documentation:

- GitHub REST issues: https://docs.github.com/en/rest/issues/issues
- GitHub REST pulls: https://docs.github.com/en/rest/pulls/pulls
- GitHub REST releases: https://docs.github.com/en/rest/releases/releases
- GitLab issues API: https://docs.gitlab.com/api/issues/
- GitLab merge requests API: https://docs.gitlab.com/api/merge_requests/
- GitLab releases API: https://docs.gitlab.com/api/releases/
- Bitbucket pull requests API: https://developer.atlassian.com/cloud/bitbucket/rest/api-group-pullrequests/
