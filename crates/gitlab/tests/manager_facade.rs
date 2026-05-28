use git_cognition_core::{RequestMethod, auth, cognition, rate_limit, repo, run_async_test};
use git_cognition_gitlab::{GitLabProvider, gitlab};

#[test]
fn gitlab_facade_builds_repo_requests() {
    let repository = cognition()
        .provider(gitlab())
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .get();

    assert_eq!(
        repository.url().value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fgit-cognition-rs"
    );
}

#[test]
fn gitlab_facade_uses_configured_base_url() {
    let repository = cognition()
        .provider(gitlab().base_url("https://gitlab.internal.example"))
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .get();

    assert_eq!(
        repository.url().value(),
        "https://gitlab.internal.example/api/v4/projects/akira-io%2Fgit-cognition-rs"
    );
}

#[test]
fn gitlab_facade_builds_issue_requests() {
    let issue = cognition()
        .provider(gitlab())
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .issue("42")
        .get();

    assert_eq!(
        issue.url().value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fgit-cognition-rs/issues/42"
    );
}

#[test]
fn gitlab_facade_builds_code_review_requests() {
    let code_review = cognition()
        .provider(gitlab())
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .code_review("42")
        .get();

    assert_eq!(
        code_review.url().value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fgit-cognition-rs/merge_requests/42"
    );
    assert_eq!(code_review.merge().method(), &RequestMethod::Put);
    assert_eq!(
        code_review.merge().url().value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fgit-cognition-rs/merge_requests/42/merge"
    );
}

#[test]
fn gitlab_facade_builds_release_requests() {
    let release = cognition()
        .provider(gitlab())
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .release("v1.0.0")
        .get();

    assert_eq!(
        release.url().value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fgit-cognition-rs/releases/v1.0.0"
    );
}

#[test]
fn gitlab_facade_builds_pipeline_requests() -> git_cognition_core::CognitionResult<()> {
    let pipeline = cognition()
        .provider(gitlab())
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .pipeline("42")
        .get();
    let pipelines = cognition()
        .provider(gitlab())
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
        "https://gitlab.com/api/v4/projects/akira-io%2Fgit-cognition-rs/pipelines/42"
    );
    assert_eq!(
        pipelines.value(),
        "https://gitlab.com/api/v4/projects/akira-io%2Fgit-cognition-rs/pipelines?per_page=50&page=2"
    );
    assert_eq!(pipeline.rerun()?.method(), &RequestMethod::Post);
    assert_eq!(pipeline.cancel()?.method(), &RequestMethod::Post);

    Ok(())
}

#[test]
fn gitlab_facade_builds_mutation_requests() {
    let create_request = cognition()
        .provider(gitlab())
        .repo()
        .draft(repository())
        .visibility(git_cognition_core::Visibility::Private)
        .create();

    assert_eq!(create_request.method(), &RequestMethod::Post);
}

#[test]
fn gitlab_facade_executes_repo_client_with_auth() -> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        let transport = gitlab()
            .body(r#"{"path_with_namespace":"akira-io/git-cognition-rs","visibility":"public"}"#)
            .record();
        let repository = cognition()
            .provider(gitlab())
            .transport(transport.clone())
            .auth(auth().personal_access_token("gitlab-token"))
            .repos()
            .get(repo().owner("akira-io").name("git-cognition-rs").get())
            .await?;
        let requests = transport.requests();
        let auth_header = requests[0]
            .headers()
            .iter()
            .find(|header| header.name().as_str() == "private-token");

        assert_eq!(repository.repo().owner().as_str(), "akira-io");
        assert_eq!(
            requests[0].url().value(),
            "https://gitlab.com/api/v4/projects/akira-io%2Fgit-cognition-rs"
        );
        assert_eq!(
            auth_header.map(|header| header.value().as_str()),
            Some("gitlab-token")
        );

        Ok(())
    })
}

#[test]
fn gitlab_facade_executes_repo_client_with_middleware() -> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        let transport = gitlab()
            .body(r#"{"path_with_namespace":"akira-io/git-cognition-rs","visibility":"public"}"#)
            .record();
        let repository = cognition()
            .provider(gitlab())
            .middleware(transport.clone())
            .header("x-request-id", "request-1")
            .auth(auth().personal_access_token("gitlab-token"))
            .repos()
            .get(repo().owner("akira-io").name("git-cognition-rs").get())
            .await?;
        let requests = transport.requests();
        let request_headers = requests[0].headers();

        assert_eq!(repository.repo().owner().as_str(), "akira-io");
        assert!(request_headers.iter().any(|header| {
            header.name().as_str() == "private-token" && header.value().as_str() == "gitlab-token"
        }));
        assert!(request_headers.iter().any(|header| {
            header.name().as_str() == "x-request-id" && header.value().as_str() == "request-1"
        }));

        Ok(())
    })
}

#[test]
fn gitlab_facade_executes_client_with_configured_base_url()
-> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        let transport = gitlab()
            .body(r#"{"path_with_namespace":"akira-io/git-cognition-rs","visibility":"public"}"#)
            .record();

        cognition()
            .provider(gitlab().base_url("https://gitlab.internal.example"))
            .transport(transport.clone())
            .repos()
            .get(repo().owner("akira-io").name("git-cognition-rs").get())
            .await?;

        assert_eq!(
            transport.requests()[0].url().value(),
            "https://gitlab.internal.example/api/v4/projects/akira-io%2Fgit-cognition-rs"
        );

        Ok(())
    })
}

#[test]
fn gitlab_facade_executes_repo_client_with_retry() -> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        let recording_transport = gitlab()
            .responses()
            .status(503)
            .body(r#"{"path_with_namespace":"akira-io/git-cognition-rs","visibility":"public"}"#)
            .record();
        let repository = cognition()
            .provider(gitlab())
            .retry(recording_transport.clone())
            .attempts(2)
            .on_status(503)
            .repos()
            .get(repo().owner("akira-io").name("git-cognition-rs").get())
            .await?;

        assert_eq!(repository.repo().owner().as_str(), "akira-io");
        assert_eq!(recording_transport.requests().len(), 2);

        Ok(())
    })
}

#[test]
fn gitlab_facade_observes_rate_limit_headers() -> git_cognition_core::CognitionResult<()> {
    run_async_test(async {
        let recorder = rate_limit().recorder();
        let recording_transport = gitlab()
            .header("ratelimit-remaining", "41")
            .header("ratelimit-reset", "1710000001")
            .header("retry-after", "31")
            .header("ratelimit-observed", "8")
            .body(r#"{"path_with_namespace":"akira-io/git-cognition-rs","visibility":"public"}"#)
            .record();
        let repository = cognition()
            .provider(gitlab())
            .rate_limit(recording_transport)
            .remaining(["ratelimit-remaining"])
            .reset_at(["ratelimit-reset"])
            .retry_after(["retry-after"])
            .cost(["ratelimit-observed"])
            .recorder(recorder.clone())
            .repos()
            .get(repo().owner("akira-io").name("git-cognition-rs").get())
            .await?;
        let observations = recorder.observations();

        assert_eq!(repository.repo().owner().as_str(), "akira-io");
        assert_eq!(observations.len(), 1);
        assert_eq!(
            observations[0].remaining().map(|quota| quota.as_u64()),
            Some(41)
        );
        assert_eq!(observations[0].cost().map(|cost| cost.as_u64()), Some(8));

        Ok(())
    })
}

fn repository() -> git_cognition_core::ManagedRepo<GitLabProvider> {
    cognition()
        .provider(gitlab())
        .repo()
        .owner("akira-io")
        .name("git-cognition-rs")
        .get()
}
