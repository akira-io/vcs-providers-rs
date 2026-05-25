use vcs_provider_core::{IssuePatchBuilder, RequestMethod, issue};
use vcs_provider_gitlab::gitlab;

#[test]
fn gitlab_issue_urls_target_repository_endpoints() {
    let issue_resource = gitlab()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .issue("42")
        .build();
    let issues = gitlab()
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
        "https://gitlab.com/api/v4/projects/akira-io%2Fvcs-providers-rs/issues/42"
    );
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
        .build();
    let issue_resource = gitlab().issue().repo(repo).id("42").build();

    assert_eq!(
        issue_resource.url().value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fvcs-providers-rs/issues/42"
    );
}

#[test]
fn gitlab_issue_requests_build_mutation_requests() {
    let repo = gitlab()
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
    let issue_resource = gitlab().issue().repo(repo).id("42").build();
    let patch = IssuePatchBuilder::make(issue_resource.issue().clone())
        .closed()
        .build();
    let collection = gitlab().issue().collection();

    assert_eq!(collection.create(&draft).method(), &RequestMethod::Post);
    assert!(collection.create(&draft).body().is_some());
    assert_eq!(issue_resource.update(&patch).method(), &RequestMethod::Put);
    assert_eq!(issue_resource.delete().method(), &RequestMethod::Delete);
}
