use vcs_provider_core::{VcsResult, run_async_test};
use vcs_provider_gitlab::gitlab;

#[test]
fn gitlab_authentication_validate_targets_authenticated_user_endpoint() -> VcsResult<()> {
    run_async_test(async {
        gitlab().status(200).authentication().validate().await?;

        Ok(())
    })
}

#[test]
fn gitlab_client_hydrates_authenticated_user_groups() -> VcsResult<()> {
    run_async_test(async {
        let organizations = gitlab()
            .body(r#"[{"id":42,"full_path":"akira-io"}]"#)
            .organizations()
            .list()
            .await?;

        assert_eq!(organizations.items().len(), 1);
        assert_eq!(organizations.items()[0].provider().as_str(), "gitlab");
        assert_eq!(organizations.items()[0].id(), "42");
        assert_eq!(organizations.items()[0].login(), "akira-io");

        Ok(())
    })
}
