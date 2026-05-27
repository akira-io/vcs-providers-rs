use vcs_provider_core::{LifecycleState, Repo, ReposFluent, Visibility, repo, run_async_test};
use vcs_provider_github::github;

#[test]
fn github_client_hydrates_repository() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let repository = github()
            .body(
                r#"{"full_name":"akira-io/vcs-providers-rs","private":false,"archived":false,"disabled":false}"#,
            )
            .repos()
            .get(repository_location())
            .await?;

        assert_eq!(repository.provider().as_str(), "github");
        assert_eq!(repository.repo().owner().as_str(), "akira-io");
        assert_eq!(repository.repo().name().as_str(), "vcs-providers-rs");
        assert_eq!(repository.visibility(), &Visibility::Public);
        assert_eq!(repository.lifecycle_state(), &LifecycleState::Active);

        Ok(())
    })
}

#[test]
fn github_client_hydrates_repository_list() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let repositories = github()
            .body(
                r#"[{"full_name":"akira-io/vcs-providers-rs","private":true,"archived":true,"disabled":false}]"#,
            )
            .repos()
            .list(github().repo().query().optional_pagination(None).list())
            .await?;

        assert_eq!(repositories.items().len(), 1);
        assert_eq!(repositories.items()[0].visibility(), &Visibility::Private);
        assert_eq!(
            repositories.items()[0].lifecycle_state(),
            &LifecycleState::Archived
        );

        Ok(())
    })
}

#[test]
fn github_client_hydrates_repository_create() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let repository = github()
            .body(
                r#"{"full_name":"akira-io/vcs-providers-rs","private":true,"archived":false,"disabled":false}"#,
            )
            .repos()
            .create()
            .location(repository_location())
            .visibility(Visibility::Private)
            .create()
            .await?;

        assert_eq!(repository.provider().as_str(), "github");
        assert_eq!(repository.visibility(), &Visibility::Private);

        Ok(())
    })
}

#[test]
fn github_client_hydrates_repository_update() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let repository = github()
            .body(
                r#"{"full_name":"akira-io/vcs-providers-rs","private":false,"archived":false,"disabled":false}"#,
            )
            .repos()
            .update()
            .location(repository_location())
            .visibility(Visibility::Public)
            .update()
            .await?;

        assert_eq!(repository.provider().as_str(), "github");
        assert_eq!(repository.visibility(), &Visibility::Public);

        Ok(())
    })
}

#[test]
fn github_client_deletes_repository() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        github()
            .status(204)
            .repos()
            .delete(repository_location())
            .await
    })
}

#[test]
fn github_client_hydrates_branches_and_commits() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let branch_page = github()
            .body(r#"[{"name":"main"}]"#)
            .repos()
            .branches(repository_location())
            .await?;
        let commit_page = github()
            .body(r#"[{"sha":"abc123"}]"#)
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
