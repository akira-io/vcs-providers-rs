use vcs_provider_core::{Transport, VcsResult, provider_responses, request, retry};

#[test]
fn retry_transport_retries_configured_response_statuses() -> VcsResult<()> {
    let provider_transport = provider_responses().status(500).status(200).record();
    let transport = retry()
        .transport(provider_transport.clone())
        .attempts(2)
        .on_status(500)
        .build();
    let request = request().get("https://api.example.test/repos").build();

    let response = futures::executor::block_on(transport.send(request))?;

    assert_eq!(response.status().code(), 200);
    assert_eq!(provider_transport.requests().len(), 2);

    Ok(())
}

#[test]
fn retry_transport_stops_after_max_attempts() -> VcsResult<()> {
    let provider_transport = provider_responses()
        .status(503)
        .status(503)
        .status(200)
        .record();
    let transport = retry()
        .transport(provider_transport.clone())
        .attempts(2)
        .on_status(503)
        .build();
    let request = request().get("https://api.example.test/repos").build();

    let response = futures::executor::block_on(transport.send(request))?;

    assert_eq!(response.status().code(), 503);
    assert_eq!(provider_transport.requests().len(), 2);

    Ok(())
}

#[test]
fn retry_transport_keeps_successful_responses_single_attempt() -> VcsResult<()> {
    let provider_transport = provider_responses().status(200).record();
    let transport = retry().transport(provider_transport.clone()).build();
    let request = request().get("https://api.example.test/repos").build();

    let response = futures::executor::block_on(transport.send(request))?;

    assert_eq!(response.status().code(), 200);
    assert_eq!(provider_transport.requests().len(), 1);

    Ok(())
}
