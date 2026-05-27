use crate::{
    Capability, CodeReviewId, PipelineId, Provider, VcsResult, Visibility, code_review, issue,
    issue_id, pipeline, release, release_id, repo, testing::run_async_test,
};

#[path = "contracts/support.rs"]
mod support;

use support::{
    assert_capability_contract_error, assert_transport_not_configured, provider_supports,
    sample_code_review, sample_issue, sample_pipeline, sample_release, sample_repo_location,
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

    assert_transport_not_configured("repo get", run_async_test(repos.get(repo_location.clone())))?;
    assert_transport_not_configured(
        "repo list",
        run_async_test(repos.list(repo().query().optional_pagination(None).list())),
    )?;
    assert_transport_not_configured(
        "repo search",
        run_async_test(
            repos.search(
                repo()
                    .query()
                    .search("vcs")
                    .optional_pagination(None)
                    .search(),
            ),
        ),
    )?;
    assert_transport_not_configured(
        "repo create",
        run_async_test(
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
        run_async_test(
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
        run_async_test(repos.delete(repo_location.clone())),
    )?;
    assert_transport_not_configured(
        "repo branches",
        run_async_test(repos.branches(repo_location.clone())),
    )?;
    assert_transport_not_configured("repo commits", run_async_test(repos.commits(repo_location)))
}

fn check_issues(provider: &impl Provider) -> VcsResult<()> {
    let repo_location = sample_repo_location();
    let issue_resource = sample_issue(repo_location.clone());
    let issues = provider.issues();
    let supported = provider_supports(provider, Capability::Issues);

    assert_capability_contract_error(
        "issue get",
        run_async_test(issues.get(repo_location.clone(), issue_id("42"))),
        supported,
    )?;
    assert_capability_contract_error(
        "issue list",
        run_async_test(issues.list(issue().query().location(repo_location.clone()).list())),
        supported,
    )?;
    assert_capability_contract_error(
        "issue create",
        run_async_test(
            issues.create(
                issue()
                    .draft()
                    .repo(repo_location)
                    .title("Fix release transport")
                    .get(),
            ),
        ),
        supported,
    )?;
    assert_capability_contract_error(
        "issue update",
        run_async_test(issues.update(issue_resource.patch().title("Fix").get())),
        supported,
    )?;
    assert_capability_contract_error(
        "issue close",
        run_async_test(issues.close(issue_resource.patch().closed().get())),
        supported,
    )?;
    assert_capability_contract_error(
        "issue delete",
        run_async_test(issues.delete(sample_issue(sample_repo_location()))),
        supported,
    )
}

fn check_code_reviews(provider: &impl Provider) -> VcsResult<()> {
    let repo_location = sample_repo_location();
    let code_review_resource = sample_code_review(repo_location.clone());
    let code_reviews = provider.code_reviews();

    assert_transport_not_configured(
        "code review get",
        run_async_test(code_reviews.get(repo_location.clone(), CodeReviewId::make("42"))),
    )?;
    assert_transport_not_configured(
        "code review list",
        run_async_test(
            code_reviews.list(code_review().query().location(repo_location.clone()).list()),
        ),
    )?;
    assert_transport_not_configured(
        "code review create",
        run_async_test(
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
        run_async_test(
            code_reviews.update(
                code_review_resource
                    .patch()
                    .title("Update conformance checks")
                    .get(),
            ),
        ),
    )?;
    assert_transport_not_configured(
        "code review merge",
        run_async_test(code_reviews.merge(code_review_resource.clone())),
    )?;
    assert_transport_not_configured(
        "code review close",
        run_async_test(code_reviews.close(code_review_resource.clone())),
    )?;
    assert_transport_not_configured(
        "code review delete",
        run_async_test(code_reviews.delete(code_review_resource)),
    )
}

fn check_pipelines(provider: &impl Provider) -> VcsResult<()> {
    let repo_location = sample_repo_location();
    let pipeline_resource = sample_pipeline(repo_location.clone());
    let pipelines = provider.pipelines();

    assert_transport_not_configured(
        "pipeline get",
        run_async_test(pipelines.get(repo_location.clone(), PipelineId::make("42"))),
    )?;
    assert_transport_not_configured(
        "pipeline list",
        run_async_test(pipelines.list(pipeline().query().location(repo_location).list())),
    )?;
    assert_transport_not_configured(
        "pipeline rerun",
        run_async_test(pipelines.rerun(pipeline_resource.clone())),
    )?;
    assert_transport_not_configured(
        "pipeline cancel",
        run_async_test(pipelines.cancel(pipeline_resource)),
    )
}

fn check_releases(provider: &impl Provider) -> VcsResult<()> {
    let repo_location = sample_repo_location();
    let release_resource = sample_release(repo_location.clone());
    let releases = provider.releases();
    let supported = provider_supports(provider, Capability::Releases);

    assert_capability_contract_error(
        "release get",
        run_async_test(releases.get(repo_location.clone(), release_id("v1.0.0"))),
        supported,
    )?;
    assert_capability_contract_error(
        "release list",
        run_async_test(releases.list(release().query().location(repo_location.clone()).list())),
        supported,
    )?;
    assert_capability_contract_error(
        "release create",
        run_async_test(releases.create(release().draft().repo(repo_location).tag("v1.0.0").get())),
        supported,
    )?;
    assert_capability_contract_error(
        "release update",
        run_async_test(releases.update(release_resource.patch().body("Release notes").get())),
        supported,
    )?;
    assert_capability_contract_error(
        "release delete",
        run_async_test(releases.delete(release_resource)),
        supported,
    )
}
