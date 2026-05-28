use vcs_provider_core::{RequestMethod, VcsResult};
use vcs_provider_gitlab::gitlab;

#[test]
fn gitlab_pipeline_get_targets_pipeline_endpoint() {
    assert_eq!(
        pipeline_resource().url().value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fvcs-providers-rs/pipelines/42"
    );
}

#[test]
fn gitlab_pipeline_list_targets_pipelines_endpoint() {
    assert_eq!(
        gitlab()
            .repo()
            .owner("akira-io")
            .name("vcs-providers-rs")
            .pipelines()
            .pagination()
            .limit(50)
            .cursor("2")
            .url()
            .value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fvcs-providers-rs/pipelines?per_page=50&page=2"
    );
}

#[test]
fn gitlab_pipeline_rerun_builds_retry_request() -> VcsResult<()> {
    assert_eq!(pipeline_resource().rerun()?.method(), &RequestMethod::Post);

    Ok(())
}

#[test]
fn gitlab_pipeline_cancel_builds_post_request() -> VcsResult<()> {
    assert_eq!(pipeline_resource().cancel()?.method(), &RequestMethod::Post);

    Ok(())
}

fn pipeline_resource() -> vcs_provider_core::ManagedPipeline<vcs_provider_gitlab::GitLabProvider> {
    gitlab()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .pipeline("42")
        .get()
}
