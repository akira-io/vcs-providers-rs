mod support;

use vcs_provider_core::{TelemetryEvent, Transport, VcsResult, request, telemetry};

use support::EchoTransport;

#[test]
fn telemetry_transport_records_request_lifecycle() -> VcsResult<()> {
    let recorder = telemetry().recorder();
    let transport = telemetry()
        .transport(EchoTransport)
        .sink(recorder.clone())
        .build();
    let request = request().get("https://api.example.test/repos").build();
    let request_telemetry = telemetry().request().make(&request);

    let response = futures::executor::block_on(transport.send(request))?;
    let events = recorder.events();

    assert_eq!(response.status().code(), 200);
    assert_eq!(events.len(), 2);
    assert_eq!(
        events.first().cloned(),
        Some(TelemetryEvent::RequestStarted(request_telemetry))
    );
    assert!(matches!(
        events.get(1),
        Some(TelemetryEvent::RequestFinished(event)) if event.status_code() == 200
    ));

    Ok(())
}
