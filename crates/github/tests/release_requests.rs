use vcs_provider_core::{ReleasePatchBuilder, RequestMethod, release};
use vcs_provider_github::github;

#[test]
fn github_release_urls_target_repository_endpoints() {
    let release = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .release("123")
        .build();
    let releases = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .releases()
        .pagination()
        .limit(50)
        .cursor("2")
        .get();

    assert_eq!(
        release.url().value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/releases/123"
    );
    assert_eq!(
        releases.url().value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/releases?per_page=50&page=2"
    );
}

#[test]
fn github_release_builder_accepts_existing_repo() {
    let repo = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .build();
    let release = github().release().repo(repo).id("123").build();

    assert_eq!(
        release.url().value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/releases/123"
    );
}

#[test]
fn github_release_requests_build_mutation_requests() {
    let repo = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .build();
    let draft = release()
        .draft()
        .repo(repo.clone())
        .tag("v1.0.0")
        .name("v1.0.0")
        .body("Release notes")
        .build();
    let release_resource = github().release().repo(repo).id("123").build();
    let patch = ReleasePatchBuilder::make(release_resource.release().clone())
        .body("Updated")
        .build();
    let collection = github().release().collection();

    assert_eq!(collection.create(&draft).method(), &RequestMethod::Post);
    assert!(collection.create(&draft).body().is_some());
    assert_eq!(
        release_resource.update(&patch).method(),
        &RequestMethod::Patch
    );
    assert_eq!(release_resource.delete().method(), &RequestMethod::Delete);
}
