use vcs_provider_core::{CodeReviewPatchBuilder, RequestMethod};
use vcs_provider_github::github;

#[test]
fn github_code_review_get_targets_repository_endpoint() {
    assert_eq!(
        code_review_resource().url().value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/pulls/42"
    );
}

#[test]
fn github_code_review_list_targets_repository_endpoint() {
    let code_reviews = github()
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
        "https://api.github.com/repos/akira-io/vcs-providers-rs/pulls?per_page=50&page=2"
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
        "https://api.github.com/repos/akira-io/vcs-providers-rs/pulls/42"
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
    assert!(create_request.body().is_some());
}

#[test]
fn github_code_review_update_builds_patch_request() {
    assert_eq!(
        code_review_resource().update(&code_review_patch()).method(),
        &RequestMethod::Patch
    );
}

#[test]
fn github_code_review_close_builds_patch_request() {
    assert_eq!(
        code_review_resource().close().method(),
        &RequestMethod::Patch
    );
}

fn repository() -> vcs_provider_core::ManagedRepo<vcs_provider_github::GitHubProvider> {
    github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get()
}

fn code_review_resource()
-> vcs_provider_core::ManagedCodeReview<vcs_provider_github::GitHubProvider> {
    github().code_review().repo(repository()).id("42").get()
}

fn code_review_patch() -> vcs_provider_core::CodeReviewPatch {
    CodeReviewPatchBuilder::make(code_review_resource().code_review().clone())
        .closed()
        .get()
}
