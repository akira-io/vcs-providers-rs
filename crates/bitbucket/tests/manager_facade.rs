use git_cognition_bitbucket::{BitbucketProvider, bitbucket};
use git_cognition_core::{RequestMethod, auth, cognition, rate_limit, repo, run_async_test};

#[test]
fn bitbucket_facade_builds_repo_requests() {
    let repository = cognition()
        .provider(bitbucket())
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .get();

    assert_eq!(
        repository.url().value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/git-cognition-rs"
    );
}

#[test]
fn bitbucket_facade_uses_configured_base_url() {
    let repository = cognition()
        .provider(bitbucket().base_url("https://bitbucket.internal.example/rest"))
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .get();

    assert_eq!(
        repository.url().value(),
        "https://bitbucket.internal.example/rest/repositories/akira-io/git-cognition-rs"
    );
}

#[test]
fn bitbucket_facade_builds_code_review_requests() {
    let code_review = cognition()
        .provider(bitbucket())
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .code_review("42")
        .get();

    assert_eq!(
        code_review.url().value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/git-cognition-rs/pullrequests/42"
    );
    assert_eq!(code_review.merge().method(), &RequestMethod::Post);
    assert_eq!(
        code_review.merge().url().value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/git-cognition-rs/pullrequests/42/merge"
    );
}

#[test]
fn bitbucket_facade_builds_pipeline_requests() -> git_cognition_core::CognitionResult<()> {
    let pipeline = cognition()
        .provider(bitbucket())
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .pipeline("{pipeline-uuid}")
        .get();
    let pipelines = cognition()
        .provider(bitbucket())
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .pipelines()
        .pagination()
        .limit(50)
        .cursor("2")
        .url();

    assert_eq!(
        pipeline.url().value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/git-cognition-rs/pipelines/%7Bpipeline-uuid%7D"
    );
    assert_eq!(
        pipelines.value(),
        "https://api.bitbucket.org/2.0/repositories/akira-io/git-cognition-rs/pipelines?pagelen=50&page=2"
    );
    assert_eq!(pipeline.cancel()?.method(), &RequestMethod::Post);

    Ok(())
}

#[test]
fn bitbucket_facade_builds_mutation_requests() {
    let create_request = cognition()
        .provider(bitbucket())
        .repo()
        .draft(repository())
        .visibility(git_cognition_core::Visibility::Private)
        .create();

    assert_eq!(create_request.method(), &RequestMethod::Put);
}

#[test]
fn bitbucket_facade_executes_repo_client_with_auth() -> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        let transport = bitbucket()
            .body(r#"{"full_name":"akira-io/git-cognition-rs","is_private":false}"#)
            .record();
        let repository = cognition()
            .provider(bitbucket())
            .transport(transport.clone())
            .auth(auth().oauth("bitbucket-token"))
            .repos()
            .get(repo().owner("akira-io").name("git-cognition-rs").get())
            .await?;
        let requests = transport.requests();
        let auth_header = requests[0]
            .headers()
            .iter()
            .find(|header| header.name().as_str() == "authorization");

        assert_eq!(repository.repo().owner().as_str(), "akira-io");
        assert_eq!(
            requests[0].url().value(),
            "https://api.bitbucket.org/2.0/repositories/akira-io/git-cognition-rs"
        );
        assert_eq!(
            auth_header.map(|header| header.value().as_str()),
            Some("Bearer bitbucket-token")
        );

        Ok(())
    })
}

#[test]
fn bitbucket_facade_executes_repo_client_with_middleware() -> git_cognition_core::CognitionResult<()>
{
    run_async_test(async {
        let transport = bitbucket()
            .body(r#"{"full_name":"akira-io/git-cognition-rs","is_private":false}"#)
            .record();
        let repository = cognition()
            .provider(bitbucket())
            .middleware(transport.clone())
            .header("x-request-id", "request-1")
            .auth(auth().oauth("bitbucket-token"))
            .repos()
            .get(repo().owner("akira-io").name("git-cognition-rs").get())
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
fn bitbucket_facade_executes_client_with_configured_base_url()
-> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        let transport = bitbucket()
            .body(r#"{"full_name":"akira-io/git-cognition-rs","is_private":false}"#)
            .record();

        cognition()
            .provider(bitbucket().base_url("https://bitbucket.internal.example/rest"))
            .transport(transport.clone())
            .repos()
            .get(repo().owner("akira-io").name("git-cognition-rs").get())
            .await?;

        assert_eq!(
            transport.requests()[0].url().value(),
            "https://bitbucket.internal.example/rest/repositories/akira-io/git-cognition-rs"
        );

        Ok(())
    })
}

#[test]
fn bitbucket_facade_executes_repo_client_with_retry() -> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        let recording_transport = bitbucket()
            .responses()
            .status(429)
            .body(r#"{"full_name":"akira-io/git-cognition-rs","is_private":false}"#)
            .record();
        let repository = cognition()
            .provider(bitbucket())
            .retry(recording_transport.clone())
            .attempts(2)
            .on_status(429)
            .repos()
            .get(repo().owner("akira-io").name("git-cognition-rs").get())
            .await?;

        assert_eq!(repository.repo().owner().as_str(), "akira-io");
        assert_eq!(recording_transport.requests().len(), 2);

        Ok(())
    })
}

#[test]
fn bitbucket_facade_observes_rate_limit_headers() -> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        let recorder = rate_limit().recorder();
        let recording_transport = bitbucket()
            .header("x-ratelimit-remaining", "40")
            .header("x-ratelimit-reset", "1710000002")
            .header("retry-after", "32")
            .body(r#"{"full_name":"akira-io/git-cognition-rs","is_private":false}"#)
            .record();
        let repository = cognition()
            .provider(bitbucket())
            .rate_limit(recording_transport)
            .remaining(["x-ratelimit-remaining"])
            .reset_at(["x-ratelimit-reset"])
            .retry_after(["retry-after"])
            .recorder(recorder.clone())
            .repos()
            .get(repo().owner("akira-io").name("git-cognition-rs").get())
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

fn repository() -> git_cognition_core::ManagedRepo<BitbucketProvider> {
    cognition()
        .provider(bitbucket())
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .get()
}
