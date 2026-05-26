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
            Capability::CodeReviews,
            Capability::Pipelines,
            Capability::PipelineGet,
            Capability::PipelineList,
            Capability::PipelineCancel,
            Capability::Webhooks,
        ])
        .does_not_support([
            Capability::Issues,
            Capability::Releases,
            Capability::Organizations,
            Capability::Discussions,
            Capability::SelfHosted,
            Capability::PipelineRerun,
        ])
        .auth(AuthKind::OAuth, AuthHeaderStyle::AuthorizationBearer)
        .check()
}
