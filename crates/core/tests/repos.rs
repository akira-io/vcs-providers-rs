use vcs_provider_core::{LifecycleState, Visibility, repo};

#[test]
fn repository_resource_uses_order_independent_repo_builder() {
    let repository = repo()
        .name("vcs-providers-rs")
        .owner("akira-io")
        .provider("github")
        .visibility(Visibility::Public)
        .lifecycle(LifecycleState::Active)
        .get();

    assert_eq!(repository.provider().as_str(), "github");
    assert_eq!(repository.repo().owner().as_str(), "akira-io");
    assert_eq!(repository.repo().name().as_str(), "vcs-providers-rs");
    assert_eq!(repository.visibility(), &Visibility::Public);
    assert_eq!(repository.lifecycle_state(), &LifecycleState::Active);
}
