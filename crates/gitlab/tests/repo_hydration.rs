use vcs_provider_core::{LifecycleState, SingleResponseTransport, Visibility, repo, response};
use vcs_provider_gitlab::gitlab;

#[test]
fn gitlab_transport_backed_repos_hydrate_repository() -> vcs_provider_core::VcsResult<()> {
    let repository = futures::executor::block_on(
        gitlab()
            .transport(SingleResponseTransport::make(
                response()
                    .body(
                        r#"{"path_with_namespace":"akira-io/vcs-providers-rs","visibility":"internal","archived":false}"#,
                    )
                    .build(),
            ))
            .repos()
            .get(repo().owner("akira-io").name("vcs-providers-rs").get()),
    )?;

    assert_eq!(repository.provider().as_str(), "gitlab");
    assert_eq!(repository.repo().owner().as_str(), "akira-io");
    assert_eq!(repository.repo().name().as_str(), "vcs-providers-rs");
    assert_eq!(repository.visibility(), &Visibility::Internal);
    assert_eq!(repository.lifecycle_state(), &LifecycleState::Active);

    Ok(())
}

#[test]
fn gitlab_transport_backed_repos_hydrate_repository_list() -> vcs_provider_core::VcsResult<()> {
    let repositories = futures::executor::block_on(
        gitlab()
            .transport(SingleResponseTransport::make(
                response()
                    .body(
                        r#"[{"path_with_namespace":"platform/akira-io/vcs-providers-rs","visibility":"private","archived":true}]"#,
                    )
                    .build(),
            ))
            .repos()
            .list(gitlab().repo().query().list(None)),
    )?;

    assert_eq!(repositories.items().len(), 1);
    assert_eq!(
        repositories.items()[0].repo().owner().as_str(),
        "platform/akira-io"
    );
    assert_eq!(repositories.items()[0].visibility(), &Visibility::Private);
    assert_eq!(
        repositories.items()[0].lifecycle_state(),
        &LifecycleState::Archived
    );

    Ok(())
}

#[test]
fn gitlab_transport_backed_repos_hydrate_branches_and_commits() -> vcs_provider_core::VcsResult<()>
{
    let branch_page = futures::executor::block_on(
        gitlab()
            .transport(SingleResponseTransport::make(
                response().body(r#"[{"name":"main"}]"#).build(),
            ))
            .repos()
            .branches(repo().owner("akira-io").name("vcs-providers-rs").get()),
    )?;
    let commit_page = futures::executor::block_on(
        gitlab()
            .transport(SingleResponseTransport::make(
                response().body(r#"[{"id":"abc123"}]"#).build(),
            ))
            .repos()
            .commits(repo().owner("akira-io").name("vcs-providers-rs").get()),
    )?;

    assert_eq!(branch_page.items()[0].name(), "main");
    assert_eq!(commit_page.items()[0].id(), "abc123");

    Ok(())
}
