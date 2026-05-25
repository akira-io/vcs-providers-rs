use vcs_provider_core::{RequestHeader, Response, ResponseStatus, rate_limit};

#[test]
fn rate_limit_profile_reads_configured_headers() {
    let profile = rate_limit()
        .headers()
        .remaining(["x-ratelimit-remaining", "ratelimit-remaining"])
        .reset_at(["x-ratelimit-reset", "ratelimit-reset"])
        .retry_after(["retry-after"])
        .cost(["x-ratelimit-used", "ratelimit-used"])
        .build();
    let response = Response::make(
        ResponseStatus::make(200),
        vec![
            RequestHeader::make("x-ratelimit-remaining", "42"),
            RequestHeader::make("x-ratelimit-reset", "1710000000"),
            RequestHeader::make("retry-after", "30"),
            RequestHeader::make("x-ratelimit-used", "7"),
        ],
    );
    let observation = profile.observe(&response);

    assert_eq!(
        observation.remaining().map(|quota| quota.as_u64()),
        Some(42)
    );
    assert_eq!(
        observation.reset_at().map(|reset_at| reset_at.as_str()),
        Some("1710000000")
    );
    assert_eq!(
        observation
            .retry_after()
            .map(|retry_after| retry_after.as_str()),
        Some("30")
    );
    assert_eq!(observation.cost().map(|cost| cost.as_u64()), Some(7));
}

#[test]
fn rate_limit_profile_ignores_unconfigured_headers() {
    let profile = rate_limit()
        .headers()
        .remaining(["ratelimit-remaining"])
        .build();
    let response = Response::make(
        ResponseStatus::make(200),
        vec![RequestHeader::make("x-ratelimit-remaining", "42")],
    );
    let observation = profile.observe(&response);

    assert_eq!(observation.remaining(), None);
    assert_eq!(observation.reset_at(), None);
    assert_eq!(observation.retry_after(), None);
    assert_eq!(observation.cost(), None);
}

#[test]
fn rate_limit_profile_matches_headers_case_insensitively() {
    let profile = rate_limit()
        .headers()
        .remaining(["x-ratelimit-remaining"])
        .build();
    let response = Response::make(
        ResponseStatus::make(200),
        vec![RequestHeader::make("X-RateLimit-Remaining", "42")],
    );
    let observation = profile.observe(&response);

    assert_eq!(
        observation.remaining().map(|quota| quota.as_u64()),
        Some(42)
    );
}
