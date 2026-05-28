use git_cognition_core::{RequestMethod, auth, cognition, rate_limit, repo, run_async_test};
use git_cognition_github::{GitHubProvider, github};

#[test]
fn github_facade_builds_repo_requests() {
    let repository = cognition()
        .provider(github())
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .get();

    assert_eq!(
        repository.url().value(),
        "https://api.github.com/repos/akira-io/git-cognition-rs"
    );
}

#[test]
fn github_facade_uses_configured_base_url() {
    let repository = cognition()
        .provider(github().base_url("https://github.enterprise.test/api/v3"))
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .get();

    assert_eq!(
        repository.url().value(),
        "https://github.enterprise.test/api/v3/repos/akira-io/git-cognition-rs"
    );
}

#[test]
fn github_facade_builds_issue_requests() {
    let issue = cognition()
        .provider(github())
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .issue("42")
        .get();

    assert_eq!(
        issue.url().value(),
        "https://api.github.com/repos/akira-io/git-cognition-rs/issues/42"
    );
}

#[test]
fn github_facade_builds_code_review_requests() {
    let code_review = cognition()
        .provider(github())
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .code_review("42")
        .get();

    assert_eq!(
        code_review.url().value(),
        "https://api.github.com/repos/akira-io/git-cognition-rs/pulls/42"
    );
    assert_eq!(code_review.merge().method(), &RequestMethod::Put);
    assert_eq!(
        code_review.merge().url().value(),
        "https://api.github.com/repos/akira-io/git-cognition-rs/pulls/42/merge"
    );
}

#[test]
fn github_facade_builds_release_requests() {
    let release = cognition()
        .provider(github())
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .release("123")
        .get();

    assert_eq!(
        release.url().value(),
        "https://api.github.com/repos/akira-io/git-cognition-rs/releases/123"
    );
}

#[test]
fn github_facade_builds_pipeline_requests() -> git_cognition_core::CognitionResult<()> {
    let pipeline = cognition()
        .provider(github())
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .pipeline("42")
        .get();
    let pipelines = cognition()
        .provider(github())
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
        "https://api.github.com/repos/akira-io/git-cognition-rs/actions/runs/42"
    );
    assert_eq!(
        pipelines.value(),
        "https://api.github.com/repos/akira-io/git-cognition-rs/actions/runs?per_page=50&page=2"
    );
    assert_eq!(pipeline.rerun()?.method(), &RequestMethod::Post);
    assert_eq!(pipeline.cancel()?.method(), &RequestMethod::Post);

    Ok(())
}

#[test]
fn github_facade_builds_mutation_requests() {
    let create_request = cognition()
        .provider(github())
        .repo()
        .draft(repository())
        .visibility(git_cognition_core::Visibility::Private)
        .create();

    assert_eq!(create_request.method(), &RequestMethod::Post);
}

#[test]
fn github_facade_executes_repo_client_with_auth() -> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        let transport = github()
            .body(r#"{"full_name":"akira-io/git-cognition-rs","private":false}"#)
            .record();
        let repository = cognition()
            .provider(github())
            .transport(transport.clone())
            .auth(auth().personal_access_token("github-token"))
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
            "https://api.github.com/repos/akira-io/git-cognition-rs"
        );
        assert_eq!(
            auth_header.map(|header| header.value().as_str()),
            Some("Bearer github-token")
        );

        Ok(())
    })
}

#[test]
fn github_facade_executes_repo_client_with_middleware() -> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        let transport = github()
            .body(r#"{"full_name":"akira-io/git-cognition-rs","private":false}"#)
            .record();
        let repository = cognition()
            .provider(github())
            .middleware(transport.clone())
            .header("x-request-id", "request-1")
            .auth(auth().personal_access_token("github-token"))
            .repos()
            .get(repo().owner("akira-io").name("git-cognition-rs").get())
            .await?;
        let requests = transport.requests();
        let request_headers = requests[0].headers();

        assert_eq!(repository.repo().owner().as_str(), "akira-io");
        assert!(request_headers.iter().any(|header| {
            header.name().as_str() == "authorization"
                && header.value().as_str() == "Bearer github-token"
        }));
        assert!(request_headers.iter().any(|header| {
            header.name().as_str() == "x-request-id" && header.value().as_str() == "request-1"
        }));

        Ok(())
    })
}

#[test]
fn github_facade_executes_client_with_configured_base_url()
-> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        let transport = github()
            .body(r#"{"full_name":"akira-io/git-cognition-rs","private":false}"#)
            .record();

        cognition()
            .provider(github().base_url("https://github.enterprise.test/api/v3"))
            .transport(transport.clone())
            .repos()
            .get(repo().owner("akira-io").name("git-cognition-rs").get())
            .await?;

        assert_eq!(
            transport.requests()[0].url().value(),
            "https://github.enterprise.test/api/v3/repos/akira-io/git-cognition-rs"
        );

        Ok(())
    })
}

#[test]
fn github_facade_executes_repo_client_with_retry() -> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        let recording_transport = github()
            .responses()
            .status(500)
            .body(r#"{"full_name":"akira-io/git-cognition-rs","private":false}"#)
            .record();
        let repository = cognition()
            .provider(github())
            .retry(recording_transport.clone())
            .attempts(2)
            .on_status(500)
            .repos()
            .get(repo().owner("akira-io").name("git-cognition-rs").get())
            .await?;

        assert_eq!(repository.repo().owner().as_str(), "akira-io");
        assert_eq!(recording_transport.requests().len(), 2);

        Ok(())
    })
}

#[test]
fn github_facade_observes_rate_limit_headers() -> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        let recorder = rate_limit().recorder();
        let recording_transport = github()
            .header("x-ratelimit-remaining", "42")
            .header("x-ratelimit-reset", "1710000000")
            .header("retry-after", "30")
            .header("x-ratelimit-used", "7")
            .body(r#"{"full_name":"akira-io/git-cognition-rs","private":false}"#)
            .record();
        let repository = cognition()
            .provider(github())
            .rate_limit(recording_transport)
            .remaining(["x-ratelimit-remaining"])
            .reset_at(["x-ratelimit-reset"])
            .retry_after(["retry-after"])
            .cost(["x-ratelimit-used"])
            .recorder(recorder.clone())
            .repos()
            .get(repo().owner("akira-io").name("git-cognition-rs").get())
            .await?;
        let observations = recorder.observations();

        assert_eq!(repository.repo().owner().as_str(), "akira-io");
        assert_eq!(observations.len(), 1);
        assert_eq!(
            observations[0].remaining().map(|quota| quota.as_u64()),
            Some(42)
        );
        assert_eq!(observations[0].cost().map(|cost| cost.as_u64()), Some(7));

        Ok(())
    })
}

fn repository() -> git_cognition_core::ManagedRepo<GitHubProvider> {
    cognition()
        .provider(github())
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .get()
}
