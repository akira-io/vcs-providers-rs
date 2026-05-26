use vcs_provider_core::{IssuePatchBuilder, RequestMethod};
use vcs_provider_gitlab::gitlab;

#[test]
fn gitlab_issue_get_targets_repository_endpoint() {
    let issue_resource = gitlab()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .issue("42")
        .get();

    assert_eq!(
        issue_resource.url().value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fvcs-providers-rs/issues/42"
    );
}

#[test]
fn gitlab_issue_list_targets_repository_endpoint() {
    let issues = gitlab()
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
        "https://gitlab.com/api/v4/projects/akira-io%2Fvcs-providers-rs/issues?per_page=50&page=2"
    );
}

#[test]
fn gitlab_issue_builder_accepts_existing_repo() {
    let repo = gitlab()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let issue_resource = gitlab().issue().repo(repo).id("42").get();

    assert_eq!(
        issue_resource.url().value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fvcs-providers-rs/issues/42"
    );
}

#[test]
fn gitlab_issue_create_builds_post_request() {
    let repo = gitlab()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let create_request = gitlab()
        .issue()
        .draft()
        .repo(repo.clone())
        .title("Track mutable issue requests")
        .body("Details")
        .create();

    assert_eq!(create_request.method(), &RequestMethod::Post);
    assert_eq!(
        request_body(&create_request),
        Some(r#"{"title":"Track mutable issue requests","description":"Details"}"#)
    );
}

#[test]
fn gitlab_issue_update_builds_put_request() {
    let repo = gitlab()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let issue_resource = gitlab().issue().repo(repo).id("42").get();
    let issue_patch = IssuePatchBuilder::make(issue_resource.issue().clone())
        .title("Track mutable issue requests safely")
        .body("Updated details")
        .get();
    let update_request = issue_resource.update(&issue_patch);

    assert_eq!(update_request.method(), &RequestMethod::Put);
    assert_eq!(
        request_body(&update_request),
        Some(r#"{"title":"Track mutable issue requests safely","description":"Updated details"}"#)
    );
}

#[test]
fn gitlab_issue_close_builds_put_request() {
    let repo = gitlab()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let issue_resource = gitlab().issue().repo(repo).id("42").get();
    let issue_patch = IssuePatchBuilder::make(issue_resource.issue().clone())
        .closed()
        .get();

    let close_request = issue_resource.close(&issue_patch);

    assert_eq!(close_request.method(), &RequestMethod::Put);
    assert_eq!(
        request_body(&close_request),
        Some(r#"{"state_event":"close"}"#)
    );
}

#[test]
fn gitlab_issue_delete_builds_delete_request() {
    let repo = gitlab()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let issue_resource = gitlab().issue().repo(repo).id("42").get();

    assert_eq!(issue_resource.delete().method(), &RequestMethod::Delete);
}

fn request_body(request: &vcs_provider_core::Request) -> Option<&str> {
    request.body().map(vcs_provider_core::RequestBody::as_str)
}
