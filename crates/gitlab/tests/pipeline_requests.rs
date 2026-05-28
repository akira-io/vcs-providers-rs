use git_cognition_core::{CognitionResult, RequestMethod};
use git_cognition_gitlab::gitlab;

#[test]
fn gitlab_pipeline_get_targets_pipeline_endpoint() {
    assert_eq!(
        pipeline_resource().url().value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fgit-cognition-rs/pipelines/42"
    );
}

#[test]
fn gitlab_pipeline_list_targets_pipelines_endpoint() {
    assert_eq!(
        gitlab()
            .repo()
            .owner("akira-io")
            .name("git-cognition-rs")
            .pipelines()
            .pagination()
            .limit(50)
            .cursor("2")
            .url()
            .value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fgit-cognition-rs/pipelines?per_page=50&page=2"
    );
}

#[test]
fn gitlab_pipeline_rerun_builds_retry_request() -> CognitionResult<()> {
    assert_eq!(pipeline_resource().rerun()?.method(), &RequestMethod::Post);

    Ok(())
}

#[test]
fn gitlab_pipeline_cancel_builds_post_request() -> CognitionResult<()> {
    assert_eq!(pipeline_resource().cancel()?.method(), &RequestMethod::Post);

    Ok(())
}

fn pipeline_resource() -> git_cognition_core::ManagedPipeline<git_cognition_gitlab::GitLabProvider>
{
    gitlab()
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .pipeline("42")
        .get()
}
