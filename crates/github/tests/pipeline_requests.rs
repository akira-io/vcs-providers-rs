use vcs_provider_core::{RequestMethod, VcsResult};
use vcs_provider_github::github;

#[test]
fn github_pipeline_get_targets_workflow_run_endpoint() {
    assert_eq!(
        pipeline_resource().url().value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/actions/runs/42"
    );
}

#[test]
fn github_pipeline_list_targets_workflow_runs_endpoint() {
    assert_eq!(
        github()
            .repo()
            .owner("akira-io")
            .name("vcs-providers-rs")
            .pipelines()
            .pagination()
            .limit(50)
            .cursor("2")
            .url()
            .value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/actions/runs?per_page=50&page=2"
    );
}

#[test]
fn github_pipeline_rerun_builds_post_request() -> VcsResult<()> {
    assert_eq!(pipeline_resource().rerun()?.method(), &RequestMethod::Post);

    Ok(())
}

#[test]
fn github_pipeline_cancel_builds_post_request() -> VcsResult<()> {
    assert_eq!(pipeline_resource().cancel()?.method(), &RequestMethod::Post);

    Ok(())
}

fn pipeline_resource() -> vcs_provider_core::ManagedPipeline<vcs_provider_github::GitHubProvider> {
    github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .pipeline("42")
        .get()
}
