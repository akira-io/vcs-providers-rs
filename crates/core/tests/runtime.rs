mod support;

use git_cognition_core::{
    CognitionError, CognitionResult, auth, error, response, run_async_test, runtime, telemetry,
};

use support::EchoTransport;

#[test]
fn runtime_builds_provider_request_with_auth() {
    let request = runtime()
        .provider()
        .base_url("https://api.example.test")
        .bearer_auth()
        .transport(EchoTransport)
        .request()
        .get("/repos")
        .auth(&auth().personal_access_token("test-token"))
        .build();

    assert_eq!(request.url().as_str(), "https://api.example.test/repos");
    assert_eq!(request.headers()[0].name().as_str(), "authorization");
    assert_eq!(request.headers()[0].value().as_str(), "Bearer test-token");
}

#[test]
fn runtime_executes_transport_with_telemetry() -> CognitionResult<()> {
    let recorder = telemetry().recorder();
    let response = run_async_test(async {
        runtime()
            .provider()
            .telemetry(recorder.clone())
            .transport(EchoTransport)
            .request()
            .get("/repos")
            .send()
            .await
    })?;

    let events = recorder.events();

    assert_eq!(response.status().code(), 200);
    assert_eq!(events.len(), 2);

    Ok(())
}

#[test]
fn runtime_request_builder_still_exposes_request() {
    let request = runtime()
        .provider()
        .transport(EchoTransport)
        .request()
        .get("/repos")
        .build();

    assert_eq!(request.url().as_str(), "https://api.example.test/repos");
}

#[test]
fn runtime_maps_failed_response_status_to_universal_error() {
    let response = response().status(429).build();

    assert_eq!(
        error().from_response(&response),
        Some(CognitionError::RateLimited)
    );
}
