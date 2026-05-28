use vcs_provider_core::{ErrorKind, VcsError, error, response};

#[test]
fn error_builder_maps_http_status_to_universal_error() {
    assert_eq!(
        error().from_response(&response().status(401).build()),
        Some(VcsError::Unauthorized)
    );
    assert_eq!(
        error().from_response(&response().status(429).build()),
        Some(VcsError::RateLimited)
    );
    assert_eq!(
        error().from_response(&response().status(503).build()),
        Some(VcsError::ProviderUnavailable)
    );
    assert_eq!(
        error().unsupported_operation("issue delete").kind(),
        vcs_provider_core::ErrorKind::UnsupportedOperation
    );
    assert_eq!(error().from_response(&response().status(204).build()), None);
}

#[test]
fn vcs_error_exposes_stable_kind() {
    let duplicate_provider = error().provider_already_registered("github");
    let missing_provider = error().provider_not_registered("gitlab");
    let invalid_input = error().invalid_input("missing owner");

    assert_eq!(
        duplicate_provider.kind(),
        ErrorKind::ProviderAlreadyRegistered
    );
    assert_eq!(missing_provider.kind(), ErrorKind::ProviderNotRegistered);
    assert_eq!(invalid_input.kind(), ErrorKind::InvalidInput);
}
