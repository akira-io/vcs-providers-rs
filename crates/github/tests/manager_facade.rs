use vcs_provider_core::{
    RequestMethod, auth, provider_response, provider_responses, repo, run_async_test, vcs,
};
use vcs_provider_github::{GitHubProvider, github};

#[test]
fn github_facade_builds_repo_requests() {
    let repository = vcs(github())
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();

    assert_eq!(
        repository.url().value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs"
    );
}

#[test]
fn github_facade_uses_configured_base_url() {
    let repository = vcs(github().base_url("https://github.enterprise.test/api/v3"))
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get();

    assert_eq!(
        repository.url().value(),
        "https://github.enterprise.test/api/v3/repos/akira-io/vcs-providers-rs"
    );
}

#[test]
fn github_facade_builds_issue_requests() {
    let issue = vcs(github())
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .issue("42")
        .get();

    assert_eq!(
        issue.url().value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/issues/42"
    );
}

#[test]
fn github_facade_builds_code_review_requests() {
    let code_review = vcs(github())
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .code_review("42")
        .get();

    assert_eq!(
        code_review.url().value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/pulls/42"
    );
    assert_eq!(code_review.merge().method(), &RequestMethod::Put);
    assert_eq!(
        code_review.merge().url().value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/pulls/42/merge"
    );
}

#[test]
fn github_facade_builds_release_requests() {
    let release = vcs(github())
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .release("123")
        .get();

    assert_eq!(
        release.url().value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/releases/123"
    );
}

#[test]
fn github_facade_builds_pipeline_requests() -> vcs_provider_core::VcsResult<()> {
    let pipeline = vcs(github())
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .pipeline("42")
        .get();
    let pipelines = vcs(github())
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
        "https://api.github.com/repos/akira-io/vcs-providers-rs/actions/runs/42"
    );
    assert_eq!(
        pipelines.value(),
        "https://api.github.com/repos/akira-io/vcs-providers-rs/actions/runs?per_page=50&page=2"
    );
    assert_eq!(pipeline.rerun()?.method(), &RequestMethod::Post);
    assert_eq!(pipeline.cancel()?.method(), &RequestMethod::Post);

    Ok(())
}

#[test]
fn github_facade_builds_mutation_requests() {
    let create_request = vcs(github())
        .repo()
        .draft(repository())
        .visibility(vcs_provider_core::Visibility::Private)
        .create();

    assert_eq!(create_request.method(), &RequestMethod::Post);
}

#[test]
fn github_facade_executes_repo_client_with_auth() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let transport = provider_response()
            .body(r#"{"full_name":"akira-io/vcs-providers-rs","private":false}"#)
            .record();
        let repository = vcs(github())
            .transport(transport.clone())
            .auth(auth().personal_access_token("github-token"))
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
            "https://api.github.com/repos/akira-io/vcs-providers-rs"
        );
        assert_eq!(
            auth_header.map(|header| header.value().as_str()),
            Some("Bearer github-token")
        );

        Ok(())
    })
}

#[test]
fn github_facade_executes_client_with_configured_base_url() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let transport = provider_response()
            .body(r#"{"full_name":"akira-io/vcs-providers-rs","private":false}"#)
            .record();

        vcs(github().base_url("https://github.enterprise.test/api/v3"))
            .transport(transport.clone())
            .repos()
            .get(repo().owner("akira-io").name("vcs-providers-rs").get())
            .await?;

        assert_eq!(
            transport.requests()[0].url().value(),
            "https://github.enterprise.test/api/v3/repos/akira-io/vcs-providers-rs"
        );

        Ok(())
    })
}

#[test]
fn github_facade_executes_repo_client_with_retry() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let provider_transport = provider_responses()
            .status(500)
            .body(r#"{"full_name":"akira-io/vcs-providers-rs","private":false}"#)
            .record();
        let repository = vcs(github())
            .retry(provider_transport.clone())
            .attempts(2)
            .on_status(500)
            .repos()
            .get(repo().owner("akira-io").name("vcs-providers-rs").get())
            .await?;

        assert_eq!(repository.repo().owner().as_str(), "akira-io");
        assert_eq!(provider_transport.requests().len(), 2);

        Ok(())
    })
}

fn repository() -> vcs_provider_core::ManagedRepo<GitHubProvider> {
    vcs(github())
        .repo()
        .owner("akira-io")
        .name("vcs-providers-rs")
        .get()
}
