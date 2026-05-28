use git_cognition_bitbucket::bitbucket;
use git_cognition_core::{CognitionResult, EchoTransport, run_async_test, runtime};

#[test]
fn bitbucket_provider_runs_through_core_runtime() -> CognitionResult<()> {
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
