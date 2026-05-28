use std::marker::PhantomData;
use std::path::PathBuf;
use std::process::Command;

use crate::{CognitionResult, error};

use super::LocalGitRepository;
use super::commands::git_stdout_arguments;
use tree_output::MergeTreeOutput;

#[path = "merge/conflict_text.rs"]
mod conflict_text;
#[path = "merge/tree_output.rs"]
mod tree_output;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingBase;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvidedBase;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingOurs;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvidedOurs;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingTheirs;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProvidedTheirs;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalGitMergeBuilder<Base, Ours, Theirs> {
    repository: LocalGitRepository,
    base: Option<String>,
    ours: Option<String>,
    theirs: Option<String>,
    state: PhantomData<(Base, Ours, Theirs)>,
}

impl LocalGitMergeBuilder<MissingBase, MissingOurs, MissingTheirs> {
    pub(super) fn make(repository: LocalGitRepository) -> Self {
        Self {
            repository,
            base: None,
            ours: None,
            theirs: None,
            state: PhantomData,
        }
    }
}

impl<Ours, Theirs> LocalGitMergeBuilder<MissingBase, Ours, Theirs> {
    pub fn base(self, base: impl Into<String>) -> LocalGitMergeBuilder<ProvidedBase, Ours, Theirs> {
        LocalGitMergeBuilder {
            repository: self.repository,
            base: Some(base.into()),
            ours: self.ours,
            theirs: self.theirs,
            state: PhantomData,
        }
    }
}

impl<Base, Theirs> LocalGitMergeBuilder<Base, MissingOurs, Theirs> {
    pub fn ours(self, ours: impl Into<String>) -> LocalGitMergeBuilder<Base, ProvidedOurs, Theirs> {
        LocalGitMergeBuilder {
            repository: self.repository,
            base: self.base,
            ours: Some(ours.into()),
            theirs: self.theirs,
            state: PhantomData,
        }
    }
}

impl<Base, Ours> LocalGitMergeBuilder<Base, Ours, MissingTheirs> {
    pub fn theirs(
        self,
        theirs: impl Into<String>,
    ) -> LocalGitMergeBuilder<Base, Ours, ProvidedTheirs> {
        LocalGitMergeBuilder {
            repository: self.repository,
            base: self.base,
            ours: self.ours,
            theirs: Some(theirs.into()),
            state: PhantomData,
        }
    }
}

impl LocalGitMergeBuilder<ProvidedBase, ProvidedOurs, ProvidedTheirs> {
    pub fn preview(self) -> CognitionResult<MergePreview> {
        let output = self.merge_tree_output()?;
        let merge_output = MergeTreeOutput::parse(&output);
        let conflicts = self.conflicts(&merge_output)?;

        Ok(MergePreview {
            clean: conflicts.is_empty(),
            merged_files: merge_output.merged_files(),
            conflicts,
        })
    }

    pub fn apply(self, plan: MergePlan) -> CognitionResult<MergeOutcome> {
        let _ = self;
        let _ = plan;

        Err(error().unsupported_operation("local git merge apply"))
    }

    fn merge_tree_output(&self) -> CognitionResult<String> {
        let output = Command::new("git")
            .current_dir(&self.repository.path)
            .args(self.arguments()?)
            .output()
            .map_err(|io_error| error().invalid_input(io_error.to_string()))?;

        if !output.stdout.is_empty() {
            return Ok(String::from_utf8_lossy(&output.stdout).to_string());
        }

        if output.status.success() {
            return Ok(String::new());
        }

        Err(error().invalid_input(String::from_utf8_lossy(&output.stderr).trim().to_owned()))
    }

    fn arguments(&self) -> CognitionResult<Vec<String>> {
        Ok(vec![
            "merge-tree".into(),
            "--write-tree".into(),
            "--messages".into(),
            "-z".into(),
            "--merge-base".into(),
            self.base
                .clone()
                .ok_or_else(|| error().invalid_input("missing merge base"))?,
            self.ours
                .clone()
                .ok_or_else(|| error().invalid_input("missing merge ours"))?,
            self.theirs
                .clone()
                .ok_or_else(|| error().invalid_input("missing merge theirs"))?,
        ])
    }

    fn conflicts(&self, output: &MergeTreeOutput) -> CognitionResult<Vec<ConflictRegion>> {
        output.conflicts(|revision| self.git_show(revision))
    }

    fn git_show(&self, revision: impl Into<String>) -> CognitionResult<String> {
        git_stdout_arguments(&self.repository.path, &["show".into(), revision.into()])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MergePreview {
    pub clean: bool,
    pub conflicts: Vec<ConflictRegion>,
    pub merged_files: Vec<PathBuf>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConflictRegion {
    pub path: PathBuf,
    pub base: Option<String>,
    pub ours: String,
    pub theirs: String,
    pub kind: ConflictKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConflictKind {
    Whitespace,
    ImportOrder,
    AdjacentEdit,
    Overlap,
    AddAdd,
    DeleteModify,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MergePlan;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MergeOutcome {
    pub merged_sha: Option<String>,
    pub recovery_ref: String,
}
