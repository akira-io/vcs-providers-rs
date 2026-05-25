use vcs_provider_core::{ProviderRegistry, VcsResult};

fn main() -> VcsResult<()> {
    let registry = ProviderRegistry::builder()
        .register(vcs_provider_github::driver())?
        .register(vcs_provider_gitlab::driver())?
        .register(vcs_provider_bitbucket::driver())?
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
