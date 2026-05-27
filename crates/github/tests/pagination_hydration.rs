use vcs_provider_core::{
    CodeReviewsFluent, IssuesFluent, Page, PipelinesFluent, ReleasesFluent, Repo, repo,
    run_async_test,
};
use vcs_provider_github::github;

const NEXT_LINK: &str =
    r#"<https://api.github.com/repos/akira-io/vcs-providers-rs/items?page=2>; rel="next""#;

#[test]
fn github_repo_lists_preserve_next_cursor() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let repositories = github()
            .header("link", NEXT_LINK)
            .body(r#"[{"full_name":"akira-io/vcs-providers-rs","private":false}]"#)
            .repos()
            .list(github().repo().query().optional_pagination(None).list())
            .await?;
        let branches = github()
            .header("link", NEXT_LINK)
            .body(r#"[{"name":"main"}]"#)
            .repos()
            .branches(repository_location())
            .await?;

        assert_next(repositories, "2");
        assert_next(branches, "2");

        Ok(())
    })
}

#[test]
fn github_collaboration_lists_preserve_next_cursor() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let issues = github()
            .header("link", NEXT_LINK)
            .body(r#"[{"number":42}]"#)
            .issues()
            .location(repository_location())
            .list()
            .await?;
        let code_reviews = github()
            .header("link", NEXT_LINK)
            .body(r#"[{"number":42}]"#)
            .code_reviews()
            .location(repository_location())
            .list()
            .await?;
        let releases = github()
            .header("link", NEXT_LINK)
            .body(r#"[{"id":123}]"#)
            .releases()
            .location(repository_location())
            .list()
            .await?;

        assert_next(issues, "2");
        assert_next(code_reviews, "2");
        assert_next(releases, "2");

        Ok(())
    })
}

#[test]
fn github_pipeline_lists_preserve_next_cursor() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let pipelines = github()
            .header("link", NEXT_LINK)
            .body(r#"{"workflow_runs":[{"id":42}]}"#)
            .pipelines()
            .location(repository_location())
            .list()
            .await?;

        assert_next(pipelines, "2");

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
