use git_cognition_core::RequestMethod;
use git_cognition_github::github;

#[test]
fn github_release_get_targets_repository_endpoint() {
    let release = github()
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .release("123")
        .get();

    assert_eq!(
        release.url().value(),
        "https://api.github.com/repos/akira-io/git-cognition-rs/releases/123"
    );
}

#[test]
fn github_release_list_targets_repository_endpoint() {
    let releases = github()
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .releases()
        .pagination()
        .limit(50)
        .cursor("2")
        .list();

    assert_eq!(
        releases.url().value(),
        "https://api.github.com/repos/akira-io/git-cognition-rs/releases?per_page=50&page=2"
    );
}

#[test]
fn github_release_builder_accepts_existing_repo() {
    let repo = github()
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .get();
    let release = github().release().repo(repo).id("123").get();

    assert_eq!(
        release.url().value(),
        "https://api.github.com/repos/akira-io/git-cognition-rs/releases/123"
    );
}

#[test]
fn github_release_create_builds_post_request() {
    let repo = github()
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
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
    assert_eq!(
        create_request.body().map(|body| body.as_str()),
        Some(r#"{"tag_name":"v1.0.0","name":"v1.0.0","body":"Release notes"}"#)
    );
}

#[test]
fn github_release_update_builds_patch_request() {
    let repo = github()
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .get();
    let update_request = github()
        .release()
        .repo(repo)
        .id("123")
        .body("Updated")
        .update();

    assert_eq!(update_request.method(), &RequestMethod::Patch);
    assert_eq!(
        update_request.body().map(|body| body.as_str()),
        Some(r#"{"body":"Updated"}"#)
    );
}

#[test]
fn github_release_delete_builds_delete_request() {
    let repo = github()
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .get();
    let release_resource = github().release().repo(repo).id("123").get();

    assert_eq!(release_resource.delete().method(), &RequestMethod::Delete);
}
