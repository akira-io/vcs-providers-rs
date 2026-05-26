use vcs_provider_core::{RequestMethod, vcs};
use vcs_provider_gitlab::{GitLabProvider, gitlab};

#[test]
fn gitlab_facade_builds_repo_requests() {
    let repository = vcs(gitlab())
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();

    assert_eq!(
        repository.url().value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fvcs-providers-rs"
    );
}

#[test]
fn gitlab_facade_builds_issue_requests() {
    let issue = vcs(gitlab())
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .issue("42")
        .get();

    assert_eq!(
        issue.url().value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fvcs-providers-rs/issues/42"
    );
}

#[test]
fn gitlab_facade_builds_code_review_requests() {
    let code_review = vcs(gitlab())
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .code_review("42")
        .get();

    assert_eq!(
        code_review.url().value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fvcs-providers-rs/merge_requests/42"
    );
}

#[test]
fn gitlab_facade_builds_release_requests() {
    let release = vcs(gitlab())
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .release("v1.0.0")
        .get();

    assert_eq!(
        release.url().value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fvcs-providers-rs/releases/v1.0.0"
    );
}

#[test]
fn gitlab_facade_builds_mutation_requests() {
    let create_request = vcs(gitlab())
        .repo()
        .draft(repository())
        .visibility(vcs_provider_core::Visibility::Private)
        .create();

    assert_eq!(create_request.method(), &RequestMethod::Post);
}

fn repository() -> vcs_provider_core::ManagedRepo<GitLabProvider> {
    vcs(gitlab())
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get()
}
