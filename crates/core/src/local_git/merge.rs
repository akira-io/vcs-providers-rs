use std::marker::PhantomData;
use std::path::PathBuf;

use crate::{VcsResult, error};

use super::LocalGitRepository;
use super::commands::git_stdout_arguments;

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
    pub fn preview(self) -> VcsResult<MergePreview> {
        let output = git_stdout_arguments(&self.repository.path, &self.arguments()?)?;

        Ok(MergePreview {
            clean: merge_output_is_clean(&output),
            conflicts: conflict_regions(&output),
            merged_files: Vec::new(),
        })
    }

    pub fn apply(self, plan: MergePlan) -> VcsResult<MergeOutcome> {
        let _ = self;
        let _ = plan;

        Err(error().unsupported_operation("local git merge apply"))
    }

    fn arguments(&self) -> VcsResult<Vec<String>> {
        Ok(vec![
            "merge-tree".into(),
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

fn conflict_regions(output: &str) -> Vec<ConflictRegion> {
    let mut regions = Vec::new();
    let mut ours = Vec::new();
    let mut theirs = Vec::new();
    let mut in_ours = false;
    let mut in_theirs = false;

    for line in output.lines() {
        if line.starts_with("<<<<<<<") {
            in_ours = true;
            continue;
        }

        if line.starts_with("=======") {
            in_ours = false;
            in_theirs = true;
            continue;
        }

        if line.starts_with(">>>>>>>") {
            in_theirs = false;
            regions.push(region(&ours, &theirs));
            ours.clear();
            theirs.clear();
            continue;
        }

        if in_ours {
            ours.push(line.to_owned());
            continue;
        }

        if in_theirs {
            theirs.push(line.to_owned());
        }
    }

    if !regions.is_empty() {
        return regions;
    }

    if output.contains("changed in both") {
        return vec![region(&[], &[])];
    }

    regions
}

fn merge_output_is_clean(output: &str) -> bool {
    if output.contains("<<<<<<<") {
        return false;
    }

    !output.contains("changed in both")
}

fn region(ours: &[String], theirs: &[String]) -> ConflictRegion {
    ConflictRegion {
        path: PathBuf::new(),
        base: None,
        ours: ours.join("\n"),
        theirs: theirs.join("\n"),
        kind: ConflictKind::Overlap,
    }
}
