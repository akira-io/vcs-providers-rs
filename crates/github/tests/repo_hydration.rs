use vcs_provider_core::{
    LifecycleState, Repo, SingleResponseTransport, Visibility, provider_response, repo,
};
use vcs_provider_github::github;

#[test]
fn github_client_hydrates_repository() -> vcs_provider_core::VcsResult<()> {
    let repository = futures::executor::block_on(
        github()
            .client(provider_response_body(
                r#"{"full_name":"akira-io/vcs-providers-rs","private":false,"archived":false,"disabled":false}"#,
            ))
            .repos()
            .get(repository_location()),
    )?;

    assert_eq!(repository.provider().as_str(), "github");
    assert_eq!(repository.repo().owner().as_str(), "akira-io");
    assert_eq!(repository.repo().name().as_str(), "vcs-providers-rs");
    assert_eq!(repository.visibility(), &Visibility::Public);
    assert_eq!(repository.lifecycle_state(), &LifecycleState::Active);

    Ok(())
}

#[test]
fn github_client_hydrates_repository_list() -> vcs_provider_core::VcsResult<()> {
    let repositories = futures::executor::block_on(
        github()
            .client(provider_response_body(
                r#"[{"full_name":"akira-io/vcs-providers-rs","private":true,"archived":true,"disabled":false}]"#,
            ))
            .repos()
            .list(github().repo().query().list(None)),
    )?;

    assert_eq!(repositories.items().len(), 1);
    assert_eq!(repositories.items()[0].visibility(), &Visibility::Private);
    assert_eq!(
        repositories.items()[0].lifecycle_state(),
        &LifecycleState::Archived
    );

    Ok(())
}

#[test]
fn github_client_hydrates_branches_and_commits() -> vcs_provider_core::VcsResult<()> {
    let branch_page = futures::executor::block_on(
        github()
            .client(provider_response_body(r#"[{"name":"main"}]"#))
            .repos()
            .branches(repository_location()),
    )?;
    let commit_page = futures::executor::block_on(
        github()
            .client(provider_response_body(r#"[{"sha":"abc123"}]"#))
            .repos()
            .commits(repository_location()),
    )?;

    assert_eq!(branch_page.items()[0].name(), "main");
    assert_eq!(commit_page.items()[0].id(), "abc123");

    Ok(())
}

fn repository_location() -> Repo {
    repo().owner("akira-io").name("vcs-providers-rs").get()
}

fn provider_response_body(body: &str) -> SingleResponseTransport {
    provider_response().body(body).get()
}
