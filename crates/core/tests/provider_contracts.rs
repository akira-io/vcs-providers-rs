use vcs_provider_core::{
    CodeReviews, IssueId, Issues, Pipelines, ReleaseId, Releases,
    TransportNotConfiguredCodeReviews, TransportNotConfiguredIssues,
    TransportNotConfiguredPipelines, TransportNotConfiguredReleases, VcsError, VcsResult,
    code_review, issue, pipeline, release, repo,
};

#[test]
fn issue_contract_reports_unconfigured_transport() -> VcsResult<()> {
    let repo = repo().owner("akira-io").name("vcs-providers-rs").build();
    let issue = issue().repo(repo.clone()).id("1").build();
    let result =
        futures::executor::block_on(TransportNotConfiguredIssues.get(repo, IssueId::make("1")));

    assert_eq!(issue.id().as_str(), "1");
    assert_eq!(result, Err(VcsError::TransportNotConfigured));

    Ok(())
}

#[test]
fn code_review_contract_reports_unconfigured_transport() -> VcsResult<()> {
    let repo = repo().owner("akira-io").name("vcs-providers-rs").build();
    let draft = code_review()
        .draft()
        .repo(repo.clone())
        .title("Add provider contract")
        .build();
    let code_review = code_review().repo(repo).id("1").build();

    assert_eq!(
        futures::executor::block_on(TransportNotConfiguredCodeReviews.create(draft)),
        Err(VcsError::TransportNotConfigured)
    );
    assert_eq!(
        futures::executor::block_on(TransportNotConfiguredCodeReviews.merge(code_review)),
        Err(VcsError::TransportNotConfigured)
    );

    Ok(())
}

#[test]
fn pipeline_contract_reports_unconfigured_transport() -> VcsResult<()> {
    let repo = repo().owner("akira-io").name("vcs-providers-rs").build();
    let pipeline = pipeline().repo(repo).id("1").build();
    let result = futures::executor::block_on(TransportNotConfiguredPipelines.cancel(pipeline));

    assert_eq!(result, Err(VcsError::TransportNotConfigured));

    Ok(())
}

#[test]
fn release_contract_reports_unconfigured_transport() -> VcsResult<()> {
    let repo = repo().owner("akira-io").name("vcs-providers-rs").build();
    let release = release().repo(repo.clone()).id("1").build();
    let result =
        futures::executor::block_on(TransportNotConfiguredReleases.get(repo, ReleaseId::make("1")));

    assert_eq!(release.id().as_str(), "1");
    assert_eq!(result, Err(VcsError::TransportNotConfigured));

    Ok(())
}
