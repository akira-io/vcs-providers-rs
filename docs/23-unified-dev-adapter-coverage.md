# Unified Dev Adapter Coverage

`git-cognition-rs` is intended to own provider transport, endpoint semantics,
hydration, pagination, capabilities, and normalized errors. `unified-dev`
should keep only thin adapters that convert these resources into app DTOs.

## Current Provider Coverage

These paths are implemented across providers when the official provider
documentation exposes equivalent semantics:

| Area | GitHub | GitLab | Bitbucket | Notes |
| --- | --- | --- | --- | --- |
| Authentication | `validate` | `validate` | `validate` | Uses the authenticated-user endpoint through configured transport and auth headers. |
| Organizations | `list` | `list` | `list` | Maps organizations, groups, and workspaces into provider-neutral `Organization`. |
| Repositories | `get`, `list`, `search`, `create`, `update`, `delete` | `get`, `list`, `search`, `create`, `update`, `delete` | `get`, `list`, `search`, `create`, `update`, `delete` | Pagination stays opaque through `PageCursor`. |
| Branches | `list`, `create`, `delete` | `list`, `create`, `delete` | `list`, `create`, `delete` | Provider crates own branch endpoint and payload differences. |
| Issues | `get`, `list`, `create`, `update`, `close` | `get`, `list`, `create`, `update`, `close`, `delete` | `get`, `list`, `create`, `update`, `close`, `delete` | GitHub does not expose issue delete in REST. |
| Code reviews | `get`, `list`, `create`, `update`, `merge`, `close` | `get`, `list`, `create`, `update`, `merge`, `close`, `delete` | `get`, `list`, `create`, `update`, `merge`, `close` | Pull request comments and review submission are not yet covered. |
| Pipelines | `get`, `list`, `rerun`, `cancel` | `get`, `list`, `rerun`, `cancel` | `get`, `list`, `cancel` | Job logs and commit check aggregation are not yet covered. |
| Releases | `get`, `list`, `create`, `update`, `delete` | `get`, `list`, `create`, `update`, `delete` | Unsupported | Bitbucket Cloud exposes Downloads, not a release resource with equivalent semantics. |

Official sources used for this coverage:

| Provider | Area | Official source |
| --- | --- |
| GitHub | Auth validation | https://docs.github.com/en/rest/users/users |
| GitHub | Organization list | https://docs.github.com/en/rest/orgs/orgs |
| GitHub | Branch list | https://docs.github.com/en/rest/branches/branches |
| GitHub | Branch create/delete | https://docs.github.com/en/rest/git/refs |
| GitLab | Auth validation | https://docs.gitlab.com/api/users/ |
| GitLab | Group list | https://docs.gitlab.com/api/groups/ |
| GitLab | Branch create/delete | https://docs.gitlab.com/api/branches/ |
| Bitbucket | Auth validation and workspaces | https://developer.atlassian.com/cloud/bitbucket/rest/api-group-users/ |
| Bitbucket | Branch create/delete | https://developer.atlassian.com/cloud/bitbucket/rest/api-group-refs/ |
| Bitbucket | Issues | https://developer.atlassian.com/cloud/bitbucket/rest/api-group-issue-tracker/ |
| Bitbucket | Downloads | https://developer.atlassian.com/cloud/bitbucket/rest/api-group-downloads/ |

## Provider Differences

Callers must rely on `Capability` rather than provider names. Unsupported
operations must be based on official provider gaps, not implementation order.

## Adapter Rule

The `unified-dev` adapter should check capability support before exposing a UI
action or calling an operation. Unsupported operations must surface
`CognitionError::UnsupportedOperation` and should not fall back to direct provider API
calls inside the app.

```rust
use git_cognition_core::{Capability, cognition};
use git_cognition_bitbucket::bitbucket;

let provider = bitbucket();

if provider.capabilities().supports(&Capability::Releases) {
    // expose the "Create release" affordance
} else {
    // hide it; do not call provider HTTP directly as a workaround
}

// Local plane has its own capability set, separate from the remote one above.
use git_cognition_core::LocalGitCapability;

let local = cognition().local().repo("/workspace/project");
if local.capabilities().supports(&LocalGitCapability::MergePreview) {
    // expose merge preview UI
}
```
