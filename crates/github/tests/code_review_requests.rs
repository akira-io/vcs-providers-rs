use git_cognition_core::RequestMethod;
use git_cognition_github::github;

#[test]
fn github_code_review_get_targets_repository_endpoint() {
    assert_eq!(
        code_review_resource().url().value(),
        "https://api.github.com/repos/akira-io/git-cognition-rs/pulls/42"
    );
}

#[test]
fn github_code_review_list_targets_repository_endpoint() {
    let code_reviews = github()
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .code_reviews()
        .pagination()
        .limit(50)
        .cursor("2")
        .list();

    assert_eq!(
        code_reviews.url().value(),
        "https://api.github.com/repos/akira-io/git-cognition-rs/pulls?per_page=50&page=2"
    );
}

#[test]
fn github_code_review_builder_accepts_existing_repo() {
    assert_eq!(
        github()
            .code_review()
            .repo(repository())
            .id("42")
            .get()
            .url()
            .value(),
        "https://api.github.com/repos/akira-io/git-cognition-rs/pulls/42"
    );
}

#[test]
fn github_code_review_create_builds_post_request() {
    let create_request = github()
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
            r#"{"title":"Add mutable operations","head":"feature","base":"main","body":"Details"}"#
        )
    );
}

#[test]
fn github_code_review_update_builds_patch_request() {
    let update_request = github()
        .code_review()
        .repo(repository())
        .id("42")
        .title("Add safe mutable operations")
        .body("Updated details")
        .update();

    assert_eq!(update_request.method(), &RequestMethod::Patch);
    assert_eq!(
        request_body(&update_request),
        Some(r#"{"title":"Add safe mutable operations","body":"Updated details"}"#)
    );
}

#[test]
fn github_code_review_merge_builds_put_request() {
    let merge_request = code_review_resource().merge();

    assert_eq!(merge_request.method(), &RequestMethod::Put);
    assert_eq!(
        merge_request.url().as_str(),
        "https://api.github.com/repos/akira-io/git-cognition-rs/pulls/42/merge"
    );
}

#[test]
fn github_code_review_close_builds_patch_request() {
    let close_request = code_review_resource().close();

    assert_eq!(close_request.method(), &RequestMethod::Patch);
    assert_eq!(request_body(&close_request), Some(r#"{"state":"closed"}"#));
}

fn repository() -> git_cognition_core::ManagedRepo<git_cognition_github::GitHubProvider> {
    github()
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .get()
}

fn code_review_resource()
-> git_cognition_core::ManagedCodeReview<git_cognition_github::GitHubProvider> {
    github().code_review().repo(repository()).id("42").get()
}

fn request_body(request: &git_cognition_core::Request) -> Option<&str> {
    request.body().map(git_cognition_core::RequestBody::as_str)
}
