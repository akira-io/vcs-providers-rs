use vcs_provider_core::{Pipeline, PipelineId, RequestMethod, pipeline, repo};
use vcs_provider_github::{GitHubPipeline, GitHubPipelineCollection};

#[test]
fn github_pipeline_get_targets_workflow_run_endpoint() {
    assert_eq!(
        pipeline_resource().url().value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/actions/runs/42"
    );
}

#[test]
fn github_pipeline_list_targets_workflow_runs_endpoint() {
    let repo = repo().owner("akira-io").name("vcs-providers-rs").get();
    let page = vcs_provider_core::pagination()
        .request()
        .limit(50)
        .cursor("2")
        .build();
    let query = pipeline().query().list(repo, Some(page));

    assert_eq!(
        GitHubPipelineCollection::default().list(&query).value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/actions/runs?per_page=50&page=2"
    );
}

#[test]
fn github_pipeline_rerun_builds_post_request() {
    assert_eq!(pipeline_resource().rerun().method(), &RequestMethod::Post);
}

#[test]
fn github_pipeline_cancel_builds_post_request() {
    assert_eq!(pipeline_resource().cancel().method(), &RequestMethod::Post);
}

fn pipeline_resource() -> GitHubPipeline {
    GitHubPipeline::make("https://api.github.com", pipeline_reference())
}

fn pipeline_reference() -> Pipeline {
    let repo = repo().owner("akira-io").name("vcs-providers-rs").get();

    Pipeline::make(repo, PipelineId::make("42"))
}
