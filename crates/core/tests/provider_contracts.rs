use vcs_provider_core::{
    CodeReview, CodeReviewDraft, CodeReviewId, CodeReviews, IssueId, Issues, Pipeline, PipelineId,
    Pipelines, ReleaseId, Releases, TransportNotConfiguredCodeReviews,
    TransportNotConfiguredIssues, TransportNotConfiguredPipelines, TransportNotConfiguredReleases,
    VcsError, VcsResult, repo,
};

#[test]
fn issue_contract_reports_unconfigured_transport() -> VcsResult<()> {
    let repo = repo().owner("akira-io").name("vcs-providers-rs").build();
    let result =
        futures::executor::block_on(TransportNotConfiguredIssues.get(repo, IssueId::make("1")));

    assert_eq!(result, Err(VcsError::TransportNotConfigured));

    Ok(())
}

#[test]
fn code_review_contract_reports_unconfigured_transport() -> VcsResult<()> {
    let repo = repo().owner("akira-io").name("vcs-providers-rs").build();
    let draft = CodeReviewDraft::make(repo.clone(), "Add provider contract");
    let code_review = CodeReview::make(repo, CodeReviewId::make("1"));

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
    let pipeline = Pipeline::make(repo, PipelineId::make("1"));
    let result = futures::executor::block_on(TransportNotConfiguredPipelines.cancel(pipeline));

    assert_eq!(result, Err(VcsError::TransportNotConfigured));

    Ok(())
}

#[test]
fn release_contract_reports_unconfigured_transport() -> VcsResult<()> {
    let repo = repo().owner("akira-io").name("vcs-providers-rs").build();
    let result =
        futures::executor::block_on(TransportNotConfiguredReleases.get(repo, ReleaseId::make("1")));

    assert_eq!(result, Err(VcsError::TransportNotConfigured));

    Ok(())
}
