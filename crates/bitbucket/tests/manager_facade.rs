use vcs_provider_bitbucket::{BitbucketProvider, bitbucket};
use vcs_provider_core::{RequestMethod, vcs};

#[test]
fn bitbucket_facade_builds_repo_requests() {
    let repository = vcs(bitbucket())
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();

    assert_eq!(
        repository.url().value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs"
    );
}

#[test]
fn bitbucket_facade_builds_code_review_requests() {
    let code_review = vcs(bitbucket())
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .code_review("42")
        .get();

    assert_eq!(
        code_review.url().value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/pullrequests/42"
    );
}

#[test]
fn bitbucket_facade_builds_pipeline_requests() -> vcs_provider_core::VcsResult<()> {
    let pipeline = vcs(bitbucket())
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .pipeline("{pipeline-uuid}")
        .get();
    let pipelines = vcs(bitbucket())
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .pipelines()
        .pagination()
        .limit(50)
        .cursor("2")
        .url();

    assert_eq!(
        pipeline.url().value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/pipelines/%7Bpipeline-uuid%7D"
    );
    assert_eq!(
        pipelines.value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/pipelines?pagelen=50&page=2"
    );
    assert_eq!(pipeline.cancel()?.method(), &RequestMethod::Post);

    Ok(())
}

#[test]
fn bitbucket_facade_builds_mutation_requests() {
    let create_request = vcs(bitbucket())
        .repo()
        .draft(repository())
        .visibility(vcs_provider_core::Visibility::Private)
        .create();

    assert_eq!(create_request.method(), &RequestMethod::Put);
}

fn repository() -> vcs_provider_core::ManagedRepo<BitbucketProvider> {
    vcs(bitbucket())
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get()
}
