use vcs_provider_core::{
    ResponseSequenceTransport, auth, rate_limit, repo, response, run_async_test, vcs,
};
use vcs_provider_gitlab::gitlab;

#[test]
fn gitlab_facade_composes_middleware_retry_and_rate_limit() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let recorder = rate_limit().recorder();
        let provider_transport = ResponseSequenceTransport::make([
            response()
                .status(429)
                .header("ratelimit-remaining", "0")
                .build(),
            response()
                .header("ratelimit-remaining", "41")
                .body(
                    r#"{"path_with_namespace":"akira-io/vcs-providers-rs","visibility":"public"}"#,
                )
                .build(),
        ]);
        let repository = vcs(gitlab())
            .middleware(provider_transport.clone())
            .header("x-request-id", "request-1")
            .retry()
            .attempts(2)
            .on_status(429)
            .rate_limit()
            .remaining(["ratelimit-remaining"])
            .recorder(recorder.clone())
            .auth(auth().personal_access_token("gitlab-token"))
            .repos()
            .get(repo().owner("akira-io").name("vcs-providers-rs").get())
            .await?;
        let requests = provider_transport.requests();
        let observations = recorder.observations();

        assert_eq!(repository.repo().owner().as_str(), "akira-io");
        assert_eq!(requests.len(), 2);
        assert!(requests[0].headers().iter().any(|header| {
            header.name().as_str() == "x-request-id" && header.value().as_str() == "request-1"
        }));
        assert_eq!(observations.len(), 2);
        assert_eq!(
            observations[1].remaining().map(|quota| quota.as_u64()),
            Some(41)
        );

        Ok(())
    })
}
