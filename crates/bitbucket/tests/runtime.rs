use vcs_provider_bitbucket::bitbucket;
use vcs_provider_core::{EchoTransport, VcsResult, runtime};

#[test]
fn bitbucket_provider_runs_through_core_runtime() -> VcsResult<()> {
    let response = futures::executor::block_on(async {
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
