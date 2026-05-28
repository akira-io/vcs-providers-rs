use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::CognitionResult;

use super::conflict_text::conflict_regions;
use super::{ConflictKind, ConflictRegion};

const MESSAGE_WINDOW: usize = 256;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(super) struct MergeTreeOutput {
    tree_oid: String,
    stages: BTreeMap<String, ConflictStages>,
    messages_blob: String,
}

impl MergeTreeOutput {
    pub(super) fn parse(output: &str) -> Self {
        let mut tokens = output.split('\0');
        let tree_oid = tokens.next().unwrap_or_default().to_owned();
        let mut parsed = Self {
            tree_oid,
            stages: BTreeMap::new(),
            messages_blob: String::new(),
        };

        let mut message_tokens = Vec::new();
        let mut in_messages = false;
        for token in tokens {
            if token.is_empty() {
                in_messages = true;
                continue;
            }

            if in_messages {
                message_tokens.push(token);
                continue;
            }

            parsed.insert_stage(token);
        }

        parsed.messages_blob = message_tokens.join("\n");
        parsed
    }

    pub(super) fn conflicts<F>(&self, mut git_show: F) -> CognitionResult<Vec<ConflictRegion>>
    where
        F: FnMut(String) -> CognitionResult<String>,
    {
        let mut out = Vec::new();
        for path in self.stages.keys() {
            out.extend(self.conflict(path, &mut git_show)?);
        }

        Ok(out)
    }

    pub(super) fn merged_files(&self) -> Vec<PathBuf> {
        self.stages.keys().map(PathBuf::from).collect()
    }

    fn conflict<F>(&self, path: &str, git_show: &mut F) -> CognitionResult<Vec<ConflictRegion>>
    where
        F: FnMut(String) -> CognitionResult<String>,
    {
        let merged = git_show(format!("{}:{path}", self.tree_oid))?;
        let kind = self.kind(path);
        let base = self.base_text(path, git_show)?;
        let mut regions = conflict_regions(path, &merged, kind.clone());

        if regions.is_empty() {
            return Ok(vec![ConflictRegion {
                path: PathBuf::from(path),
                base,
                ours: self.stage_text(path, StageSide::Ours, git_show)?,
                theirs: self.stage_text(path, StageSide::Theirs, git_show)?,
                kind,
            }]);
        }

        for region in &mut regions {
            region.base = base.clone();
        }

        Ok(regions)
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
        let blob = self.messages_blob.as_str();
        let Some(start) = blob.find(path) else {
            return ConflictKind::Overlap;
        };
        let end = blob.len().min(start + path.len() + MESSAGE_WINDOW);
        let window = &blob[start..end];

        if window.contains("add/add") {
            return ConflictKind::AddAdd;
        }

        if window.contains("delete/modify") || window.contains("modify/delete") {
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

#[cfg(test)]
mod tests {
    use super::{ConflictKind, MergeTreeOutput};
    use crate::CognitionResult;
    use std::path::PathBuf;

    fn tree_with_stages(paths: &[&str]) -> String {
        let mut out = String::from("treeoid");
        for path in paths {
            out.push('\0');
            out.push_str(&format!(
                "100644 baseblob 1\t{path}\0100644 oursblob 2\t{path}\0100644 theirsblob 3\t{path}"
            ));
        }
        out.push('\0');
        out.push('\0');
        out
    }

    #[test]
    fn parse_collects_stage_paths_into_merged_files() {
        let raw = tree_with_stages(&["src/a.rs", "src/b.rs"]);
        let parsed = MergeTreeOutput::parse(&raw);

        let files: Vec<PathBuf> = parsed.merged_files();
        assert!(files.contains(&PathBuf::from("src/a.rs")));
        assert!(files.contains(&PathBuf::from("src/b.rs")));
    }

    #[test]
    fn kind_detects_add_add_message_near_path() {
        let mut raw = tree_with_stages(&["src/a.rs"]);
        raw.push_str("src/a.rs\0CONFLICT (add/add)\0both sides added\0");
        let parsed = MergeTreeOutput::parse(&raw);

        assert_eq!(parsed.kind_for_test("src/a.rs"), ConflictKind::AddAdd);
    }

    #[test]
    fn kind_detects_modify_delete_message_near_path() {
        let mut raw = tree_with_stages(&["src/a.rs"]);
        raw.push_str("src/a.rs\0CONFLICT (modify/delete)\0one side deleted\0");
        let parsed = MergeTreeOutput::parse(&raw);

        assert_eq!(parsed.kind_for_test("src/a.rs"), ConflictKind::DeleteModify);
    }

    #[test]
    fn kind_defaults_to_overlap_when_path_absent_from_messages() {
        let raw = tree_with_stages(&["src/a.rs"]);
        let parsed = MergeTreeOutput::parse(&raw);

        assert_eq!(parsed.kind_for_test("src/a.rs"), ConflictKind::Overlap);
    }

    #[test]
    fn conflicts_returns_every_region_with_base_propagated() -> CognitionResult<()> {
        let raw = tree_with_stages(&["src/a.rs"]);
        let parsed = MergeTreeOutput::parse(&raw);

        let merged = String::from(
            "ctx\n\
             <<<<<<< ours\nours-1\n=======\ntheirs-1\n>>>>>>> theirs\n\
             middle\n\
             <<<<<<< ours\nours-2\n=======\ntheirs-2\n>>>>>>> theirs\n",
        );

        let regions = parsed.conflicts(|revision| {
            if revision == format!("{}:{}", "treeoid", "src/a.rs") {
                return Ok(merged.clone());
            }

            if revision == "baseblob" {
                return Ok("base-text".into());
            }

            Ok(String::new())
        })?;

        assert_eq!(regions.len(), 2);
        for region in &regions {
            assert_eq!(region.base.as_deref(), Some("base-text"));
            assert_eq!(region.path, PathBuf::from("src/a.rs"));
        }
        assert_eq!(regions[0].ours, "ours-1");
        assert_eq!(regions[1].theirs, "theirs-2");
        Ok(())
    }

    impl MergeTreeOutput {
        fn kind_for_test(&self, path: &str) -> ConflictKind {
            self.kind(path)
        }
    }
}
