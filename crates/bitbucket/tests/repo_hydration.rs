use git_cognition_bitbucket::bitbucket;
use git_cognition_core::{LifecycleState, Repo, ReposFluent, Visibility, repo, run_async_test};

#[test]
fn bitbucket_client_hydrates_repository() -> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        let repository = bitbucket()
            .body(r#"{"full_name":"akira-io/git-cognition-rs","is_private":false}"#)
            .repos()
            .get(repository_location())
            .await?;

        assert_eq!(repository.provider().as_str(), "bitbucket");
        assert_eq!(repository.repo().owner().as_str(), "akira-io");
        assert_eq!(repository.repo().name().as_str(), "git-cognition-rs");
        assert_eq!(repository.visibility(), &Visibility::Public);
        assert_eq!(repository.lifecycle_state(), &LifecycleState::Active);

        Ok(())
    })
}

#[test]
fn bitbucket_client_hydrates_repository_list() -> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        let repositories = bitbucket()
            .body(r#"{"values":[{"full_name":"akira-io/git-cognition-rs","is_private":true}]}"#)
            .repos()
            .list(bitbucket().repo().query().optional_pagination(None).list())
            .await?;

        assert_eq!(repositories.items().len(), 1);
        assert_eq!(repositories.items()[0].visibility(), &Visibility::Private);

        Ok(())
    })
}

#[test]
fn bitbucket_client_hydrates_repository_create() -> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        let repository = bitbucket()
            .body(r#"{"full_name":"akira-io/git-cognition-rs","is_private":true}"#)
            .repos()
            .create()
            .location(repository_location())
            .visibility(Visibility::Private)
            .create()
            .await?;

        assert_eq!(repository.provider().as_str(), "bitbucket");
        assert_eq!(repository.visibility(), &Visibility::Private);

        Ok(())
    })
}

#[test]
fn bitbucket_client_hydrates_repository_update() -> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        let repository = bitbucket()
            .body(r#"{"full_name":"akira-io/git-cognition-rs","is_private":false}"#)
            .repos()
            .update()
            .location(repository_location())
            .visibility(Visibility::Public)
            .update()
            .await?;

        assert_eq!(repository.provider().as_str(), "bitbucket");
        assert_eq!(repository.visibility(), &Visibility::Public);

        Ok(())
    })
}

#[test]
fn bitbucket_client_deletes_repository() -> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        bitbucket()
            .status(204)
            .repos()
            .delete(repository_location())
            .await
    })
}

#[test]
fn bitbucket_client_hydrates_branches_and_commits() -> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        let branch_page = bitbucket()
            .body(r#"{"values":[{"name":"main"}]}"#)
            .repos()
            .branches(repository_location())
            .await?;
        let commit_page = bitbucket()
            .body(r#"{"values":[{"hash":"abc123"}]}"#)
            .repos()
            .commits(repository_location())
            .await?;

        assert_eq!(branch_page.items()[0].name(), "main");
        assert_eq!(commit_page.items()[0].id(), "abc123");

        Ok(())
    })
}

#[test]
fn bitbucket_client_hydrates_branch_create() -> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        let branch = bitbucket()
            .body(r#"{"name":"feature"}"#)
            .repos()
            .branch()
            .location(repository_location())
            .name("feature")
            .sha("abc123")
            .create()
            .await?;

        assert_eq!(branch.name(), "feature");

        Ok(())
    })
}

#[test]
fn bitbucket_client_deletes_branch() -> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        bitbucket()
            .status(204)
            .repos()
            .branch()
            .location(repository_location())
            .name("feature")
            .delete()
            .await
    })
}

fn repository_location() -> Repo {
    repo().owner("akira-io").name("git-cognition-rs").get()
}
