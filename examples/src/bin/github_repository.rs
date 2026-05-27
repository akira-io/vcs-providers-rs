use vcs_provider_core::{VcsResult, repo, run_async_test};
use vcs_provider_github::github;

fn main() -> VcsResult<()> {
    run_async_test(async {
        let repository = github()
            .body(r#"{"full_name":"akira-io/vcs-providers-rs","private":false}"#)
            .repos()
            .get(repo().owner("akira-io").name("vcs-providers-rs").get())
            .await?;

        println!(
            "{} {}",
            repository.provider().as_str(),
            repository.repo().name().as_str()
        );

        Ok(())
    })
}
