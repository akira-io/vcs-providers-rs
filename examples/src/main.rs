use vcs_provider_core::{ProviderRegistry, VcsResult};

fn main() -> VcsResult<()> {
    let registry = ProviderRegistry::builder()
        .register(vcs_provider_github::provider())?
        .register(vcs_provider_gitlab::provider())?
        .register(vcs_provider_bitbucket::provider())?
        .build();

    for descriptor in registry.descriptors() {
        println!(
            "{} ({})",
            descriptor.display_name(),
            descriptor.id().as_str()
        );
    }

    Ok(())
}
