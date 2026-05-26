use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
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
