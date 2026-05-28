use vcs_provider_bitbucket::bitbucket;
use vcs_provider_core::RequestMethod;

#[test]
fn bitbucket_issue_get_targets_repository_endpoint() {
    let issue_resource = bitbucket()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .issue("42")
        .get();

    assert_eq!(
        issue_resource.url().value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/issues/42"
    );
}

#[test]
fn bitbucket_issue_list_targets_repository_endpoint() {
    let issues = bitbucket()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .issues()
        .pagination()
        .limit(50)
        .cursor("2")
        .list();

    assert_eq!(
        issues.url().value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/issues?pagelen=50&page=2"
    );
}

#[test]
fn bitbucket_issue_create_builds_post_request() {
    let repo = bitbucket()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let create_request = bitbucket()
        .issue()
        .draft()
        .repo(repo)
        .title("Track mutable issue requests")
        .body("Details")
        .create();

    assert_eq!(create_request.method(), &RequestMethod::Post);
    assert_eq!(
        request_body(&create_request),
        Some(r#"{"title":"Track mutable issue requests","content":{"raw":"Details"}}"#)
    );
}

#[test]
fn bitbucket_issue_update_builds_put_request() {
    let repo = bitbucket()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let update_request = bitbucket()
        .issue()
        .repo(repo)
        .id("42")
        .title("Track mutable issue requests safely")
        .body("Updated details")
        .update();

    assert_eq!(update_request.method(), &RequestMethod::Put);
    assert_eq!(
        request_body(&update_request),
        Some(
            r#"{"title":"Track mutable issue requests safely","content":{"raw":"Updated details"}}"#
        )
    );
}

#[test]
fn bitbucket_issue_close_builds_put_request() {
    let repo = bitbucket()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let close_request = bitbucket().issue().repo(repo).id("42").closed().close();

    assert_eq!(close_request.method(), &RequestMethod::Put);
    assert_eq!(
        request_body(&close_request),
        Some(r#"{"state":"resolved"}"#)
    );
}

#[test]
fn bitbucket_issue_delete_builds_delete_request() -> vcs_provider_core::VcsResult<()> {
    let repo = bitbucket()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let issue_resource = bitbucket().issue().repo(repo).id("42").get();
    let delete_request = issue_resource.delete()?;

    assert_eq!(delete_request.method(), &RequestMethod::Delete);

    Ok(())
}

fn request_body(request: &vcs_provider_core::Request) -> Option<&str> {
    request.body().map(vcs_provider_core::RequestBody::as_str)
}
