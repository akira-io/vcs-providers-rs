use vcs_provider_bitbucket::bitbucket;
use vcs_provider_core::{PipelinesFluent, Repo, VcsError, repo, run_async_test};

#[test]
fn bitbucket_client_hydrates_pipeline_get_and_list() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let pipeline_resource = bitbucket()
            .body(r#"{"uuid":"{pipeline-uuid}"}"#)
            .pipelines()
            .location(repository_location())
            .id("{pipeline-uuid}")
            .get()
            .await?;
        let pipeline_page = bitbucket()
            .body(r#"{"values":[{"uuid":"{pipeline-uuid}"}]}"#)
            .pipelines()
            .location(repository_location())
            .list()
            .await?;

        assert_eq!(pipeline_resource.id().as_str(), "{pipeline-uuid}");
        assert_eq!(pipeline_page.items()[0].id().as_str(), "{pipeline-uuid}");

        Ok(())
    })
}

#[test]
fn bitbucket_client_hydrates_pipeline_cancel() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let canceled_pipeline = bitbucket()
            .body(r#"{"uuid":"{pipeline-uuid}"}"#)
            .pipelines()
            .location(repository_location())
            .id("{pipeline-uuid}")
            .cancel()
            .await?;

        assert_eq!(canceled_pipeline.id().as_str(), "{pipeline-uuid}");

        Ok(())
    })
}

#[test]
fn bitbucket_client_reports_unvalidated_pipeline_rerun() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let result = bitbucket()
            .body(r#"{"uuid":"{pipeline-uuid}"}"#)
            .pipelines()
            .location(repository_location())
            .id("{pipeline-uuid}")
            .rerun()
            .await;

        assert_eq!(
            result,
            Err(VcsError::InvalidInput(
                "bitbucket pipeline rerun is not exposed by a validated pipeline endpoint".into()
            ))
        );

        Ok(())
    })
}

fn repository_location() -> Repo {
    repo().owner("akira-io").name("vcs-providers-rs").get()
}
