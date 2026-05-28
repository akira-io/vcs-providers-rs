use vcs_provider_core::{
    Authentication, CodeReviews, CodeReviewsFluent, Issues, IssuesFluent, Organizations, Pipelines,
    Releases, ReleasesFluent, Repos, TransportNotConfiguredAuthentication,
    TransportNotConfiguredCodeReviews, TransportNotConfiguredIssues,
    TransportNotConfiguredOrganizations, TransportNotConfiguredPipelines,
    TransportNotConfiguredReleases, TransportNotConfiguredRepos, UnsupportedIssues,
    UnsupportedReleases, VcsError, VcsResult, Visibility, code_review, issue, issue_id, pipeline,
    release, release_id, repo, run_async_test,
};

#[test]
fn authentication_contract_reports_unconfigured_transport() -> VcsResult<()> {
    assert_eq!(
        run_async_test(TransportNotConfiguredAuthentication.validate()),
        Err(VcsError::TransportNotConfigured)
    );

    Ok(())
}

#[test]
fn organization_contract_reports_unconfigured_transport() -> VcsResult<()> {
    assert_eq!(
        run_async_test(TransportNotConfiguredOrganizations.list()),
        Err(VcsError::TransportNotConfigured)
    );

    Ok(())
}

#[test]
fn repo_contract_reports_unconfigured_transport() -> VcsResult<()> {
    let repo = repo().owner("akira-io").name("vcs-providers-rs").get();
    let draft = repo.clone().draft().visibility(Visibility::Private).get();
    let patch = repo.clone().patch().visibility(Visibility::Public).get();
    let branch_draft = vcs_provider_core::BranchDraft::make(repo.clone(), "feature", "abc123");

    assert_eq!(
        run_async_test(TransportNotConfiguredRepos.create(draft)),
        Err(VcsError::TransportNotConfigured)
    );
    assert_eq!(
        run_async_test(TransportNotConfiguredRepos.update(patch)),
        Err(VcsError::TransportNotConfigured)
    );
    assert_eq!(
        run_async_test(TransportNotConfiguredRepos.delete(repo.clone())),
        Err(VcsError::TransportNotConfigured)
    );
    assert_eq!(
        run_async_test(TransportNotConfiguredRepos.create_branch(branch_draft)),
        Err(VcsError::TransportNotConfigured)
    );
    assert_eq!(
        run_async_test(TransportNotConfiguredRepos.delete_branch(repo, "feature".into())),
        Err(VcsError::TransportNotConfigured)
    );

    Ok(())
}

#[test]
fn issue_contract_reports_unconfigured_transport() -> VcsResult<()> {
    let repo = repo().owner("akira-io").name("vcs-providers-rs").get();
    let issue_resource = issue().repo(repo.clone()).id("1").get();
    let result = run_async_test(TransportNotConfiguredIssues.get(repo.clone(), issue_id("1")));
    let list_result = run_async_test(
        (Box::new(TransportNotConfiguredIssues) as Box<dyn Issues>)
            .list()
            .location(repo)
            .list(),
    );
    let delete_result = run_async_test(TransportNotConfiguredIssues.delete(issue_resource.clone()));

    assert_eq!(issue_resource.id().as_str(), "1");
    assert_eq!(result, Err(VcsError::TransportNotConfigured));
    assert_eq!(list_result, Err(VcsError::TransportNotConfigured));
    assert_eq!(delete_result, Err(VcsError::TransportNotConfigured));

    Ok(())
}

#[test]
fn unsupported_issue_contract_reports_unsupported_operation() -> VcsResult<()> {
    let repo = repo().owner("akira-io").name("vcs-providers-rs").get();
    let issue_resource = issue().repo(repo.clone()).id("1").get();
    let get_result = run_async_test(UnsupportedIssues.get(repo.clone(), issue_id("1")));
    let list_result = run_async_test(
        (Box::new(UnsupportedIssues) as Box<dyn Issues>)
            .list()
            .location(repo)
            .list(),
    );
    let delete_result = run_async_test(UnsupportedIssues.delete(issue_resource));

    assert!(matches!(
        get_result,
        Err(VcsError::UnsupportedOperation(operation)) if operation == "issue get"
    ));
    assert!(matches!(
        list_result,
        Err(VcsError::UnsupportedOperation(operation)) if operation == "issue list"
    ));
    assert!(matches!(
        delete_result,
        Err(VcsError::UnsupportedOperation(operation)) if operation == "issue delete"
    ));

    Ok(())
}

#[test]
fn code_review_contract_reports_unconfigured_transport() -> VcsResult<()> {
    let repo = repo().owner("akira-io").name("vcs-providers-rs").get();
    let draft = code_review()
        .draft()
        .repo(repo.clone())
        .title("Add provider contract")
        .get();
    let code_review = code_review().repo(repo.clone()).id("1").get();

    assert_eq!(
        run_async_test(TransportNotConfiguredCodeReviews.create(draft)),
        Err(VcsError::TransportNotConfigured)
    );
    assert_eq!(
        run_async_test(TransportNotConfiguredCodeReviews.merge(code_review.clone())),
        Err(VcsError::TransportNotConfigured)
    );
    assert_eq!(
        run_async_test(TransportNotConfiguredCodeReviews.delete(code_review.clone())),
        Err(VcsError::TransportNotConfigured)
    );
    assert_eq!(
        run_async_test(
            (Box::new(TransportNotConfiguredCodeReviews) as Box<dyn CodeReviews>)
                .list()
                .location(repo)
                .list()
        ),
        Err(VcsError::TransportNotConfigured)
    );

    Ok(())
}

#[test]
fn pipeline_contract_reports_unconfigured_transport() -> VcsResult<()> {
    let repo = repo().owner("akira-io").name("vcs-providers-rs").get();
    let list_query = pipeline().query().location(repo.clone()).list();
    let pipeline = pipeline().repo(repo).id("1").get();
    let result = run_async_test(TransportNotConfiguredPipelines.cancel(pipeline));
    let list_result = run_async_test(TransportNotConfiguredPipelines.list(list_query));

    assert_eq!(result, Err(VcsError::TransportNotConfigured));
    assert_eq!(list_result, Err(VcsError::TransportNotConfigured));

    Ok(())
}

#[test]
fn release_contract_reports_unconfigured_transport() -> VcsResult<()> {
    let repo = repo().owner("akira-io").name("vcs-providers-rs").get();
    let release_resource = release().repo(repo.clone()).id("1").get();
    let result = run_async_test(TransportNotConfiguredReleases.get(repo.clone(), release_id("1")));
    let list_result = run_async_test(
        (Box::new(TransportNotConfiguredReleases) as Box<dyn Releases>)
            .list()
            .location(repo)
            .list(),
    );

    assert_eq!(release_resource.id().as_str(), "1");
    assert_eq!(result, Err(VcsError::TransportNotConfigured));
    assert_eq!(list_result, Err(VcsError::TransportNotConfigured));

    Ok(())
}

#[test]
fn unsupported_release_contract_reports_unsupported_operation() -> VcsResult<()> {
    let repo = repo().owner("akira-io").name("vcs-providers-rs").get();
    let release_resource = release().repo(repo.clone()).id("1").get();
    let get_result = run_async_test(UnsupportedReleases.get(repo.clone(), release_id("1")));
    let list_result = run_async_test(
        (Box::new(UnsupportedReleases) as Box<dyn Releases>)
            .list()
            .location(repo)
            .list(),
    );
    let delete_result = run_async_test(UnsupportedReleases.delete(release_resource));

    assert!(matches!(
        get_result,
        Err(VcsError::UnsupportedOperation(operation)) if operation == "release get"
    ));
    assert!(matches!(
        list_result,
        Err(VcsError::UnsupportedOperation(operation)) if operation == "release list"
    ));
    assert!(matches!(
        delete_result,
        Err(VcsError::UnsupportedOperation(operation)) if operation == "release delete"
    ));

    Ok(())
}
