use vcs_provider_core::{IssuePatchBuilder, RequestMethod, issue};
use vcs_provider_github::github;

#[test]
fn github_issue_urls_target_repository_endpoints() {
    let issue_resource = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .issue("42")
        .build();
    let issues = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .issues()
        .pagination()
        .limit(50)
        .cursor("2")
        .get();

    assert_eq!(
        issue_resource.url().value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/issues/42"
    );
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
        .build();
    let issue_resource = github().issue().repo(repo).id("42").build();

    assert_eq!(
        issue_resource.url().value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/issues/42"
    );
}

#[test]
fn github_issue_requests_build_mutation_requests() {
    let repo = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .build();
    let draft = issue()
        .draft()
        .repo(repo.clone())
        .title("Track mutable issue requests")
        .body("Details")
        .build();
    let issue_resource = github().issue().repo(repo).id("42").build();
    let patch = IssuePatchBuilder::make(issue_resource.issue().clone())
        .closed()
        .build();
    let collection = github().issue().collection();

    assert_eq!(collection.create(&draft).method(), &RequestMethod::Post);
    assert!(collection.create(&draft).body().is_some());
    assert_eq!(
        issue_resource.update(&patch).method(),
        &RequestMethod::Patch
    );
}
