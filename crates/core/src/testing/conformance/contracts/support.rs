use crate::{
    Capability, CodeReview, CognitionError, CognitionResult, Issue, Pipeline, Provider, Release,
    Repo, code_review, error, issue, pipeline, release, repo,
};

pub(super) fn sample_repo_location() -> Repo {
    repo().owner("akira-io").name("git-cognition-rs").get()
}

pub(super) fn sample_issue(repo_location: Repo) -> Issue {
    issue().repo(repo_location).id("42").get()
}

pub(super) fn sample_code_review(repo_location: Repo) -> CodeReview {
    code_review().repo(repo_location).id("42").get()
}

pub(super) fn sample_pipeline(repo_location: Repo) -> Pipeline {
    pipeline().repo(repo_location).id("42").get()
}

pub(super) fn sample_release(repo_location: Repo) -> Release {
    release().repo(repo_location).id("v1.0.0").get()
}

pub(super) fn provider_supports(provider: &impl Provider, capability: Capability) -> bool {
    provider.descriptor().capabilities().supports(&capability)
}

pub(super) fn assert_transport_not_configured<T>(
    operation: &str,
    result: CognitionResult<T>,
) -> CognitionResult<()> {
    match result {
        Err(CognitionError::TransportNotConfigured) => Ok(()),
        Err(_) => Err(error().invalid_input(format!("{operation} returned wrong error"))),
        Ok(_) => Err(error().invalid_input(format!(
            "{operation} succeeded without configured transport"
        ))),
    }
}

pub(super) fn assert_capability_contract_error<T>(
    operation: &str,
    result: CognitionResult<T>,
    supported: bool,
) -> CognitionResult<()> {
    if supported {
        return assert_transport_not_configured(operation, result);
    }

    match result {
        Err(CognitionError::UnsupportedOperation(_)) => Ok(()),
        Err(_) => Err(error().invalid_input(format!("{operation} returned wrong error"))),
        Ok(_) => {
            Err(error().invalid_input(format!("{operation} succeeded without provider capability")))
        }
    }
}
