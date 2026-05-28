use std::path::PathBuf;

use crate::CognitionResult;

use super::LocalGitRepository;
use super::commands::git_stdout_arguments;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalGitShow {
    repository: LocalGitRepository,
    revision: String,
}

impl LocalGitShow {
    pub(super) fn make(repository: LocalGitRepository, revision: impl Into<String>) -> Self {
        Self {
            repository,
            revision: revision.into(),
        }
    }

    pub fn file(self, path: impl Into<PathBuf>) -> CognitionResult<String> {
        let path = path.into();
        let object = format!("{}:{}", self.revision, path.to_string_lossy());

        git_stdout_arguments(&self.repository.path, &["show".into(), object])
    }
}
