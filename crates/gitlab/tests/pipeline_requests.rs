use vcs_provider_core::{Pipeline, PipelineId, RequestMethod, pipeline, repo};
use vcs_provider_gitlab::{GitLabPipeline, GitLabPipelineCollection};

#[test]
fn gitlab_pipeline_get_targets_pipeline_endpoint() {
    assert_eq!(
        pipeline_resource().url().value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fvcs-providers-rs/pipelines/42"
    );
}

#[test]
fn gitlab_pipeline_list_targets_pipelines_endpoint() {
    let repo = repo().owner("akira-io").name("vcs-providers-rs").get();
    let page = vcs_provider_core::pagination()
        .request()
        .limit(50)
        .cursor("2")
        .build();
    let query = pipeline().query().location(repo).pagination(page).list();

    assert_eq!(
        GitLabPipelineCollection::default().list(&query).value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fvcs-providers-rs/pipelines?per_page=50&page=2"
    );
}

#[test]
fn gitlab_pipeline_rerun_builds_retry_request() {
    assert_eq!(pipeline_resource().rerun().method(), &RequestMethod::Post);
}

#[test]
fn gitlab_pipeline_cancel_builds_post_request() {
    assert_eq!(pipeline_resource().cancel().method(), &RequestMethod::Post);
}

fn pipeline_resource() -> GitLabPipeline {
    GitLabPipeline::make("https://gitlab.com", pipeline_reference())
}

fn pipeline_reference() -> Pipeline {
    let repo = repo().owner("akira-io").name("vcs-providers-rs").get();

    Pipeline::make(repo, PipelineId::make("42"))
}
