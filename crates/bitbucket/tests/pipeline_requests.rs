use vcs_provider_bitbucket::{BitbucketPipeline, BitbucketPipelineCollection};
use vcs_provider_core::{Pipeline, PipelineId, RequestMethod, pipeline, repo};

#[test]
fn bitbucket_pipeline_get_targets_pipeline_endpoint() {
    assert_eq!(
        pipeline_resource().url().value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/pipelines/%7Bpipeline-uuid%7D"
    );
}

#[test]
fn bitbucket_pipeline_list_targets_pipelines_endpoint() {
    let repo = repo().owner("akira-io").name("vcs-providers-rs").get();
    let page = vcs_provider_core::pagination()
        .request()
        .limit(50)
        .cursor("2")
        .build();
    let query = pipeline().query().location(repo).pagination(page).list();

    assert_eq!(
        BitbucketPipelineCollection::default().list(&query).value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/pipelines?pagelen=50&page=2"
    );
}

#[test]
fn bitbucket_pipeline_cancel_builds_stop_request() {
    assert_eq!(pipeline_resource().cancel().method(), &RequestMethod::Post);
    assert_eq!(
        pipeline_resource().cancel().url().value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/pipelines/%7Bpipeline-uuid%7D/stopPipeline"
    );
}

fn pipeline_resource() -> BitbucketPipeline {
    BitbucketPipeline::make("https://api.bitbucket.org/2.0", pipeline_reference())
}

fn pipeline_reference() -> Pipeline {
    let repo = repo().owner("akira-io").name("vcs-providers-rs").get();

    Pipeline::make(repo, PipelineId::make("{pipeline-uuid}"))
}
