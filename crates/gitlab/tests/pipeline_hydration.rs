use vcs_provider_core::{PipelinesFluent, Repo, repo, run_async_test};
use vcs_provider_gitlab::gitlab;

#[test]
fn gitlab_client_hydrates_pipeline_get_and_list() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let pipeline_resource = gitlab()
            .pipelines()
            .response_body(r#"{"id":42}"#)
            .location(repository_location())
            .id("42")
            .get()
            .await?;
        let pipeline_page = gitlab()
            .pipelines()
            .response_body(r#"[{"id":42}]"#)
            .location(repository_location())
            .list()
            .await?;

        assert_eq!(pipeline_resource.id().as_str(), "42");
        assert_eq!(pipeline_page.items()[0].id().as_str(), "42");

        Ok(())
    })
}

#[test]
fn gitlab_client_hydrates_pipeline_commands() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let retried_pipeline = gitlab()
            .pipelines()
            .response_body(r#"{"id":42}"#)
            .location(repository_location())
            .id("42")
            .rerun()
            .await?;
        let canceled_pipeline = gitlab()
            .pipelines()
            .response_body(r#"{"id":42}"#)
            .location(repository_location())
            .id("42")
            .cancel()
            .await?;

        assert_eq!(retried_pipeline.id().as_str(), "42");
        assert_eq!(canceled_pipeline.id().as_str(), "42");

        Ok(())
    })
}

fn repository_location() -> Repo {
    repo().owner("akira-io").name("vcs-providers-rs").get()
}
