use vcs_provider_core::{EchoTransport, VcsResult, run_async_test, runtime};
use vcs_provider_github::github;

#[test]
fn github_provider_runs_through_core_runtime() -> VcsResult<()> {
    let response = run_async_test(async {
        runtime()
            .with_provider(github())
            .transport(EchoTransport)
            .request()
            .get("/repos/akira-io/core")
            .send()
            .await
    })?;

    assert_eq!(response.status().code(), 200);

    Ok(())
}

#[test]
fn github_provider_runs_through_fluent_provider_configuration() -> VcsResult<()> {
    let response = run_async_test(async {
        runtime()
            .provider()
            .from(github())
            .transport(EchoTransport)
            .request()
            .get("/repos/akira-io/core")
            .send()
            .await
    })?;

    assert_eq!(response.status().code(), 200);

    Ok(())
}
