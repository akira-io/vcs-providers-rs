use git_cognition_core::{RequestMethod, Visibility};
use git_cognition_gitlab::gitlab;

#[test]
fn gitlab_repo_get_targets_repository_endpoint() {
    let repo = gitlab()
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .get();

    assert_eq!(
        repo.url().value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fgit-cognition-rs"
    );
}

#[test]
fn gitlab_repo_branch_list_targets_repository_endpoint() {
    let repo = gitlab()
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .get();
    let page = gitlab()
        .pagination()
        .request()
        .limit(50)
        .cursor("2")
        .build();

    assert_eq!(
        repo.branches(Some(&page)).value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fgit-cognition-rs/repository/branches?per_page=50&page=2"
    );
}

#[test]
fn gitlab_repo_branch_create_builds_post_request() -> git_cognition_core::CognitionResult<()> {
    let repo = gitlab()
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .get();
    let request = repo.branch().name("feature").sha("abc123").create()?;

    assert_eq!(request.method(), &RequestMethod::Post);
    assert_eq!(
        request.url().value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fgit-cognition-rs/repository/branches"
    );
    assert_eq!(
        request_body(&request),
        Some(r#"{"branch":"feature","ref":"abc123"}"#)
    );

    Ok(())
}

#[test]
fn gitlab_repo_branch_delete_builds_delete_request() -> git_cognition_core::CognitionResult<()> {
    let repo = gitlab()
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .get();
    let request = repo.branch().name("feature").delete()?;

    assert_eq!(request.method(), &RequestMethod::Delete);
    assert_eq!(
        request.url().value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fgit-cognition-rs/repository/branches/feature"
    );

    Ok(())
}

#[test]
fn gitlab_repo_commit_list_targets_repository_endpoint() {
    let repo = gitlab()
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .get();

    assert_eq!(
        repo.commits(None).value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fgit-cognition-rs/repository/commits"
    );
}

#[test]
fn gitlab_repo_list_targets_collection_endpoint() {
    let page = gitlab().pagination().request().limit(25).build();
    let repo = gitlab().repo();
    let collection = repo.collection();
    let list_query = repo.query().pagination(page.clone()).list();

    assert_eq!(
        collection.list(&list_query).value(),
        "https://gitlab.com/api/v4/projects?per_page=25"
    );
}

#[test]
fn gitlab_repo_search_targets_collection_endpoint() {
    let page = gitlab().pagination().request().limit(25).build();
    let repo = gitlab().repo();
    let collection = repo.collection();
    let search_query = repo
        .query()
        .search("cognition provider")
        .pagination(page)
        .search();

    assert_eq!(
        collection.search(&search_query).value(),
        "https://gitlab.com/api/v4/projects?search=cognition%20provider&per_page=25"
    );
}

#[test]
fn gitlab_repo_create_builds_post_request() {
    let repo = gitlab()
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .get();
    let create_request = gitlab()
        .repo()
        .draft(repo.clone())
        .visibility(Visibility::Private)
        .description("Universal provider layer")
        .create();

    assert_eq!(create_request.method(), &RequestMethod::Post);
    assert_eq!(
        request_body(&create_request),
        Some(
            r#"{"name":"git-cognition-rs","path":"git-cognition-rs","namespace_path":"akira-io","visibility":"private","description":"Universal provider layer"}"#
        )
    );
}

#[test]
fn gitlab_repo_update_builds_put_request() {
    let repo = gitlab()
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .get();
    let update_request = repo
        .visibility(Visibility::Public)
        .description("Stable universal provider layer")
        .update();

    assert_eq!(update_request.method(), &RequestMethod::Put);
    assert_eq!(
        request_body(&update_request),
        Some(r#"{"visibility":"public","description":"Stable universal provider layer"}"#)
    );
}

#[test]
fn gitlab_repo_delete_builds_delete_request() {
    let repo = gitlab()
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .get();

    assert_eq!(repo.delete().method(), &RequestMethod::Delete);
}

fn request_body(request: &git_cognition_core::Request) -> Option<&str> {
    request.body().map(git_cognition_core::RequestBody::as_str)
}
