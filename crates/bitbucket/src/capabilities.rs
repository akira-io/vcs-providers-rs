use vcs_provider_core::{Capability, CapabilitySet, capabilities};

pub fn bitbucket_capabilities() -> CapabilitySet {
    capabilities().make([
        Capability::Repos,
        Capability::RepoGet,
        Capability::RepoList,
        Capability::RepoSearch,
        Capability::RepoBranches,
        Capability::RepoCommits,
        Capability::RepoCreate,
        Capability::RepoUpdate,
        Capability::RepoDelete,
        Capability::CodeReviews,
        Capability::CodeReviewGet,
        Capability::CodeReviewList,
        Capability::CodeReviewCreate,
        Capability::CodeReviewUpdate,
        Capability::CodeReviewMerge,
        Capability::CodeReviewClose,
        Capability::Pipelines,
        Capability::PipelineGet,
        Capability::PipelineList,
        Capability::PipelineCancel,
    ])
}
