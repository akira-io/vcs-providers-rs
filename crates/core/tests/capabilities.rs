use vcs_provider_core::{Capability, CapabilitySet};

#[test]
fn capability_set_reports_supported_capabilities() {
    let capabilities = CapabilitySet::make([Capability::Repos, Capability::Pipelines]);

    assert!(capabilities.supports(&Capability::Repos));
    assert!(capabilities.supports(&Capability::Pipelines));
    assert!(!capabilities.supports(&Capability::Releases));
}
