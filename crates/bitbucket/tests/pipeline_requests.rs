use vcs_provider_bitbucket::bitbucket;
use vcs_provider_core::{RequestMethod, VcsResult};

#[test]
fn bitbucket_pipeline_get_targets_pipeline_endpoint() {
    assert_eq!(
        pipeline_resource().url().value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/pipelines/%7Bpipeline-uuid%7D"
    );
}

#[test]
fn bitbucket_pipeline_list_targets_pipelines_endpoint() {
    assert_eq!(
        bitbucket()
            .repo()
            .owner("akira-io")
            .name("vcs-providers-rs")
            .pipelines()
            .pagination()
            .limit(50)
            .cursor("2")
            .url()
            .value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/pipelines?pagelen=50&page=2"
    );
}

#[test]
fn bitbucket_pipeline_cancel_builds_stop_request() -> VcsResult<()> {
    assert_eq!(pipeline_resource().cancel()?.method(), &RequestMethod::Post);
    assert_eq!(
        pipeline_resource().cancel()?.url().value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/pipelines/%7Bpipeline-uuid%7D/stopPipeline"
    );

    Ok(())
}

fn pipeline_resource()
-> vcs_provider_core::ManagedPipeline<vcs_provider_bitbucket::BitbucketProvider> {
    bitbucket()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .pipeline("{pipeline-uuid}")
        .get()
}
