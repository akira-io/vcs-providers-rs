use vcs_provider_core::{OrganizationKind, auth, run_async_test, vcs};
use vcs_provider_github::github;

#[test]
fn github_authentication_validate_targets_authenticated_user_endpoint()
-> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let transport = github().body(r#"{"login":"octocat"}"#).record();

        vcs(github())
            .transport(transport.clone())
            .auth(auth().personal_access_token("github-token"))
            .authentication()
            .validate()
            .await?;

        let requests = transport.requests();

        assert_eq!(requests[0].url().value(), "https://api.github.com/user");
        assert_eq!(
            requests[0]
                .headers()
                .iter()
                .find(|header| header.name().as_str() == "authorization")
                .map(|header| header.value().as_str()),
            Some("Bearer github-token")
        );

        Ok(())
    })
}

#[test]
fn github_client_hydrates_authenticated_user_organizations() -> vcs_provider_core::VcsResult<()> {
    run_async_test(async {
        let organizations = github()
            .body(r#"[{"id":1,"login":"akira-io"}]"#)
            .organizations()
            .list()
            .await?;

        assert_eq!(organizations.items().len(), 1);
        assert_eq!(organizations.items()[0].provider().as_str(), "github");
        assert_eq!(organizations.items()[0].id(), "1");
        assert_eq!(organizations.items()[0].login(), "akira-io");
        assert_eq!(
            organizations.items()[0].kind(),
            &OrganizationKind::Organization
        );

        Ok(())
    })
}
