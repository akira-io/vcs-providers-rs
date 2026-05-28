use git_cognition_core::{CognitionResult, EchoTransport, run_async_test, runtime};
use git_cognition_gitlab::gitlab;

#[test]
fn gitlab_provider_runs_through_core_runtime() -> CognitionResult<()> {
    let response = run_async_test(async {
        runtime()
            .with_provider(gitlab())
            .transport(EchoTransport)
            .request()
            .get("/api/v4/projects")
            .send()
            .await
    })?;

    assert_eq!(response.status().code(), 200);

    Ok(())
}
