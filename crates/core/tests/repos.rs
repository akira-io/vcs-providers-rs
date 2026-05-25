use vcs_provider_core::{LifecycleState, ProviderId, Repository, Visibility, repo};

#[test]
fn repository_resource_is_provider_neutral() {
    let repo = repo().owner("akira-io").name("core").build();
    let repository = Repository::make(
        ProviderId::make("github"),
        repo,
        Visibility::Public,
        LifecycleState::Active,
    );

    assert_eq!(repository.provider().as_str(), "github");
    assert_eq!(repository.repo().owner().as_str(), "akira-io");
    assert_eq!(repository.repo().name().as_str(), "core");
    assert_eq!(repository.visibility(), &Visibility::Public);
    assert_eq!(repository.lifecycle_state(), &LifecycleState::Active);
}

#[test]
fn repo_builder_accepts_name_before_owner() {
    let repo = repo().name("vcs-providers-rs").owner("akira-io").build();

    assert_eq!(repo.owner().as_str(), "akira-io");
    assert_eq!(repo.name().as_str(), "vcs-providers-rs");
}
