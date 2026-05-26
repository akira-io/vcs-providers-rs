use vcs_provider_core::{AuthHeaderStyle, AuthKind, Capability, VcsResult, conformance};
use vcs_provider_gitlab::{DISPLAY_NAME, PROVIDER_ID, gitlab};

#[test]
fn gitlab_provider_passes_common_conformance_suite() -> VcsResult<()> {
    conformance()
        .provider(gitlab())
        .id(PROVIDER_ID)
        .display_name(DISPLAY_NAME)
        .supports([
            Capability::Repos,
            Capability::Issues,
            Capability::CodeReviews,
            Capability::Pipelines,
            Capability::PipelineGet,
            Capability::PipelineList,
            Capability::PipelineRerun,
            Capability::PipelineCancel,
            Capability::Releases,
            Capability::Organizations,
            Capability::Webhooks,
            Capability::SelfHosted,
        ])
        .does_not_support([Capability::Discussions])
        .auth(
            AuthKind::PersonalAccessToken,
            AuthHeaderStyle::CustomHeader("private-token".into()),
        )
        .check()
}
