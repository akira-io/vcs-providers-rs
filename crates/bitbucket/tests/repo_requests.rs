use vcs_provider_bitbucket::bitbucket;
use vcs_provider_core::{RequestMethod, Visibility};

#[test]
fn bitbucket_repo_get_targets_repository_endpoint() {
    let repo = bitbucket()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();

    assert_eq!(
        repo.url().value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs"
    );
}

#[test]
fn bitbucket_repo_branch_list_targets_repository_endpoint() {
    let repo = bitbucket()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let page = bitbucket()
        .pagination()
        .request()
        .limit(50)
        .cursor("2")
        .build();

    assert_eq!(
        repo.branches(Some(&page)).value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/refs/branches?pagelen=50&page=2"
    );
}

#[test]
fn bitbucket_repo_commit_list_targets_repository_endpoint() {
    let repo = bitbucket()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();

    assert_eq!(
        repo.commits(None).value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/commits"
    );
}

#[test]
fn bitbucket_repo_list_targets_collection_endpoint() {
    let page = bitbucket().pagination().request().limit(25).build();
    let repo = bitbucket().repo();
    let collection = repo.collection();
    let list_query = repo.query().pagination(page.clone()).list();

    assert_eq!(
        collection.list(&list_query).value(),
        "https://api.bitbucket.org/2.0/repositories?pagelen=25"
    );
}

#[test]
fn bitbucket_repo_search_targets_collection_endpoint() {
    let page = bitbucket().pagination().request().limit(25).build();
    let repo = bitbucket().repo();
    let collection = repo.collection();
    let search_query = repo
        .query()
        .search("vcs provider")
        .pagination(page)
        .search();

    assert_eq!(
        collection.search(&search_query).value(),
        "https://api.bitbucket.org/2.0/repositories?q=name~%22vcs%20provider%22&pagelen=25"
    );
}

#[test]
fn bitbucket_repo_create_builds_put_request() {
    let repo = bitbucket()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let create_request = bitbucket()
        .repo()
        .draft(repo.clone())
        .visibility(Visibility::Private)
        .description("Universal provider layer")
        .create();

    assert_eq!(create_request.method(), &RequestMethod::Put);
    assert_eq!(
        request_body(&create_request),
        Some(r#"{"scm":"git","is_private":true,"description":"Universal provider layer"}"#)
    );
}

#[test]
fn bitbucket_repo_update_builds_put_request() {
    let repo = bitbucket()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();
    let update_request = repo
        .visibility(Visibility::Public)
        .description("Stable universal provider layer")
        .update();

    assert_eq!(update_request.method(), &RequestMethod::Put);
    assert_eq!(
        request_body(&update_request),
        Some(r#"{"is_private":false,"description":"Stable universal provider layer"}"#)
    );
}

#[test]
fn bitbucket_repo_delete_builds_delete_request() {
    let repo = bitbucket()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();

    assert_eq!(repo.delete().method(), &RequestMethod::Delete);
}

fn request_body(request: &vcs_provider_core::Request) -> Option<&str> {
    request.body().map(vcs_provider_core::RequestBody::as_str)
}
