use git_cognition_bitbucket::bitbucket;
use git_cognition_core::{
    TelemetryEvent, auth, cognition, rate_limit, repo, run_async_test, telemetry,
};

#[test]
fn bitbucket_facade_composes_middleware_retry_and_rate_limit()
-> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        let recorder = rate_limit().recorder();
        let telemetry_recorder = telemetry().recorder();
        let recording_transport = bitbucket()
            .responses()
            .response()
            .status(429)
            .header("x-ratelimit-remaining", "0")
            .next_response()
            .response()
            .header("x-ratelimit-remaining", "40")
            .body(r#"{"full_name":"akira-io/git-cognition-rs","is_private":false}"#)
            .next_response()
            .record();
        let repository = cognition()
            .provider(bitbucket())
            .middleware(recording_transport.clone())
            .header("x-request-id", "request-1")
            .retry()
            .attempts(2)
            .on_status(429)
            .rate_limit()
            .remaining(["x-ratelimit-remaining"])
            .recorder(recorder.clone())
            .telemetry(telemetry_recorder.clone())
            .auth(auth().oauth("bitbucket-token"))
            .repos()
            .get(repo().owner("akira-io").name("git-cognition-rs").get())
            .await?;
        let requests = recording_transport.requests();
        let observations = recorder.observations();
        let telemetry_events = telemetry_recorder.events();

        assert_eq!(repository.repo().owner().as_str(), "akira-io");
        assert_eq!(requests.len(), 2);
        assert!(requests[0].headers().iter().any(|header| {
            header.name().as_str() == "x-request-id" && header.value().as_str() == "request-1"
        }));
        assert_eq!(observations.len(), 2);
        assert_eq!(
            observations[1].remaining().map(|quota| quota.as_u64()),
            Some(40)
        );
        assert_eq!(telemetry_events.len(), 2);
        assert!(matches!(
            telemetry_events.first(),
            Some(TelemetryEvent::RequestStarted(_))
        ));
        assert!(matches!(
            telemetry_events.get(1),
            Some(TelemetryEvent::RequestFinished(event)) if event.status_code() == 200
        ));

        Ok(())
    })
}
