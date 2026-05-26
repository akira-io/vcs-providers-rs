use vcs_provider_core::{
    CodeReviewsFluent, IssuesFluent, ReleasesFluent, Repo, repo, run_async_test,
};
use vcs_provider_gitlab::gitlab;

#[test]
fn gitlab_client_hydrates_issue_get_and_list() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let issue_resource = gitlab()
            .issues()
            .response_body(r#"{"iid":42}"#)
            .location(repository_location())
            .id("42")
            .get()
            .await?;
        let issue_page = gitlab()
            .issues()
            .response_body(r#"[{"iid":42}]"#)
            .location(repository_location())
            .list()
            .await?;

        assert_eq!(issue_resource.id().as_str(), "42");
        assert_eq!(issue_page.items()[0].id().as_str(), "42");

        Ok(())
    })
}

#[test]
fn gitlab_client_hydrates_issue_mutations() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let created_issue = gitlab()
            .issues()
            .response_body(r#"{"iid":42}"#)
            .location(repository_location())
            .title("Fix payment state")
            .body("Details")
            .create()
            .await?;
        let updated_issue = gitlab()
            .issues()
            .response_body(r#"{"iid":42}"#)
            .location(repository_location())
            .id("42")
            .title("Fix payment state safely")
            .update()
            .await?;
        let closed_issue = gitlab()
            .issues()
            .response_body(r#"{"iid":42}"#)
            .location(repository_location())
            .id("42")
            .close()
            .await?;

        assert_eq!(created_issue.id().as_str(), "42");
        assert_eq!(updated_issue.id().as_str(), "42");
        assert_eq!(closed_issue.id().as_str(), "42");

        Ok(())
    })
}

#[test]
fn gitlab_client_hydrates_code_review_get_and_list() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let code_review_resource = gitlab()
            .code_reviews()
            .response_body(r#"{"iid":42}"#)
            .location(repository_location())
            .id("42")
            .get()
            .await?;
        let code_review_page = gitlab()
            .code_reviews()
            .response_body(r#"[{"iid":42}]"#)
            .location(repository_location())
            .list()
            .await?;

        assert_eq!(code_review_resource.id().as_str(), "42");
        assert_eq!(code_review_page.items()[0].id().as_str(), "42");

        Ok(())
    })
}

#[test]
fn gitlab_client_hydrates_code_review_mutations() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let created_code_review = gitlab()
            .code_reviews()
            .response_body(r#"{"iid":42}"#)
            .location(repository_location())
            .title("Add repository hydration")
            .source("feature")
            .target("main")
            .body("Details")
            .create()
            .await?;
        let updated_code_review = gitlab()
            .code_reviews()
            .response_body(r#"{"iid":42}"#)
            .location(repository_location())
            .id("42")
            .title("Add collaboration hydration")
            .update()
            .await?;
        let closed_code_review = gitlab()
            .code_reviews()
            .response_body(r#"{"iid":42}"#)
            .location(repository_location())
            .id("42")
            .close()
            .await?;

        assert_eq!(created_code_review.id().as_str(), "42");
        assert_eq!(updated_code_review.id().as_str(), "42");
        assert_eq!(closed_code_review.id().as_str(), "42");

        Ok(())
    })
}

#[test]
fn gitlab_client_hydrates_release_get_and_list() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let release_resource = gitlab()
            .releases()
            .response_body(r#"{"tag_name":"v1.0.0"}"#)
            .location(repository_location())
            .id("v1.0.0")
            .get()
            .await?;
        let release_page = gitlab()
            .releases()
            .response_body(r#"[{"tag_name":"v1.0.0"}]"#)
            .location(repository_location())
            .list()
            .await?;

        assert_eq!(release_resource.id().as_str(), "v1.0.0");
        assert_eq!(release_page.items()[0].id().as_str(), "v1.0.0");

        Ok(())
    })
}

#[test]
fn gitlab_client_hydrates_release_mutations() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let created_release = gitlab()
            .releases()
            .response_body(r#"{"tag_name":"v1.0.0"}"#)
            .location(repository_location())
            .tag("v1.0.0")
            .name("v1.0.0")
            .body("Release notes")
            .create()
            .await?;
        let updated_release = gitlab()
            .releases()
            .response_body(r#"{"tag_name":"v1.0.0"}"#)
            .location(repository_location())
            .id("v1.0.0")
            .body("Updated notes")
            .update()
            .await?;

        gitlab()
            .releases()
            .response_body("{}")
            .location(repository_location())
            .id("v1.0.0")
            .delete()
            .await?;

        assert_eq!(created_release.id().as_str(), "v1.0.0");
        assert_eq!(updated_release.id().as_str(), "v1.0.0");

        Ok(())
    })
}

fn repository_location() -> Repo {
    repo().owner("akira-io").name("vcs-providers-rs").get()
}
