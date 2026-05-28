use git_cognition_core::{
    CodeReviewsFluent, CognitionError, IssuesFluent, ReleasesFluent, Repo, repo, run_async_test,
};
use git_cognition_github::github;

#[test]
fn github_client_hydrates_issue_get_and_list() -> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        let issue_resource = github()
            .body(r#"{"number":42}"#)
            .issues()
            .location(repository_location())
            .id("42")
            .get()
            .await?;
        let issue_page = github()
            .body(r#"[{"number":42}]"#)
            .issues()
            .location(repository_location())
            .list()
            .await?;

        assert_eq!(issue_resource.id().as_str(), "42");
        assert_eq!(issue_page.items()[0].id().as_str(), "42");

        Ok(())
    })
}

#[test]
fn github_client_hydrates_issue_mutations() -> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        let created_issue = github()
            .body(r#"{"number":42}"#)
            .issues()
            .location(repository_location())
            .title("Fix payment state")
            .body("Details")
            .create()
            .await?;
        let updated_issue = github()
            .body(r#"{"number":42}"#)
            .issues()
            .location(repository_location())
            .id("42")
            .title("Fix payment state safely")
            .update()
            .await?;
        let closed_issue = github()
            .body(r#"{"number":42}"#)
            .issues()
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
fn github_client_reports_unsupported_issue_delete() -> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        let result = github()
            .body("{}")
            .issues()
            .location(repository_location())
            .id("42")
            .delete()
            .await;

        assert_eq!(
            result,
            Err(CognitionError::UnsupportedOperation("issue delete".into()))
        );

        Ok(())
    })
}

#[test]
fn github_client_hydrates_code_review_get_and_list() -> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        let code_review_resource = github()
            .body(r#"{"number":42}"#)
            .code_reviews()
            .location(repository_location())
            .id("42")
            .get()
            .await?;
        let code_review_page = github()
            .body(r#"[{"number":42}]"#)
            .code_reviews()
            .location(repository_location())
            .list()
            .await?;

        assert_eq!(code_review_resource.id().as_str(), "42");
        assert_eq!(code_review_page.items()[0].id().as_str(), "42");

        Ok(())
    })
}

#[test]
fn github_client_hydrates_code_review_mutations() -> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        let created_code_review = github()
            .body(r#"{"number":42}"#)
            .code_reviews()
            .location(repository_location())
            .title("Add repository hydration")
            .source("feature")
            .target("main")
            .body("Details")
            .create()
            .await?;
        let updated_code_review = github()
            .body(r#"{"number":42}"#)
            .code_reviews()
            .location(repository_location())
            .id("42")
            .title("Add collaboration hydration")
            .update()
            .await?;
        let merged_code_review = github()
            .body(r#"{"number":42}"#)
            .code_reviews()
            .location(repository_location())
            .id("42")
            .merge()
            .await?;
        let closed_code_review = github()
            .body(r#"{"number":42}"#)
            .code_reviews()
            .location(repository_location())
            .id("42")
            .close()
            .await?;

        assert_eq!(created_code_review.id().as_str(), "42");
        assert_eq!(updated_code_review.id().as_str(), "42");
        assert_eq!(merged_code_review.id().as_str(), "42");
        assert_eq!(closed_code_review.id().as_str(), "42");

        Ok(())
    })
}

#[test]
fn github_client_reports_unsupported_code_review_delete() -> git_cognition_core::CognitionResult<()>
{
    run_async_test(async {
        let result = github()
            .body("{}")
            .code_reviews()
            .location(repository_location())
            .id("42")
            .delete()
            .await;

        assert_eq!(
            result,
            Err(CognitionError::UnsupportedOperation(
                "code review delete".into()
            ))
        );

        Ok(())
    })
}

#[test]
fn github_client_hydrates_release_get_and_list() -> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        let release_resource = github()
            .body(r#"{"id":123}"#)
            .releases()
            .location(repository_location())
            .id("123")
            .get()
            .await?;
        let release_page = github()
            .body(r#"[{"id":123}]"#)
            .releases()
            .location(repository_location())
            .list()
            .await?;

        assert_eq!(release_resource.id().as_str(), "123");
        assert_eq!(release_page.items()[0].id().as_str(), "123");

        Ok(())
    })
}

#[test]
fn github_client_hydrates_release_mutations() -> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        let created_release = github()
            .body(r#"{"id":123}"#)
            .releases()
            .location(repository_location())
            .tag("v1.0.0")
            .name("v1.0.0")
            .body("Release notes")
            .create()
            .await?;
        let updated_release = github()
            .body(r#"{"id":123}"#)
            .releases()
            .location(repository_location())
            .id("123")
            .body("Updated notes")
            .update()
            .await?;

        github()
            .body("{}")
            .releases()
            .location(repository_location())
            .id("123")
            .delete()
            .await?;

        assert_eq!(created_release.id().as_str(), "123");
        assert_eq!(updated_release.id().as_str(), "123");

        Ok(())
    })
}

fn repository_location() -> Repo {
    repo().owner("akira-io").name("git-cognition-rs").get()
}
