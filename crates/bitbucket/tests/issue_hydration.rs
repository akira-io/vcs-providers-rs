use vcs_provider_bitbucket::bitbucket;
use vcs_provider_core::{IssuesFluent, Repo, repo, run_async_test};

#[test]
fn bitbucket_client_hydrates_issue_get_and_list() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let issue_resource = bitbucket()
            .body(r#"{"id":42}"#)
            .issues()
            .location(repository_location())
            .id("42")
            .get()
            .await?;
        let issue_page = bitbucket()
            .body(r#"{"values":[{"id":42}]}"#)
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
fn bitbucket_client_hydrates_issue_mutations() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let created_issue = bitbucket()
            .body(r#"{"id":42}"#)
            .issues()
            .location(repository_location())
            .title("Track mutable issue requests")
            .body("Details")
            .create()
            .await?;
        let updated_issue = bitbucket()
            .body(r#"{"id":42}"#)
            .issues()
            .location(repository_location())
            .id("42")
            .title("Track mutable issue requests safely")
            .update()
            .await?;
        let closed_issue = bitbucket()
            .body(r#"{"id":42}"#)
            .issues()
            .location(repository_location())
            .id("42")
            .close()
            .await?;

        bitbucket()
            .status(204)
            .issues()
            .location(repository_location())
            .id("42")
            .delete()
            .await?;

        assert_eq!(created_issue.id().as_str(), "42");
        assert_eq!(updated_issue.id().as_str(), "42");
        assert_eq!(closed_issue.id().as_str(), "42");

        Ok(())
    })
}

fn repository_location() -> Repo {
    repo().owner("akira-io").name("vcs-providers-rs").get()
}
