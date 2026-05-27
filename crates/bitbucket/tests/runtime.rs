use vcs_provider_bitbucket::bitbucket;
use vcs_provider_core::{EchoTransport, VcsResult, run_async_test, runtime};

#[test]
fn bitbucket_provider_runs_through_core_runtime() -> VcsResult<()> {
    let response = run_async_test(async {
        runtime()
            .with_provider(bitbucket())
            .transport(EchoTransport)
            .request()
            .get("/repositories/akira-io")
            .send()
            .await
    })?;

    assert_eq!(response.status().code(), 200);

    Ok(())
}
