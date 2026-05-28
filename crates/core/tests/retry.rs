use vcs_provider_core::{Transport, VcsResult, request, retry, run_async_test, test_transport};

#[test]
fn retry_transport_retries_configured_response_statuses() -> VcsResult<()> {
    let recording_transport = test_transport()
        .responses()
        .status(500)
        .status(200)
        .record();
    let transport = retry()
        .transport(recording_transport.clone())
        .attempts(2)
        .on_status(500)
        .build();
    let request = request().get("https://api.example.test/repos").build();

    let response = run_async_test(transport.send(request))?;

    assert_eq!(response.status().code(), 200);
    assert_eq!(recording_transport.requests().len(), 2);

    Ok(())
}

#[test]
fn retry_transport_stops_after_max_attempts() -> VcsResult<()> {
    let recording_transport = test_transport()
        .responses()
        .status(503)
        .status(503)
        .status(200)
        .record();
    let transport = retry()
        .transport(recording_transport.clone())
        .attempts(2)
        .on_status(503)
        .build();
    let request = request().get("https://api.example.test/repos").build();

    let response = run_async_test(transport.send(request))?;

    assert_eq!(response.status().code(), 503);
    assert_eq!(recording_transport.requests().len(), 2);

    Ok(())
}

#[test]
fn retry_transport_keeps_successful_responses_single_attempt() -> VcsResult<()> {
    let recording_transport = test_transport().responses().status(200).record();
    let transport = retry().transport(recording_transport.clone()).build();
    let request = request().get("https://api.example.test/repos").build();

    let response = run_async_test(transport.send(request))?;

    assert_eq!(response.status().code(), 200);
    assert_eq!(recording_transport.requests().len(), 1);

    Ok(())
}
