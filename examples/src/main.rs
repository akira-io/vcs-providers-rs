use vcs_provider_core::{VcsResult, provider};

fn main() -> VcsResult<()> {
    let registry = provider()
        .register(vcs_provider_github::github())?
        .register(vcs_provider_gitlab::gitlab())?
        .register(vcs_provider_bitbucket::bitbucket())?
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
