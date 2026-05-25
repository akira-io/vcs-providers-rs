use vcs_provider_core::{ReleasePatchBuilder, RequestMethod, release};
use vcs_provider_gitlab::gitlab;

#[test]
fn gitlab_release_urls_target_repository_endpoints() {
    let release = gitlab()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .release("v1.0.0")
        .build();
    let releases = gitlab()
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
        "https://gitlab.com/api/v4/projects/akira-io%2Fvcs-providers-rs/releases/v1.0.0"
    );
    assert_eq!(
        releases.url().value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fvcs-providers-rs/releases?per_page=50&page=2"
    );
}

#[test]
fn gitlab_release_builder_accepts_existing_repo() {
    let repo = gitlab()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .build();
    let release = gitlab().release().repo(repo).id("v1.0.0").build();

    assert_eq!(
        release.url().value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fvcs-providers-rs/releases/v1.0.0"
    );
}

#[test]
fn gitlab_release_requests_build_mutation_requests() {
    let repo = gitlab()
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
    let release_resource = gitlab().release().repo(repo).id("v1.0.0").build();
    let patch = ReleasePatchBuilder::make(release_resource.release().clone())
        .body("Updated")
        .build();
    let collection = gitlab().release().collection();

    assert_eq!(collection.create(&draft).method(), &RequestMethod::Post);
    assert!(collection.create(&draft).body().is_some());
    assert_eq!(
        release_resource.update(&patch).method(),
        &RequestMethod::Put
    );
    assert_eq!(release_resource.delete().method(), &RequestMethod::Delete);
}
