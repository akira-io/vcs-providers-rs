use vcs_provider_core::{RequestMethod, Visibility};
use vcs_provider_github::github;

#[test]
fn github_repo_get_targets_repository_endpoint() {
    let repo = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();

    assert_eq!(
        repo.url().value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs"
    );
}

#[test]
fn github_repo_branch_list_targets_repository_endpoint() {
    let repo = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let page = github()
        .pagination()
        .request()
        .limit(50)
        .cursor("2")
        .build();

    assert_eq!(
        repo.branches(Some(&page)).value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/branches?per_page=50&page=2"
    );
}

#[test]
fn github_repo_commit_list_targets_repository_endpoint() {
    let repo = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();

    assert_eq!(
        repo.commits(None).value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/commits"
    );
}

#[test]
fn github_repo_list_targets_collection_endpoint() {
    let page = github().pagination().request().limit(25).build();
    let repo = github().repo();
    let collection = repo.collection();
    let list_query = repo.query().pagination(page.clone()).list();

    assert_eq!(
        collection.list(&list_query).value(),
        "https://api.github.com/user/repos?per_page=25"
    );
}

#[test]
fn github_repo_search_targets_collection_endpoint() {
    let page = github().pagination().request().limit(25).build();
    let repo = github().repo();
    let collection = repo.collection();
    let search_query = repo
        .query()
        .search("vcs provider")
        .pagination(page)
        .search();

    assert_eq!(
        collection.search(&search_query).value(),
        "https://api.github.com/search/repositories?q=vcs%20provider&per_page=25"
    );
}

#[test]
fn github_repo_create_builds_post_request() {
    let repo = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let create_request = github()
        .repo()
        .draft(repo.clone())
        .visibility(Visibility::Private)
        .description("Universal provider layer")
        .create();

    assert_eq!(create_request.method(), &RequestMethod::Post);
    assert_eq!(
        request_body(&create_request),
        Some(
            r#"{"name":"vcs-providers-rs","private":true,"description":"Universal provider layer"}"#
        )
    );
}

#[test]
fn github_repo_update_builds_patch_request() {
    let repo = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let update_request = repo
        .visibility(Visibility::Public)
        .description("Stable universal provider layer")
        .update();

    assert_eq!(update_request.method(), &RequestMethod::Patch);
    assert_eq!(
        request_body(&update_request),
        Some(r#"{"private":false,"description":"Stable universal provider layer"}"#)
    );
}

#[test]
fn github_repo_delete_builds_delete_request() {
    let repo = github()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();

    assert_eq!(repo.delete().method(), &RequestMethod::Delete);
}

fn request_body(request: &vcs_provider_core::Request) -> Option<&str> {
    request.body().map(vcs_provider_core::RequestBody::as_str)
}
