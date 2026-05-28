use git_cognition_core::{Capability, capabilities};

#[test]
fn capability_set_reports_supported_capabilities() {
    let capabilities = capabilities().make([
        Capability::Repos,
        Capability::AuthenticationValidate,
        Capability::RepoCreate,
        Capability::RepoBranchCreate,
        Capability::OrganizationList,
        Capability::Pipelines,
        Capability::PipelineRerun,
    ]);

    assert!(capabilities.supports(&Capability::Repos));
    assert!(capabilities.supports(&Capability::AuthenticationValidate));
    assert!(capabilities.supports(&Capability::RepoCreate));
    assert!(capabilities.supports(&Capability::RepoBranchCreate));
    assert!(capabilities.supports(&Capability::OrganizationList));
    assert!(capabilities.supports(&Capability::Pipelines));
    assert!(capabilities.supports(&Capability::PipelineRerun));
    assert!(!capabilities.supports(&Capability::Releases));
}

#[test]
fn capability_catalog_contains_contract_and_reserved_capabilities() {
    let all_capabilities = Capability::all();

    assert!(all_capabilities.contains(&Capability::Repos));
    assert!(all_capabilities.contains(&Capability::AuthenticationValidate));
    assert!(all_capabilities.contains(&Capability::RepoBranchDelete));
    assert!(all_capabilities.contains(&Capability::OrganizationList));
    assert!(all_capabilities.contains(&Capability::ReleaseDelete));
    assert!(all_capabilities.contains(&Capability::Webhooks));
}
