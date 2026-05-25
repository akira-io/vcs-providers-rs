use vcs_provider_core::{
    CodeReviews, IssueId, Issues, Pipelines, ReleaseId, Releases,
    TransportNotConfiguredCodeReviews, TransportNotConfiguredIssues,
    TransportNotConfiguredPipelines, TransportNotConfiguredReleases, VcsError, VcsResult,
    code_review, issue, pipeline, release, repo,
};

#[test]
fn issue_contract_reports_unconfigured_transport() -> VcsResult<()> {
    let repo = repo().owner("akira-io").name("vcs-providers-rs").get();
    let issue_resource = issue().repo(repo.clone()).id("1").get();
    let list_query = issue().query().list(repo.clone(), None);
    let result =
        futures::executor::block_on(TransportNotConfiguredIssues.get(repo, IssueId::make("1")));
    let list_result = futures::executor::block_on(TransportNotConfiguredIssues.list(list_query));

    assert_eq!(issue_resource.id().as_str(), "1");
    assert_eq!(result, Err(VcsError::TransportNotConfigured));
    assert_eq!(list_result, Err(VcsError::TransportNotConfigured));

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
    let list_query = code_review().query().list(repo.clone(), None);
    let code_review = code_review().repo(repo.clone()).id("1").get();

    assert_eq!(
        futures::executor::block_on(TransportNotConfiguredCodeReviews.create(draft)),
        Err(VcsError::TransportNotConfigured)
    );
    assert_eq!(
        futures::executor::block_on(TransportNotConfiguredCodeReviews.merge(code_review)),
        Err(VcsError::TransportNotConfigured)
    );
    assert_eq!(
        futures::executor::block_on(TransportNotConfiguredCodeReviews.list(list_query)),
        Err(VcsError::TransportNotConfigured)
    );

    Ok(())
}

#[test]
fn pipeline_contract_reports_unconfigured_transport() -> VcsResult<()> {
    let repo = repo().owner("akira-io").name("vcs-providers-rs").get();
    let list_query = pipeline().query().list(repo.clone(), None);
    let pipeline = pipeline().repo(repo).id("1").get();
    let result = futures::executor::block_on(TransportNotConfiguredPipelines.cancel(pipeline));
    let list_result = futures::executor::block_on(TransportNotConfiguredPipelines.list(list_query));

    assert_eq!(result, Err(VcsError::TransportNotConfigured));
    assert_eq!(list_result, Err(VcsError::TransportNotConfigured));

    Ok(())
}

#[test]
fn release_contract_reports_unconfigured_transport() -> VcsResult<()> {
    let repo = repo().owner("akira-io").name("vcs-providers-rs").get();
    let release_resource = release().repo(repo.clone()).id("1").get();
    let list_query = release().query().list(repo.clone(), None);
    let result =
        futures::executor::block_on(TransportNotConfiguredReleases.get(repo, ReleaseId::make("1")));
    let list_result = futures::executor::block_on(TransportNotConfiguredReleases.list(list_query));

    assert_eq!(release_resource.id().as_str(), "1");
    assert_eq!(result, Err(VcsError::TransportNotConfigured));
    assert_eq!(list_result, Err(VcsError::TransportNotConfigured));

    Ok(())
}
