use vcs_provider_core::{AuthHeaderStyle, AuthKind, Capability, VcsResult, conformance};
use vcs_provider_github::{DISPLAY_NAME, PROVIDER_ID, github};

#[test]
fn github_provider_passes_common_conformance_suite() -> VcsResult<()> {
    conformance()
        .provider(github())
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
            Capability::Discussions,
            Capability::Webhooks,
        ])
        .does_not_support([Capability::SelfHosted])
        .auth(
            AuthKind::PersonalAccessToken,
            AuthHeaderStyle::AuthorizationBearer,
        )
        .check()
}
