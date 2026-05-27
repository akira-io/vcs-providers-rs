use vcs_provider_bitbucket::bitbucket;
use vcs_provider_core::{CodeReviewsFluent, Page, PipelinesFluent, Repo, repo, run_async_test};

const NEXT_URL: &str =
    "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/items?page=2";

#[test]
fn bitbucket_repo_lists_preserve_next_cursor() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let repositories = bitbucket()
            .body(
                r#"{"next":"https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/items?page=2","values":[{"full_name":"akira-io/vcs-providers-rs","is_private":false}]}"#,
            )
            .repos()
            .list(bitbucket().repo().query().optional_pagination(None).list())
            .await?;
        let branches = bitbucket()
            .body(
                r#"{"next":"https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/items?page=2","values":[{"name":"main"}]}"#,
            )
            .repos()
            .branches(repository_location())
            .await?;

        assert_next(repositories);
        assert_next(branches);

        Ok(())
    })
}

#[test]
fn bitbucket_code_review_lists_preserve_next_cursor() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let code_reviews = bitbucket()
            .body(
                r#"{"next":"https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/items?page=2","values":[{"id":42}]}"#,
            )
            .code_reviews()
            .location(repository_location())
            .list()
            .await?;

        assert_next(code_reviews);

        Ok(())
    })
}

#[test]
fn bitbucket_pipeline_lists_preserve_next_cursor() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let pipelines = bitbucket()
            .body(
                r#"{"next":"https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/items?page=2","values":[{"uuid":"{pipeline-uuid}"}]}"#,
            )
            .pipelines()
            .location(repository_location())
            .list()
            .await?;

        assert_next(pipelines);

        Ok(())
    })
}

fn repository_location() -> Repo {
    repo().owner("akira-io").name("vcs-providers-rs").get()
}

fn assert_next<T>(page: Page<T>) {
    assert_eq!(page.next().map(|cursor| cursor.as_str()), Some(NEXT_URL));
}
