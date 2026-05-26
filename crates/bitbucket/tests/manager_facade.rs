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
