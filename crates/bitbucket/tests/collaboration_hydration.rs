use vcs_provider_bitbucket::bitbucket;
use vcs_provider_core::{CodeReviewsFluent, Repo, repo, run_async_test};

#[test]
fn bitbucket_client_hydrates_code_review_get_and_list() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let code_review_resource = bitbucket()
            .code_reviews()
            .response_body(r#"{"id":42}"#)
            .location(repository_location())
            .id("42")
            .get()
            .await?;
        let code_review_page = bitbucket()
            .code_reviews()
            .response_body(r#"{"values":[{"id":42}]}"#)
            .location(repository_location())
            .list()
            .await?;

        assert_eq!(code_review_resource.id().as_str(), "42");
        assert_eq!(code_review_page.items()[0].id().as_str(), "42");

        Ok(())
    })
}

#[test]
fn bitbucket_client_hydrates_code_review_mutations() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let created_code_review = bitbucket()
            .code_reviews()
            .response_body(r#"{"id":42}"#)
            .location(repository_location())
            .title("Add repository hydration")
            .source("feature")
            .target("main")
            .body("Details")
            .create()
            .await?;
        let updated_code_review = bitbucket()
            .code_reviews()
            .response_body(r#"{"id":42}"#)
            .location(repository_location())
            .id("42")
            .title("Add collaboration hydration")
            .update()
            .await?;
        let closed_code_review = bitbucket()
            .code_reviews()
            .response_body(r#"{"id":42}"#)
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

fn repository_location() -> Repo {
    repo().owner("akira-io").name("vcs-providers-rs").get()
}
