use vcs_provider_bitbucket::bitbucket;
use vcs_provider_core::{VcsResult, run_async_test};

#[test]
fn bitbucket_authentication_validate_targets_authenticated_user_endpoint() -> VcsResult<()> {
    run_async_test(async {
        bitbucket().status(200).authentication().validate().await?;

        Ok(())
    })
}

#[test]
fn bitbucket_client_hydrates_authenticated_user_workspaces() -> VcsResult<()> {
    run_async_test(async {
        let organizations = bitbucket()
            .body(r#"{"values":[{"uuid":"{workspace-uuid}","slug":"akira-io"}]}"#)
            .organizations()
            .list()
            .await?;

        assert_eq!(organizations.items().len(), 1);
        assert_eq!(organizations.items()[0].provider().as_str(), "bitbucket");
        assert_eq!(organizations.items()[0].id(), "{workspace-uuid}");
        assert_eq!(organizations.items()[0].login(), "akira-io");

        Ok(())
    })
}
