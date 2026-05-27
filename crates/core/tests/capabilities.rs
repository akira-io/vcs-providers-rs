use vcs_provider_core::{Capability, capabilities};

#[test]
fn capability_set_reports_supported_capabilities() {
    let capabilities = capabilities().make([
        Capability::Repos,
        Capability::RepoCreate,
        Capability::Pipelines,
        Capability::PipelineRerun,
    ]);

    assert!(capabilities.supports(&Capability::Repos));
    assert!(capabilities.supports(&Capability::RepoCreate));
    assert!(capabilities.supports(&Capability::Pipelines));
    assert!(capabilities.supports(&Capability::PipelineRerun));
    assert!(!capabilities.supports(&Capability::Releases));
}

#[test]
fn capability_catalog_contains_contract_and_reserved_capabilities() {
    let all_capabilities = Capability::all();

    assert!(all_capabilities.contains(&Capability::Repos));
    assert!(all_capabilities.contains(&Capability::ReleaseDelete));
    assert!(all_capabilities.contains(&Capability::Webhooks));
}
