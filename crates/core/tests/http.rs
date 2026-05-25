use std::time::Duration;

use vcs_provider_core::{Transport, VcsError, http, request, run_async_test};

#[test]
fn http_transport_builder_creates_real_transport() -> vcs_provider_core::VcsResult<()> {
    http()
        .transport()
        .timeout(Duration::from_secs(10))
        .user_agent("vcs-provider-rs-test")
        .get()?;

    Ok(())
}

#[test]
fn http_transport_maps_invalid_request_to_universal_error() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let result = http()
            .transport()
            .get()?
            .send(request().get("://invalid").build())
            .await;

        assert!(matches!(result, Err(VcsError::InvalidInput(_))));

        Ok(())
    })
}
