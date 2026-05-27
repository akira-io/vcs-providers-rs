use vcs_provider_bitbucket::{DISPLAY_NAME, PROVIDER_ID, bitbucket};
use vcs_provider_core::{AuthHeaderStyle, AuthKind, Capability, VcsResult, conformance};

#[test]
fn bitbucket_provider_passes_common_conformance_suite() -> VcsResult<()> {
    conformance()
        .provider(bitbucket())
        .id(PROVIDER_ID)
        .display_name(DISPLAY_NAME)
        .supports([
            Capability::Repos,
            Capability::RepoGet,
            Capability::RepoList,
            Capability::RepoSearch,
            Capability::RepoBranches,
            Capability::RepoCommits,
            Capability::RepoCreate,
            Capability::RepoUpdate,
            Capability::RepoDelete,
            Capability::CodeReviews,
            Capability::CodeReviewGet,
            Capability::CodeReviewList,
            Capability::CodeReviewCreate,
            Capability::CodeReviewUpdate,
            Capability::CodeReviewMerge,
            Capability::CodeReviewClose,
            Capability::Pipelines,
            Capability::PipelineGet,
            Capability::PipelineList,
            Capability::PipelineCancel,
        ])
        .does_not_support([
            Capability::Issues,
            Capability::IssueGet,
            Capability::IssueList,
            Capability::IssueCreate,
            Capability::IssueUpdate,
            Capability::IssueClose,
            Capability::IssueDelete,
            Capability::Releases,
            Capability::ReleaseGet,
            Capability::ReleaseList,
            Capability::ReleaseCreate,
            Capability::ReleaseUpdate,
            Capability::ReleaseDelete,
            Capability::Organizations,
            Capability::Discussions,
            Capability::Webhooks,
            Capability::SelfHosted,
            Capability::CodeReviewDelete,
            Capability::PipelineRerun,
        ])
        .auth(AuthKind::OAuth, AuthHeaderStyle::AuthorizationBearer)
        .check()
}
