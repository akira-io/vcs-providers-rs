use crate::{
    CodeReview, CodeReviewId, Issue, IssueId, Pipeline, PipelineId, Provider, Release, ReleaseId,
    Repo, VcsError, VcsResult, Visibility, code_review, error, issue, pipeline, release, repo,
};

pub fn check_provider_contracts(provider: &impl Provider) -> VcsResult<()> {
    check_repos(provider)?;
    check_issues(provider)?;
    check_code_reviews(provider)?;
    check_pipelines(provider)?;
    check_releases(provider)
}

fn check_repos(provider: &impl Provider) -> VcsResult<()> {
    let repo_location = sample_repo_location();
    let repos = provider.repos();

    assert_transport_not_configured(
        "repo get",
        futures::executor::block_on(repos.get(repo_location.clone())),
    )?;
    assert_transport_not_configured(
        "repo list",
        futures::executor::block_on(repos.list(repo().query().list(None))),
    )?;
    assert_transport_not_configured(
        "repo search",
        futures::executor::block_on(repos.search(repo().query().search("vcs", None))),
    )?;
    assert_transport_not_configured(
        "repo create",
        futures::executor::block_on(
            repos.create(
                repo_location
                    .clone()
                    .draft()
                    .visibility(Visibility::Private)
                    .get(),
            ),
        ),
    )?;
    assert_transport_not_configured(
        "repo update",
        futures::executor::block_on(
            repos.update(
                repo_location
                    .clone()
                    .patch()
                    .visibility(Visibility::Public)
                    .get(),
            ),
        ),
    )?;
    assert_transport_not_configured(
        "repo delete",
        futures::executor::block_on(repos.delete(repo_location.clone())),
    )?;
    assert_transport_not_configured(
        "repo branches",
        futures::executor::block_on(repos.branches(repo_location.clone())),
    )?;
    assert_transport_not_configured(
        "repo commits",
        futures::executor::block_on(repos.commits(repo_location)),
    )
}

fn check_issues(provider: &impl Provider) -> VcsResult<()> {
    let repo_location = sample_repo_location();
    let issue_resource = sample_issue(repo_location.clone());
    let issues = provider.issues();

    assert_transport_not_configured(
        "issue get",
        futures::executor::block_on(issues.get(repo_location.clone(), IssueId::make("42"))),
    )?;
    assert_transport_not_configured(
        "issue list",
        futures::executor::block_on(issues.list(issue().query().list(repo_location.clone(), None))),
    )?;
    assert_transport_not_configured(
        "issue create",
        futures::executor::block_on(
            issues.create(
                issue()
                    .draft()
                    .repo(repo_location)
                    .title("Fix release transport")
                    .get(),
            ),
        ),
    )?;
    assert_transport_not_configured(
        "issue update",
        futures::executor::block_on(
            issues.update(
                crate::IssuePatchBuilder::make(issue_resource.clone())
                    .title("Fix")
                    .get(),
            ),
        ),
    )?;
    assert_transport_not_configured(
        "issue close",
        futures::executor::block_on(
            issues.close(
                crate::IssuePatchBuilder::make(issue_resource)
                    .closed()
                    .get(),
            ),
        ),
    )
}

fn check_code_reviews(provider: &impl Provider) -> VcsResult<()> {
    let repo_location = sample_repo_location();
    let code_review_resource = sample_code_review(repo_location.clone());
    let code_reviews = provider.code_reviews();

    assert_transport_not_configured(
        "code review get",
        futures::executor::block_on(
            code_reviews.get(repo_location.clone(), CodeReviewId::make("42")),
        ),
    )?;
    assert_transport_not_configured(
        "code review list",
        futures::executor::block_on(
            code_reviews.list(code_review().query().list(repo_location.clone(), None)),
        ),
    )?;
    assert_transport_not_configured(
        "code review create",
        futures::executor::block_on(
            code_reviews.create(
                code_review()
                    .draft()
                    .repo(repo_location)
                    .title("Add conformance checks")
                    .get(),
            ),
        ),
    )?;
    assert_transport_not_configured(
        "code review update",
        futures::executor::block_on(
            code_reviews.update(
                crate::CodeReviewPatchBuilder::make(code_review_resource.clone())
                    .title("Update conformance checks")
                    .get(),
            ),
        ),
    )?;
    assert_transport_not_configured(
        "code review merge",
        futures::executor::block_on(code_reviews.merge(code_review_resource.clone())),
    )?;
    assert_transport_not_configured(
        "code review close",
        futures::executor::block_on(code_reviews.close(code_review_resource)),
    )
}

fn check_pipelines(provider: &impl Provider) -> VcsResult<()> {
    let repo_location = sample_repo_location();
    let pipeline_resource = sample_pipeline(repo_location.clone());
    let pipelines = provider.pipelines();

    assert_transport_not_configured(
        "pipeline get",
        futures::executor::block_on(pipelines.get(repo_location.clone(), PipelineId::make("42"))),
    )?;
    assert_transport_not_configured(
        "pipeline list",
        futures::executor::block_on(pipelines.list(pipeline().query().list(repo_location, None))),
    )?;
    assert_transport_not_configured(
        "pipeline rerun",
        futures::executor::block_on(pipelines.rerun(pipeline_resource.clone())),
    )?;
    assert_transport_not_configured(
        "pipeline cancel",
        futures::executor::block_on(pipelines.cancel(pipeline_resource)),
    )
}

fn check_releases(provider: &impl Provider) -> VcsResult<()> {
    let repo_location = sample_repo_location();
    let release_resource = sample_release(repo_location.clone());
    let releases = provider.releases();

    assert_transport_not_configured(
        "release get",
        futures::executor::block_on(releases.get(repo_location.clone(), ReleaseId::make("v1.0.0"))),
    )?;
    assert_transport_not_configured(
        "release list",
        futures::executor::block_on(
            releases.list(release().query().list(repo_location.clone(), None)),
        ),
    )?;
    assert_transport_not_configured(
        "release create",
        futures::executor::block_on(
            releases.create(release().draft().repo(repo_location).tag("v1.0.0").get()),
        ),
    )?;
    assert_transport_not_configured(
        "release update",
        futures::executor::block_on(
            releases.update(
                crate::ReleasePatchBuilder::make(release_resource.clone())
                    .body("Release notes")
                    .get(),
            ),
        ),
    )?;
    assert_transport_not_configured(
        "release delete",
        futures::executor::block_on(releases.delete(release_resource)),
    )
}

fn sample_repo_location() -> Repo {
    repo().owner("akira-io").name("vcs-providers-rs").get()
}

fn sample_issue(repo_location: Repo) -> Issue {
    issue().repo(repo_location).id("42").get()
}

fn sample_code_review(repo_location: Repo) -> CodeReview {
    code_review().repo(repo_location).id("42").get()
}

fn sample_pipeline(repo_location: Repo) -> Pipeline {
    pipeline().repo(repo_location).id("42").get()
}

fn sample_release(repo_location: Repo) -> Release {
    release().repo(repo_location).id("v1.0.0").get()
}

fn assert_transport_not_configured<T>(operation: &str, result: VcsResult<T>) -> VcsResult<()> {
    match result {
        Err(VcsError::TransportNotConfigured) => Ok(()),
        Err(_) => Err(error().invalid_input(format!("{operation} returned wrong error"))),
        Ok(_) => Err(error().invalid_input(format!(
            "{operation} succeeded without configured transport"
        ))),
    }
}
