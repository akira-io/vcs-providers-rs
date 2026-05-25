use vcs_provider_core::{CodeReviewPatchBuilder, RequestMethod, code_review};
use vcs_provider_github::github;

#[test]
fn github_code_review_urls_target_repository_endpoints() {
    let code_review = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .code_review("42")
        .build();
    let code_reviews = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .code_reviews()
        .pagination()
        .limit(50)
        .cursor("2")
        .get();

    assert_eq!(
        code_review.url().value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/pulls/42"
    );
    assert_eq!(
        code_reviews.url().value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/pulls?per_page=50&page=2"
    );
}

#[test]
fn github_code_review_builder_accepts_existing_repo() {
    let repo = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .build();
    let code_review = github().code_review().repo(repo).id("42").build();

    assert_eq!(
        code_review.url().value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/pulls/42"
    );
}

#[test]
fn github_code_review_requests_build_mutation_requests() {
    let repo = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .build();
    let draft = code_review()
        .draft()
        .repo(repo.clone())
        .title("Add mutable operations")
        .source("feature")
        .target("main")
        .body("Details")
        .build();
    let code_review_resource = github().code_review().repo(repo).id("42").build();
    let patch = CodeReviewPatchBuilder::make(code_review_resource.code_review().clone())
        .closed()
        .build();
    let collection = github().code_review().collection();

    assert_eq!(collection.create(&draft).method(), &RequestMethod::Post);
    assert!(collection.create(&draft).body().is_some());
    assert_eq!(
        code_review_resource.update(&patch).method(),
        &RequestMethod::Patch
    );
    assert_eq!(
        code_review_resource.delete().method(),
        &RequestMethod::Patch
    );
}
