use vcs_provider_core::{ErrorKind, ResponseStatus, VcsError, error};

#[test]
fn error_builder_maps_http_status_to_universal_error() {
    assert_eq!(
        error().from_status(&ResponseStatus::make(401)),
        Some(VcsError::Unauthorized)
    );
    assert_eq!(
        error().from_status(&ResponseStatus::make(429)),
        Some(VcsError::RateLimited)
    );
    assert_eq!(
        error().from_status(&ResponseStatus::make(503)),
        Some(VcsError::ProviderUnavailable)
    );
    assert_eq!(error().from_status(&ResponseStatus::make(204)), None);
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
