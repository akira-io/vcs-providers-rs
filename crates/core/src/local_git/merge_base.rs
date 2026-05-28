use crate::{CognitionResult, error};

use super::LocalGitRepository;
use super::commands::git_stdout_arguments;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalGitMergeBase {
    repository: LocalGitRepository,
    reference: Option<String>,
    other_reference: Option<String>,
}

impl LocalGitMergeBase {
    pub(super) fn make(repository: LocalGitRepository) -> Self {
        Self {
            repository,
            reference: None,
            other_reference: None,
        }
    }

    pub fn reference(mut self, reference: impl Into<String>) -> Self {
        self.reference = Some(reference.into());
        self
    }

    pub fn and(mut self, reference: impl Into<String>) -> Self {
        self.other_reference = Some(reference.into());
        self
    }

    pub fn get(self) -> CognitionResult<String> {
        git_stdout_arguments(&self.repository.path, &self.arguments()?)
    }

    fn arguments(&self) -> CognitionResult<Vec<String>> {
        Ok(vec![
            "merge-base".into(),
            self.reference
                .clone()
                .ok_or_else(|| error().invalid_input("missing merge-base reference"))?,
            self.other_reference
                .clone()
                .ok_or_else(|| error().invalid_input("missing other merge-base reference"))?,
        ])
    }
}
