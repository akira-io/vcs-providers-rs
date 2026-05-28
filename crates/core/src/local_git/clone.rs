use std::path::PathBuf;

use crate::CognitionResult;

use super::commands::run_git_without_repository;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingCloneDestination;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvidedCloneDestination {
    pub(super) destination: PathBuf,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalGitCloneBuilder<DestinationState> {
    pub(super) source: PathBuf,
    pub(super) destination: DestinationState,
}

impl LocalGitCloneBuilder<MissingCloneDestination> {
    pub fn to(
        self,
        destination: impl Into<PathBuf>,
    ) -> LocalGitCloneBuilder<ProvidedCloneDestination> {
        LocalGitCloneBuilder {
            source: self.source,
            destination: ProvidedCloneDestination {
                destination: destination.into(),
            },
        }
    }
}

impl LocalGitCloneBuilder<ProvidedCloneDestination> {
    pub fn clone(self) -> CognitionResult<()> {
        run_git_without_repository(["clone"], [self.source, self.destination.destination])
            .map(|_| ())
    }
}
