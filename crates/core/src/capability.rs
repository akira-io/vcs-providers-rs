use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Capability {
    Repos,
    RepoGet,
    RepoList,
    RepoSearch,
    RepoBranches,
    RepoCommits,
    RepoCreate,
    RepoUpdate,
    RepoDelete,
    Issues,
    IssueGet,
    IssueList,
    IssueCreate,
    IssueUpdate,
    IssueClose,
    IssueDelete,
    CodeReviews,
    CodeReviewGet,
    CodeReviewList,
    CodeReviewCreate,
    CodeReviewUpdate,
    CodeReviewMerge,
    CodeReviewClose,
    CodeReviewDelete,
    Pipelines,
    PipelineGet,
    PipelineList,
    PipelineRerun,
    PipelineCancel,
    Releases,
    ReleaseGet,
    ReleaseList,
    ReleaseCreate,
    ReleaseUpdate,
    ReleaseDelete,
    Organizations,
    Discussions,
    Webhooks,
    SelfHosted,
}

impl Capability {
    pub fn all() -> &'static [Capability] {
        &[
            Capability::Repos,
            Capability::RepoGet,
            Capability::RepoList,
            Capability::RepoSearch,
            Capability::RepoBranches,
            Capability::RepoCommits,
            Capability::RepoCreate,
            Capability::RepoUpdate,
            Capability::RepoDelete,
            Capability::Issues,
            Capability::IssueGet,
            Capability::IssueList,
            Capability::IssueCreate,
            Capability::IssueUpdate,
            Capability::IssueClose,
            Capability::IssueDelete,
            Capability::CodeReviews,
            Capability::CodeReviewGet,
            Capability::CodeReviewList,
            Capability::CodeReviewCreate,
            Capability::CodeReviewUpdate,
            Capability::CodeReviewMerge,
            Capability::CodeReviewClose,
            Capability::CodeReviewDelete,
            Capability::Pipelines,
            Capability::PipelineGet,
            Capability::PipelineList,
            Capability::PipelineRerun,
            Capability::PipelineCancel,
            Capability::Releases,
            Capability::ReleaseGet,
            Capability::ReleaseList,
            Capability::ReleaseCreate,
            Capability::ReleaseUpdate,
            Capability::ReleaseDelete,
            Capability::Organizations,
            Capability::Discussions,
            Capability::Webhooks,
            Capability::SelfHosted,
        ]
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct CapabilitySet {
    capabilities: BTreeSet<Capability>,
}

impl CapabilitySet {
    pub fn make(capabilities: impl IntoIterator<Item = Capability>) -> Self {
        Self {
            capabilities: capabilities.into_iter().collect(),
        }
    }

    pub fn supports(&self, capability: &Capability) -> bool {
        self.capabilities.contains(capability)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Capability> {
        self.capabilities.iter()
    }
}
