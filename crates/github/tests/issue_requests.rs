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
        .build();

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
