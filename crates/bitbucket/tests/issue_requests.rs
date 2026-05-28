use git_cognition_bitbucket::bitbucket;
use git_cognition_core::RequestMethod;

#[test]
fn bitbucket_issue_get_targets_repository_endpoint() {
    let issue_resource = bitbucket()
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .issue("42")
        .get();

    assert_eq!(
        issue_resource.url().value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/git-cognition-rs/issues/42"
    );
}

#[test]
fn bitbucket_issue_list_targets_repository_endpoint() {
    let issues = bitbucket()
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .issues()
        .pagination()
        .limit(50)
        .cursor("2")
        .list();

    assert_eq!(
        issues.url().value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/git-cognition-rs/issues?pagelen=50&page=2"
    );
}

#[test]
fn bitbucket_issue_create_builds_post_request() {
    let repo = bitbucket()
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
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
        .name("git-cognition-rs")
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
        .name("git-cognition-rs")
        .get();
    let close_request = bitbucket().issue().repo(repo).id("42").closed().close();

    assert_eq!(close_request.method(), &RequestMethod::Put);
    assert_eq!(
        request_body(&close_request),
        Some(r#"{"state":"resolved"}"#)
    );
}

#[test]
fn bitbucket_issue_delete_builds_delete_request() -> git_cognition_core::CognitionResult<()> {
    let repo = bitbucket()
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .get();
    let issue_resource = bitbucket().issue().repo(repo).id("42").get();
    let delete_request = issue_resource.delete()?;

    assert_eq!(delete_request.method(), &RequestMethod::Delete);

    Ok(())
}

fn request_body(request: &git_cognition_core::Request) -> Option<&str> {
    request.body().map(git_cognition_core::RequestBody::as_str)
}
