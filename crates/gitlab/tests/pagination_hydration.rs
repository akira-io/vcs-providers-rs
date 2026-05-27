use vcs_provider_core::{
    CodeReviewsFluent, IssuesFluent, Page, PipelinesFluent, ReleasesFluent, Repo, repo,
    run_async_test,
};
use vcs_provider_gitlab::gitlab;

#[test]
fn gitlab_repo_lists_preserve_next_cursor() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let repositories = gitlab()
            .header("x-next-page", "3")
            .body(r#"[{"path_with_namespace":"akira-io/vcs-providers-rs","visibility":"public"}]"#)
            .repos()
            .list(gitlab().repo().query().optional_pagination(None).list())
            .await?;
        let branches = gitlab()
            .header("x-next-page", "3")
            .body(r#"[{"name":"main"}]"#)
            .repos()
            .branches(repository_location())
            .await?;

        assert_next(repositories, "3");
        assert_next(branches, "3");

        Ok(())
    })
}

#[test]
fn gitlab_collaboration_lists_preserve_next_cursor() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let issues = gitlab()
            .header("x-next-page", "3")
            .body(r#"[{"iid":42}]"#)
            .issues()
            .location(repository_location())
            .list()
            .await?;
        let code_reviews = gitlab()
            .header("x-next-page", "3")
            .body(r#"[{"iid":42}]"#)
            .code_reviews()
            .location(repository_location())
            .list()
            .await?;
        let releases = gitlab()
            .header("x-next-page", "3")
            .body(r#"[{"tag_name":"v1.0.0"}]"#)
            .releases()
            .location(repository_location())
            .list()
            .await?;

        assert_next(issues, "3");
        assert_next(code_reviews, "3");
        assert_next(releases, "3");

        Ok(())
    })
}

#[test]
fn gitlab_pipeline_lists_preserve_next_cursor() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let pipelines = gitlab()
            .header("x-next-page", "3")
            .body(r#"[{"id":42}]"#)
            .pipelines()
            .location(repository_location())
            .list()
            .await?;

        assert_next(pipelines, "3");

        Ok(())
    })
}

fn repository_location() -> Repo {
    repo().owner("akira-io").name("vcs-providers-rs").get()
}

fn assert_next<T>(page: Page<T>, expected_next: &str) {
    assert_eq!(
        page.next().map(|cursor| cursor.as_str()),
        Some(expected_next)
    );
}
