use vcs_provider_core::{CodeReviewPatchBuilder, RequestMethod, code_review};
use vcs_provider_gitlab::gitlab;

#[test]
fn gitlab_code_review_urls_target_repository_endpoints() {
    let code_review = gitlab()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .code_review("42")
        .build();
    let code_reviews = gitlab()
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
        "https://gitlab.com/api/v4/projects/akira-io%2Fvcs-providers-rs/merge_requests/42"
    );
    assert_eq!(
        code_reviews.url().value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fvcs-providers-rs/merge_requests?per_page=50&page=2"
    );
}

#[test]
fn gitlab_code_review_builder_accepts_existing_repo() {
    let repo = gitlab()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .build();
    let code_review = gitlab().code_review().repo(repo).id("42").build();

    assert_eq!(
        code_review.url().value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fvcs-providers-rs/merge_requests/42"
    );
}

#[test]
fn gitlab_code_review_requests_build_mutation_requests() {
    let repo = gitlab()
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
    let code_review_resource = gitlab().code_review().repo(repo).id("42").build();
    let patch = CodeReviewPatchBuilder::make(code_review_resource.code_review().clone())
        .closed()
        .build();
    let collection = gitlab().code_review().collection();

    assert_eq!(collection.create(&draft).method(), &RequestMethod::Post);
    assert!(collection.create(&draft).body().is_some());
    assert_eq!(
        code_review_resource.update(&patch).method(),
        &RequestMethod::Put
    );
    assert_eq!(
        code_review_resource.delete().method(),
        &RequestMethod::Delete
    );
}
