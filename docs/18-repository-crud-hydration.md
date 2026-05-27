# Repository CRUD Hydration

Repository runtime clients expose read and write operations through the universal `Repos` contract.

```rust
let repo = repo().owner("akira-io").name("vcs-providers-rs").get();

let repository = vcs(github())
    .transport(http().transport().get()?)
    .auth(auth().personal_access_token("token"))
    .repos()
    .create()
    .location(repo)
    .visibility(Visibility::Private)
    .create()
    .await?;
```

The same contract is implemented by GitHub, GitLab, and Bitbucket clients:

| Operation | Return | Notes |
| --- | --- | --- |
| `get(repo)` | `Repository` | Hydrates one provider response into a provider-neutral repository. |
| `list(query)` | `Page<Repository>` | Preserves provider-neutral pagination input. |
| `search(query)` | `Page<Repository>` | Maps provider search response shapes internally. |
| `create(draft)` | `Repository` | Sends the provider create request and hydrates the response. |
| `update(patch)` | `Repository` | Sends the provider update request and hydrates the response. |
| `delete(repo)` | `()` | Accepts provider success statuses without requiring a response body. |
| `branches(repo)` | `Page<Branch>` | Hydrates provider branch responses. |
| `commits(repo)` | `Page<Commit>` | Hydrates provider commit responses. |

Provider-specific payload structs remain private inside provider crates.

## Provider Research

The implementation was checked against official provider documentation:

| Provider | Create | Update | Delete |
| --- | --- | --- | --- |
| GitHub | `POST /orgs/{org}/repos` | `PATCH /repos/{owner}/{repo}` | `DELETE /repos/{owner}/{repo}` |
| GitLab | `POST /projects` | `PUT /projects/:id` | `DELETE /projects/:id` |
| Bitbucket | `PUT /repositories/{workspace}/{repo_slug}` | `PUT /repositories/{workspace}/{repo_slug}` | `DELETE /repositories/{workspace}/{repo_slug}` |

Sources:

- [GitHub repository REST endpoints](https://docs.github.com/en/rest/repos/repos)
- [GitLab Projects API](https://docs.gitlab.com/api/projects/)
- [Bitbucket repositories API](https://developer.atlassian.com/cloud/bitbucket/rest/api-group-repositories/)

## Delete Semantics

Delete returns `VcsResult<()>` because providers commonly return a success status with no resource body. If the transport returns a non-success status, the universal error mapper still converts it into `VcsError`.
