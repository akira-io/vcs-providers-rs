use vcs_provider_core::{RequestMethod, vcs};
use vcs_provider_github::{GitHubProvider, github};

#[test]
fn github_facade_builds_repo_requests() {
    let repository = vcs(github())
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();

    assert_eq!(
        repository.url().value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs"
    );
}

#[test]
fn github_facade_builds_issue_requests() {
    let issue = vcs(github())
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .issue("42")
        .get();

    assert_eq!(
        issue.url().value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/issues/42"
    );
}

#[test]
fn github_facade_builds_code_review_requests() {
    let code_review = vcs(github())
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .code_review("42")
        .get();

    assert_eq!(
        code_review.url().value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/pulls/42"
    );
}

#[test]
fn github_facade_builds_release_requests() {
    let release = vcs(github())
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .release("123")
        .get();

    assert_eq!(
        release.url().value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/releases/123"
    );
}

#[test]
fn github_facade_builds_mutation_requests() {
    let create_request = vcs(github())
        .repo()
        .draft(repository())
        .visibility(vcs_provider_core::Visibility::Private)
        .create();

    assert_eq!(create_request.method(), &RequestMethod::Post);
}

fn repository() -> vcs_provider_core::ManagedRepo<GitHubProvider> {
    vcs(github())
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get()
}
