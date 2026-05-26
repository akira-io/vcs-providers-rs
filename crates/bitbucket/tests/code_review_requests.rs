use vcs_provider_bitbucket::bitbucket;
use vcs_provider_core::RequestMethod;

#[test]
fn bitbucket_code_review_get_targets_repository_endpoint() {
    assert_eq!(
        code_review_resource().url().value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/pullrequests/42"
    );
}

#[test]
fn bitbucket_code_review_list_targets_repository_endpoint() {
    let code_reviews = bitbucket()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .code_reviews()
        .pagination()
        .limit(50)
        .cursor("2")
        .list();

    assert_eq!(
        code_reviews.url().value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/pullrequests?pagelen=50&page=2"
    );
}

#[test]
fn bitbucket_code_review_builder_accepts_existing_repo() {
    assert_eq!(
        bitbucket()
            .code_review()
            .repo(repository())
            .id("42")
            .get()
            .url()
            .value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/pullrequests/42"
    );
}

#[test]
fn bitbucket_code_review_create_builds_post_request() {
    let create_request = bitbucket()
        .code_review()
        .draft()
        .repo(repository())
        .title("Add mutable operations")
        .source("feature")
        .target("main")
        .body("Details")
        .create();

    assert_eq!(create_request.method(), &RequestMethod::Post);
    assert_eq!(
        request_body(&create_request),
        Some(
            r#"{"title":"Add mutable operations","source":{"branch":{"name":"feature"}},"destination":{"branch":{"name":"main"}},"description":"Details"}"#
        )
    );
}

#[test]
fn bitbucket_code_review_update_builds_put_request() {
    let update_request = bitbucket()
        .code_review()
        .repo(repository())
        .id("42")
        .title("Add safe mutable operations")
        .body("Updated details")
        .update();

    assert_eq!(update_request.method(), &RequestMethod::Put);
    assert_eq!(
        request_body(&update_request),
        Some(r#"{"title":"Add safe mutable operations","description":"Updated details"}"#)
    );
}

#[test]
fn bitbucket_code_review_merge_builds_post_request() {
    let merge_request = code_review_resource().merge();

    assert_eq!(merge_request.method(), &RequestMethod::Post);
    assert_eq!(
        merge_request.url().as_str(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/pullrequests/42/merge"
    );
}

#[test]
fn bitbucket_code_review_close_builds_decline_request() {
    assert_eq!(
        code_review_resource().close().method(),
        &RequestMethod::Post
    );
}

fn repository() -> vcs_provider_core::ManagedRepo<vcs_provider_bitbucket::BitbucketProvider> {
    bitbucket()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get()
}

fn code_review_resource()
-> vcs_provider_core::ManagedCodeReview<vcs_provider_bitbucket::BitbucketProvider> {
    bitbucket().code_review().repo(repository()).id("42").get()
}

fn request_body(request: &vcs_provider_core::Request) -> Option<&str> {
    request.body().map(vcs_provider_core::RequestBody::as_str)
}
