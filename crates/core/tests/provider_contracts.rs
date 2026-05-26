use vcs_provider_core::{
    CodeReviews, CodeReviewsFluent, IssueId, Issues, IssuesFluent, Pipelines, ReleaseId, Releases,
    ReleasesFluent, Repos, TransportNotConfiguredCodeReviews, TransportNotConfiguredIssues,
    TransportNotConfiguredPipelines, TransportNotConfiguredReleases, TransportNotConfiguredRepos,
    VcsError, VcsResult, Visibility, code_review, issue, pipeline, release, repo,
};

#[test]
fn repo_contract_reports_unconfigured_transport() -> VcsResult<()> {
    let repo = repo().owner("akira-io").name("vcs-providers-rs").get();
    let draft = vcs_provider_core::RepositoryDraftBuilder::make(repo.clone())
        .visibility(Visibility::Private)
        .get();
    let patch = vcs_provider_core::RepositoryPatchBuilder::make(repo.clone())
        .visibility(Visibility::Public)
        .get();

    assert_eq!(
        futures::executor::block_on(TransportNotConfiguredRepos.create(draft)),
        Err(VcsError::TransportNotConfigured)
    );
    assert_eq!(
        futures::executor::block_on(TransportNotConfiguredRepos.update(patch)),
        Err(VcsError::TransportNotConfigured)
    );
    assert_eq!(
        futures::executor::block_on(TransportNotConfiguredRepos.delete(repo)),
        Err(VcsError::TransportNotConfigured)
    );

    Ok(())
}

#[test]
fn issue_contract_reports_unconfigured_transport() -> VcsResult<()> {
    let repo = repo().owner("akira-io").name("vcs-providers-rs").get();
    let issue_resource = issue().repo(repo.clone()).id("1").get();
    let result = futures::executor::block_on(
        TransportNotConfiguredIssues.get(repo.clone(), IssueId::make("1")),
    );
    let list_result = futures::executor::block_on(
        (Box::new(TransportNotConfiguredIssues) as Box<dyn Issues>)
            .list()
            .location(repo)
            .list(),
    );

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
        futures::executor::block_on(
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
    let result = futures::executor::block_on(
        TransportNotConfiguredReleases.get(repo.clone(), ReleaseId::make("1")),
    );
    let list_result = futures::executor::block_on(
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
