use std::time::Duration;

use git_cognition_core::{CognitionError, Transport, http, request, run_async_test};

#[test]
fn http_transport_builder_creates_real_transport() -> git_cognition_core::CognitionResult<()> {
    http()
        .transport()
        .timeout(Duration::from_secs(10))
        .user_agent("cognition-provider-rs-test")
        .get()?;

    Ok(())
}

#[test]
fn http_transport_maps_invalid_request_to_universal_error()
-> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        let result = http()
            .transport()
            .get()?
            .send(request().get("://invalid").build())
            .await;

        assert!(matches!(result, Err(CognitionError::InvalidInput(_))));

        Ok(())
    })
}
