use vcs_provider_core::RequestMethod;
use vcs_provider_github::github;

#[test]
fn github_issue_get_targets_repository_endpoint() {
    let issue_resource = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .issue("42")
        .get();

    assert_eq!(
        issue_resource.url().value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/issues/42"
    );
}

#[test]
fn github_issue_list_targets_repository_endpoint() {
    let issues = github()
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
        "https://api.github.com/repos/akira-io/vcs-providers-rs/issues?per_page=50&page=2"
    );
}

#[test]
fn github_issue_builder_accepts_existing_repo() {
    let repo = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let issue_resource = github().issue().repo(repo).id("42").get();

    assert_eq!(
        issue_resource.url().value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/issues/42"
    );
}

#[test]
fn github_issue_create_builds_post_request() {
    let repo = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let create_request = github()
        .issue()
        .draft()
        .repo(repo.clone())
        .title("Track mutable issue requests")
        .body("Details")
        .create();

    assert_eq!(create_request.method(), &RequestMethod::Post);
    assert_eq!(
        request_body(&create_request),
        Some(r#"{"title":"Track mutable issue requests","body":"Details"}"#)
    );
}

#[test]
fn github_issue_update_builds_patch_request() {
    let repo = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let update_request = github()
        .issue()
        .repo(repo)
        .id("42")
        .title("Track mutable issue requests safely")
        .body("Updated details")
        .update();

    assert_eq!(update_request.method(), &RequestMethod::Patch);
    assert_eq!(
        request_body(&update_request),
        Some(r#"{"title":"Track mutable issue requests safely","body":"Updated details"}"#)
    );
}

#[test]
fn github_issue_close_builds_patch_request() {
    let repo = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let close_request = github().issue().repo(repo).id("42").closed().close();

    assert_eq!(close_request.method(), &RequestMethod::Patch);
    assert_eq!(request_body(&close_request), Some(r#"{"state":"closed"}"#));
}

fn request_body(request: &vcs_provider_core::Request) -> Option<&str> {
    request.body().map(vcs_provider_core::RequestBody::as_str)
}
