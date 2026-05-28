use git_cognition_core::{CognitionResult, RequestMethod};
use git_cognition_github::github;

#[test]
fn github_pipeline_get_targets_workflow_run_endpoint() {
    assert_eq!(
        pipeline_resource().url().value(),
        "https://api.github.com/repos/akira-io/git-cognition-rs/actions/runs/42"
    );
}

#[test]
fn github_pipeline_list_targets_workflow_runs_endpoint() {
    assert_eq!(
        github()
            .repo()
            .owner("akira-io")
            .name("git-cognition-rs")
            .pipelines()
            .pagination()
            .limit(50)
            .cursor("2")
            .url()
            .value(),
        "https://api.github.com/repos/akira-io/git-cognition-rs/actions/runs?per_page=50&page=2"
    );
}

#[test]
fn github_pipeline_rerun_builds_post_request() -> CognitionResult<()> {
    assert_eq!(pipeline_resource().rerun()?.method(), &RequestMethod::Post);

    Ok(())
}

#[test]
fn github_pipeline_cancel_builds_post_request() -> CognitionResult<()> {
    assert_eq!(pipeline_resource().cancel()?.method(), &RequestMethod::Post);

    Ok(())
}

fn pipeline_resource() -> git_cognition_core::ManagedPipeline<git_cognition_github::GitHubProvider>
{
    github()
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .pipeline("42")
        .get()
}
