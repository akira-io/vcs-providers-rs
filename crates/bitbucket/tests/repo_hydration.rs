use vcs_provider_bitbucket::bitbucket;
use vcs_provider_core::{
    LifecycleState, Repo, SingleResponseTransport, Visibility, provider_response, repo,
    run_async_test,
};

#[test]
fn bitbucket_client_hydrates_repository() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let repository = bitbucket()
            .client(provider_response_body(
                r#"{"full_name":"akira-io/vcs-providers-rs","is_private":false}"#,
            ))
            .repos()
            .get(repository_location())
            .await?;

        assert_eq!(repository.provider().as_str(), "bitbucket");
        assert_eq!(repository.repo().owner().as_str(), "akira-io");
        assert_eq!(repository.repo().name().as_str(), "vcs-providers-rs");
        assert_eq!(repository.visibility(), &Visibility::Public);
        assert_eq!(repository.lifecycle_state(), &LifecycleState::Active);

        Ok(())
    })
}

#[test]
fn bitbucket_client_hydrates_repository_list() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let repositories = bitbucket()
            .client(provider_response_body(
                r#"{"values":[{"full_name":"akira-io/vcs-providers-rs","is_private":true}]}"#,
            ))
            .repos()
            .list(bitbucket().repo().query().list(None))
            .await?;

        assert_eq!(repositories.items().len(), 1);
        assert_eq!(repositories.items()[0].visibility(), &Visibility::Private);

        Ok(())
    })
}

#[test]
fn bitbucket_client_hydrates_branches_and_commits() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let branch_page = bitbucket()
            .client(provider_response_body(r#"{"values":[{"name":"main"}]}"#))
            .repos()
            .branches(repository_location())
            .await?;
        let commit_page = bitbucket()
            .client(provider_response_body(r#"{"values":[{"hash":"abc123"}]}"#))
            .repos()
            .commits(repository_location())
            .await?;

        assert_eq!(branch_page.items()[0].name(), "main");
        assert_eq!(commit_page.items()[0].id(), "abc123");

        Ok(())
    })
}

fn repository_location() -> Repo {
    repo().owner("akira-io").name("vcs-providers-rs").get()
}

fn provider_response_body(body: &str) -> SingleResponseTransport {
    provider_response().body(body).get()
}
