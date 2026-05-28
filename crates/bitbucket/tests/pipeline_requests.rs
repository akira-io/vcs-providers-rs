use git_cognition_bitbucket::bitbucket;
use git_cognition_core::{CognitionResult, RequestMethod};

#[test]
fn bitbucket_pipeline_get_targets_pipeline_endpoint() {
    assert_eq!(
        pipeline_resource().url().value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/git-cognition-rs/pipelines/%7Bpipeline-uuid%7D"
    );
}

#[test]
fn bitbucket_pipeline_list_targets_pipelines_endpoint() {
    assert_eq!(
        bitbucket()
            .repo()
            .owner("akira-io")
            .name("git-cognition-rs")
            .pipelines()
            .pagination()
            .limit(50)
            .cursor("2")
            .url()
            .value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/git-cognition-rs/pipelines?pagelen=50&page=2"
    );
}

#[test]
fn bitbucket_pipeline_cancel_builds_stop_request() -> CognitionResult<()> {
    assert_eq!(pipeline_resource().cancel()?.method(), &RequestMethod::Post);
    assert_eq!(
        pipeline_resource().cancel()?.url().value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/git-cognition-rs/pipelines/%7Bpipeline-uuid%7D/stopPipeline"
    );

    Ok(())
}

fn pipeline_resource()
-> git_cognition_core::ManagedPipeline<git_cognition_bitbucket::BitbucketProvider> {
    bitbucket()
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .pipeline("{pipeline-uuid}")
        .get()
}
