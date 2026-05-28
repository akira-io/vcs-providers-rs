use crate::{CognitionResult, Commit};

use super::LocalGitRepository;
use super::commands::git_stdout_arguments;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalGitReference {
    repository: LocalGitRepository,
    reference: String,
}

impl LocalGitReference {
    pub(super) fn make(repository: LocalGitRepository, reference: impl Into<String>) -> Self {
        Self {
            repository,
            reference: reference.into(),
        }
    }

    pub fn sha(&self) -> CognitionResult<String> {
        git_stdout_arguments(
            &self.repository.path,
            &["rev-parse".into(), self.reference.clone()],
        )
    }

    pub fn commit(&self) -> CognitionResult<Commit> {
        Ok(Commit::make(self.sha()?))
    }
}
