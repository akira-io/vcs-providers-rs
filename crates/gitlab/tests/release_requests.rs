use vcs_provider_core::RequestMethod;
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
    assert_eq!(
        create_request.body().map(|body| body.as_str()),
        Some(r#"{"tag_name":"v1.0.0","name":"v1.0.0","description":"Release notes"}"#)
    );
}

#[test]
fn gitlab_release_update_builds_put_request() {
    let repo = gitlab()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let update_request = gitlab()
        .release()
        .repo(repo)
        .id("v1.0.0")
        .body("Updated")
        .update();

    assert_eq!(update_request.method(), &RequestMethod::Put);
    assert_eq!(
        update_request.body().map(|body| body.as_str()),
        Some(r#"{"description":"Updated"}"#)
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
