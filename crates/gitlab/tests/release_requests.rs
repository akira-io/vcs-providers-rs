use vcs_provider_core::{ReleasePatchBuilder, RequestMethod};
use vcs_provider_gitlab::gitlab;

#[test]
fn gitlab_release_get_targets_repository_endpoint() {
    let release = gitlab()
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
fn gitlab_release_list_targets_repository_endpoint() {
    let releases = gitlab()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .releases()
        .pagination()
        .limit(50)
        .cursor("2")
        .list();

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
        .get();
    let release = gitlab().release().repo(repo).id("v1.0.0").get();

    assert_eq!(
        release.url().value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fvcs-providers-rs/releases/v1.0.0"
    );
}

#[test]
fn gitlab_release_create_builds_post_request() {
    let repo = gitlab()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let create_request = gitlab()
        .release()
        .draft()
        .repo(repo.clone())
        .tag("v1.0.0")
        .name("v1.0.0")
        .body("Release notes")
        .create();

    assert_eq!(create_request.method(), &RequestMethod::Post);
    assert!(create_request.body().is_some());
}

#[test]
fn gitlab_release_update_builds_put_request() {
    let repo = gitlab()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let release_resource = gitlab().release().repo(repo).id("v1.0.0").get();
    let release_patch = ReleasePatchBuilder::make(release_resource.release().clone())
        .body("Updated")
        .get();

    assert_eq!(
        release_resource.update(&release_patch).method(),
        &RequestMethod::Put
    );
}

#[test]
fn gitlab_release_delete_builds_delete_request() {
    let repo = gitlab()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let release_resource = gitlab().release().repo(repo).id("v1.0.0").get();

    assert_eq!(release_resource.delete().method(), &RequestMethod::Delete);
}
