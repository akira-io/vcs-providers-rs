use git_cognition_core::{CognitionError, ErrorKind, error, response};

#[test]
fn error_builder_maps_http_status_to_universal_error() {
    assert_eq!(
        error().from_response(&response().status(401).build()),
        Some(CognitionError::Unauthorized)
    );
    assert_eq!(
        error().from_response(&response().status(429).build()),
        Some(CognitionError::RateLimited)
    );
    assert_eq!(
        error().from_response(&response().status(503).build()),
        Some(CognitionError::ProviderUnavailable)
    );
    assert_eq!(
        error().unsupported_operation("issue delete").kind(),
        git_cognition_core::ErrorKind::UnsupportedOperation
    );
    assert_eq!(error().from_response(&response().status(204).build()), None);
}

#[test]
fn cognition_error_exposes_stable_kind() {
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
