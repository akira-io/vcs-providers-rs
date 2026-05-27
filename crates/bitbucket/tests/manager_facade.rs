use vcs_provider_bitbucket::{BitbucketProvider, bitbucket};
use vcs_provider_core::{
    RequestMethod, auth, provider_response, provider_responses, rate_limit, repo, run_async_test,
    vcs,
};

#[test]
fn bitbucket_facade_builds_repo_requests() {
    let repository = vcs(bitbucket())
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();

    assert_eq!(
        repository.url().value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs"
    );
}

#[test]
fn bitbucket_facade_uses_configured_base_url() {
    let repository = vcs(bitbucket().base_url("https://bitbucket.internal.example/rest"))
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();

    assert_eq!(
        repository.url().value(),
        "https://bitbucket.internal.example/rest/repositories/akira-io/vcs-providers-rs"
    );
}

#[test]
fn bitbucket_facade_builds_code_review_requests() {
    let code_review = vcs(bitbucket())
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .code_review("42")
        .get();

    assert_eq!(
        code_review.url().value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/pullrequests/42"
    );
    assert_eq!(code_review.merge().method(), &RequestMethod::Post);
    assert_eq!(
        code_review.merge().url().value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/pullrequests/42/merge"
    );
}

#[test]
fn bitbucket_facade_builds_pipeline_requests() -> vcs_provider_core::VcsResult<()> {
    let pipeline = vcs(bitbucket())
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .pipeline("{pipeline-uuid}")
        .get();
    let pipelines = vcs(bitbucket())
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .pipelines()
        .pagination()
        .limit(50)
        .cursor("2")
        .url();

    assert_eq!(
        pipeline.url().value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/pipelines/%7Bpipeline-uuid%7D"
    );
    assert_eq!(
        pipelines.value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs/pipelines?pagelen=50&page=2"
    );
    assert_eq!(pipeline.cancel()?.method(), &RequestMethod::Post);

    Ok(())
}

#[test]
fn bitbucket_facade_builds_mutation_requests() {
    let create_request = vcs(bitbucket())
        .repo()
        .draft(repository())
        .visibility(vcs_provider_core::Visibility::Private)
        .create();

    assert_eq!(create_request.method(), &RequestMethod::Put);
}

#[test]
fn bitbucket_facade_executes_repo_client_with_auth() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let transport = provider_response()
            .body(r#"{"full_name":"akira-io/vcs-providers-rs","is_private":false}"#)
            .record();
        let repository = vcs(bitbucket())
            .transport(transport.clone())
            .auth(auth().oauth("bitbucket-token"))
            .repos()
            .get(repo().owner("akira-io").name("vcs-providers-rs").get())
            .await?;
        let requests = transport.requests();
        let auth_header = requests[0]
            .headers()
            .iter()
            .find(|header| header.name().as_str() == "authorization");

        assert_eq!(repository.repo().owner().as_str(), "akira-io");
        assert_eq!(
            requests[0].url().value(),
            "https://api.bitbucket.org/2.0/repositories/akira-io/vcs-providers-rs"
        );
        assert_eq!(
            auth_header.map(|header| header.value().as_str()),
            Some("Bearer bitbucket-token")
        );

        Ok(())
    })
}

#[test]
fn bitbucket_facade_executes_repo_client_with_middleware() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let transport = provider_response()
            .body(r#"{"full_name":"akira-io/vcs-providers-rs","is_private":false}"#)
            .record();
        let repository = vcs(bitbucket())
            .middleware(transport.clone())
            .header("x-request-id", "request-1")
            .auth(auth().oauth("bitbucket-token"))
            .repos()
            .get(repo().owner("akira-io").name("vcs-providers-rs").get())
            .await?;
        let requests = transport.requests();
        let request_headers = requests[0].headers();

        assert_eq!(repository.repo().owner().as_str(), "akira-io");
        assert!(request_headers.iter().any(|header| {
            header.name().as_str() == "authorization"
                && header.value().as_str() == "Bearer bitbucket-token"
        }));
        assert!(request_headers.iter().any(|header| {
            header.name().as_str() == "x-request-id" && header.value().as_str() == "request-1"
        }));

        Ok(())
    })
}

#[test]
fn bitbucket_facade_executes_client_with_configured_base_url() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let transport = provider_response()
            .body(r#"{"full_name":"akira-io/vcs-providers-rs","is_private":false}"#)
            .record();

        vcs(bitbucket().base_url("https://bitbucket.internal.example/rest"))
            .transport(transport.clone())
            .repos()
            .get(repo().owner("akira-io").name("vcs-providers-rs").get())
            .await?;

        assert_eq!(
            transport.requests()[0].url().value(),
            "https://bitbucket.internal.example/rest/repositories/akira-io/vcs-providers-rs"
        );

        Ok(())
    })
}

#[test]
fn bitbucket_facade_executes_repo_client_with_retry() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let provider_transport = provider_responses()
            .status(429)
            .body(r#"{"full_name":"akira-io/vcs-providers-rs","is_private":false}"#)
            .record();
        let repository = vcs(bitbucket())
            .retry(provider_transport.clone())
            .attempts(2)
            .on_status(429)
            .repos()
            .get(repo().owner("akira-io").name("vcs-providers-rs").get())
            .await?;

        assert_eq!(repository.repo().owner().as_str(), "akira-io");
        assert_eq!(provider_transport.requests().len(), 2);

        Ok(())
    })
}

#[test]
fn bitbucket_facade_observes_rate_limit_headers() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let recorder = rate_limit().recorder();
        let provider_transport = provider_response()
            .header("x-ratelimit-remaining", "40")
            .header("x-ratelimit-reset", "1710000002")
            .header("retry-after", "32")
            .body(r#"{"full_name":"akira-io/vcs-providers-rs","is_private":false}"#)
            .record();
        let repository = vcs(bitbucket())
            .rate_limit(provider_transport)
            .remaining(["x-ratelimit-remaining"])
            .reset_at(["x-ratelimit-reset"])
            .retry_after(["retry-after"])
            .recorder(recorder.clone())
            .repos()
            .get(repo().owner("akira-io").name("vcs-providers-rs").get())
            .await?;
        let observations = recorder.observations();

        assert_eq!(repository.repo().owner().as_str(), "akira-io");
        assert_eq!(observations.len(), 1);
        assert_eq!(
            observations[0].remaining().map(|quota| quota.as_u64()),
            Some(40)
        );
        assert_eq!(
            observations[0]
                .retry_after()
                .map(|retry_after| retry_after.as_str()),
            Some("32")
        );

        Ok(())
    })
}

fn repository() -> vcs_provider_core::ManagedRepo<BitbucketProvider> {
    vcs(bitbucket())
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get()
}
