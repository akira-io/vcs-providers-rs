use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

use crate::CognitionResult;

use super::{ConflictKind, ConflictRegion};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(super) struct MergeTreeOutput {
    tree_oid: String,
    stages: BTreeMap<String, ConflictStages>,
    messages: BTreeMap<String, Vec<String>>,
}

impl MergeTreeOutput {
    pub(super) fn parse(output: &str) -> Self {
        let mut tokens = output.split('\0');
        let tree_oid = tokens.next().unwrap_or_default().to_owned();
        let mut parsed = Self {
            tree_oid,
            stages: BTreeMap::new(),
            messages: BTreeMap::new(),
        };

        let mut message_tokens = Vec::new();
        let mut in_messages = false;
        for token in tokens {
            if token.is_empty() {
                in_messages = true;
                continue;
            }

            if in_messages {
                message_tokens.push(token.to_owned());
                continue;
            }

            parsed.insert_stage(token);
        }

        parsed.insert_messages(message_tokens);
        parsed
    }

    pub(super) fn conflicts<F>(&self, mut git_show: F) -> CognitionResult<Vec<ConflictRegion>>
    where
        F: FnMut(String) -> CognitionResult<String>,
    {
        self.stages
            .keys()
            .map(|path| self.conflict(path, &mut git_show))
            .collect()
    }

    pub(super) fn merged_files(&self) -> Vec<PathBuf> {
        let mut paths = BTreeSet::new();
        paths.extend(self.stages.keys().map(PathBuf::from));
        paths.extend(self.messages.keys().map(PathBuf::from));
        paths.into_iter().collect()
    }

    fn conflict<F>(&self, path: &str, git_show: &mut F) -> CognitionResult<ConflictRegion>
    where
        F: FnMut(String) -> CognitionResult<String>,
    {
        let merged = git_show(format!("{}:{path}", self.tree_oid))?;
        let mut regions = conflict_regions(path, &merged, self.kind(path));
        let base = self.base_text(path, git_show)?;

        if let Some(region) = regions.first_mut() {
            region.base = base;
            return Ok(region.clone());
        }

        Ok(ConflictRegion {
            path: PathBuf::from(path),
            base,
            ours: self.stage_text(path, StageSide::Ours, git_show)?,
            theirs: self.stage_text(path, StageSide::Theirs, git_show)?,
            kind: self.kind(path),
        })
    }

    fn insert_stage(&mut self, token: &str) {
        let Some((metadata, path)) = token.split_once('\t') else {
            return;
        };
        let mut parts = metadata.split_whitespace();
        let _mode = parts.next();
        let blob = parts.next().unwrap_or_default().to_owned();
        let stage = parts.next().unwrap_or_default();
        let stages = self.stages.entry(path.to_owned()).or_default();

        match stage {
            "1" => stages.base = Some(blob),
            "2" => stages.ours = Some(blob),
            "3" => stages.theirs = Some(blob),
            _ => {}
        }
    }

    fn insert_messages(&mut self, tokens: Vec<String>) {
        for chunk in tokens.chunks(4) {
            if chunk.len() < 4 {
                continue;
            }

            self.messages
                .entry(chunk[1].clone())
                .or_default()
                .push(chunk[2].clone());
        }
    }

    fn base_text<F>(&self, path: &str, git_show: &mut F) -> CognitionResult<Option<String>>
    where
        F: FnMut(String) -> CognitionResult<String>,
    {
        let Some(blob) = self.stages.get(path).and_then(|stages| stages.base.clone()) else {
            return Ok(None);
        };

        Ok(Some(git_show(blob)?))
    }

    fn stage_text<F>(
        &self,
        path: &str,
        side: StageSide,
        git_show: &mut F,
    ) -> CognitionResult<String>
    where
        F: FnMut(String) -> CognitionResult<String>,
    {
        let Some(stages) = self.stages.get(path) else {
            return Ok(String::new());
        };
        let blob = match side {
            StageSide::Ours => stages.ours.clone(),
            StageSide::Theirs => stages.theirs.clone(),
        };
        let Some(blob) = blob else {
            return Ok(String::new());
        };

        git_show(blob)
    }

    fn kind(&self, path: &str) -> ConflictKind {
        let message = self
            .messages
            .get(path)
            .into_iter()
            .flatten()
            .map(String::as_str)
            .collect::<Vec<_>>()
            .join(" ");

        if message.contains("add/add") {
            return ConflictKind::AddAdd;
        }

        if message.contains("delete/modify") {
            return ConflictKind::DeleteModify;
        }

        ConflictKind::Overlap
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct ConflictStages {
    base: Option<String>,
    ours: Option<String>,
    theirs: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum StageSide {
    Ours,
    Theirs,
}

fn conflict_regions(path: &str, output: &str, fallback_kind: ConflictKind) -> Vec<ConflictRegion> {
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
            regions.push(region(path, &ours, &theirs, fallback_kind.clone()));
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

    regions
}

fn region(
    path: &str,
    ours: &[String],
    theirs: &[String],
    fallback_kind: ConflictKind,
) -> ConflictRegion {
    let ours = ours.join("\n");
    let theirs = theirs.join("\n");
    let kind = content_kind(&ours, &theirs).unwrap_or(fallback_kind);

    ConflictRegion {
        path: PathBuf::from(path),
        base: None,
        ours,
        theirs,
        kind,
    }
}

fn content_kind(ours: &str, theirs: &str) -> Option<ConflictKind> {
    if ours.trim() == theirs.trim() {
        return Some(ConflictKind::Whitespace);
    }

    if import_only(ours) && import_only(theirs) {
        return Some(ConflictKind::ImportOrder);
    }

    None
}

fn import_only(content: &str) -> bool {
    let mut lines = content
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty());
    let Some(first) = lines.next() else {
        return false;
    };

    import_line(first) && lines.all(import_line)
}

fn import_line(line: &str) -> bool {
    line.starts_with("use ") || line.starts_with("import ") || line.starts_with("from ")
}
