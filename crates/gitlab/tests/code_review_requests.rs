use vcs_provider_core::RequestMethod;
use vcs_provider_gitlab::gitlab;

#[test]
fn gitlab_code_review_get_targets_repository_endpoint() {
    assert_eq!(
        code_review_resource().url().value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fvcs-providers-rs/merge_requests/42"
    );
}

#[test]
fn gitlab_code_review_list_targets_repository_endpoint() {
    let code_reviews = gitlab()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .code_reviews()
        .pagination()
        .limit(50)
        .cursor("2")
        .list();

    assert_eq!(
        code_reviews.url().value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fvcs-providers-rs/merge_requests?per_page=50&page=2"
    );
}

#[test]
fn gitlab_code_review_builder_accepts_existing_repo() {
    assert_eq!(
        gitlab()
            .code_review()
            .repo(repository())
            .id("42")
            .get()
            .url()
            .value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fvcs-providers-rs/merge_requests/42"
    );
}

#[test]
fn gitlab_code_review_create_builds_post_request() {
    let create_request = gitlab()
        .code_review()
        .draft()
        .repo(repository())
        .title("Add mutable operations")
        .source("feature")
        .target("main")
        .body("Details")
        .create();

    assert_eq!(create_request.method(), &RequestMethod::Post);
    assert_eq!(
        request_body(&create_request),
        Some(
            r#"{"title":"Add mutable operations","source_branch":"feature","target_branch":"main","description":"Details"}"#
        )
    );
}

#[test]
fn gitlab_code_review_update_builds_put_request() {
    let update_request = gitlab()
        .code_review()
        .repo(repository())
        .id("42")
        .title("Add safe mutable operations")
        .body("Updated details")
        .update();

    assert_eq!(update_request.method(), &RequestMethod::Put);
    assert_eq!(
        request_body(&update_request),
        Some(r#"{"title":"Add safe mutable operations","description":"Updated details"}"#)
    );
}

#[test]
fn gitlab_code_review_merge_builds_put_request() {
    let merge_request = code_review_resource().merge();

    assert_eq!(merge_request.method(), &RequestMethod::Put);
    assert_eq!(
        merge_request.url().as_str(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fvcs-providers-rs/merge_requests/42/merge"
    );
}

#[test]
fn gitlab_code_review_close_builds_put_request() {
    let close_request = code_review_resource().close();

    assert_eq!(close_request.method(), &RequestMethod::Put);
    assert_eq!(
        request_body(&close_request),
        Some(r#"{"state_event":"close"}"#)
    );
}

#[test]
fn gitlab_code_review_delete_builds_delete_request() -> vcs_provider_core::VcsResult<()> {
    let delete_request = code_review_resource().delete()?;

    assert_eq!(delete_request.method(), &RequestMethod::Delete);

    Ok(())
}

fn repository() -> vcs_provider_core::ManagedRepo<vcs_provider_gitlab::GitLabProvider> {
    gitlab()
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get()
}

fn code_review_resource()
-> vcs_provider_core::ManagedCodeReview<vcs_provider_gitlab::GitLabProvider> {
    gitlab().code_review().repo(repository()).id("42").get()
}

fn request_body(request: &vcs_provider_core::Request) -> Option<&str> {
    request.body().map(vcs_provider_core::RequestBody::as_str)
}
