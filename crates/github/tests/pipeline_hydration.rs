use git_cognition_core::{PipelinesFluent, Repo, repo, run_async_test};
use git_cognition_github::github;

#[test]
fn github_client_hydrates_pipeline_get_and_list() -> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        let pipeline_resource = github()
            .body(r#"{"id":42}"#)
            .pipelines()
            .location(repository_location())
            .id("42")
            .get()
            .await?;
        let pipeline_page = github()
            .body(r#"{"workflow_runs":[{"id":42}]}"#)
            .pipelines()
            .location(repository_location())
            .list()
            .await?;

        assert_eq!(pipeline_resource.id().as_str(), "42");
        assert_eq!(pipeline_page.items()[0].id().as_str(), "42");

        Ok(())
    })
}

#[test]
fn github_client_hydrates_pipeline_commands() -> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        let rerun_pipeline = github()
            .body(r#"{"id":42}"#)
            .pipelines()
            .location(repository_location())
            .id("42")
            .rerun()
            .await?;
        let canceled_pipeline = github()
            .body(r#"{"id":42}"#)
            .pipelines()
            .location(repository_location())
            .id("42")
            .cancel()
            .await?;

        assert_eq!(rerun_pipeline.id().as_str(), "42");
        assert_eq!(canceled_pipeline.id().as_str(), "42");

        Ok(())
    })
}

fn repository_location() -> Repo {
    repo().owner("akira-io").name("git-cognition-rs").get()
}
