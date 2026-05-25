use vcs_provider_core::{ReleasePatchBuilder, RequestMethod};
use vcs_provider_github::github;

#[test]
fn github_release_get_targets_repository_endpoint() {
    let release = github()
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
fn github_release_list_targets_repository_endpoint() {
    let releases = github()
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
        "https://api.github.com/repos/akira-io/vcs-providers-rs/releases?per_page=50&page=2"
    );
}

#[test]
fn github_release_builder_accepts_existing_repo() {
    let repo = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let release = github().release().repo(repo).id("123").get();

    assert_eq!(
        release.url().value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/releases/123"
    );
}

#[test]
fn github_release_create_builds_post_request() {
    let repo = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let create_request = github()
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
fn github_release_update_builds_patch_request() {
    let repo = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let release_resource = github().release().repo(repo).id("123").get();
    let release_patch = ReleasePatchBuilder::make(release_resource.release().clone())
        .body("Updated")
        .get();

    assert_eq!(
        release_resource.update(&release_patch).method(),
        &RequestMethod::Patch
    );
}

#[test]
fn github_release_delete_builds_delete_request() {
    let repo = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let release_resource = github().release().repo(repo).id("123").get();

    assert_eq!(release_resource.delete().method(), &RequestMethod::Delete);
}
