#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LocalGitCapability {
    Log,
    LogGraph,
    Diff,
    DiffRename,
    Blame,
    MergePreview,
    MergeApply,
    Worktree,
    Status,
    Show,
    MergeBase,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LocalGitCapabilitySet {
    capabilities: Vec<LocalGitCapability>,
}

impl LocalGitCapabilitySet {
    pub fn make(capabilities: impl IntoIterator<Item = LocalGitCapability>) -> Self {
        Self {
            capabilities: capabilities.into_iter().collect(),
        }
    }

    pub fn supports(&self, capability: &LocalGitCapability) -> bool {
        self.capabilities.contains(capability)
    }
}
