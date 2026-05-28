use std::path::PathBuf;

use crate::CognitionResult;

use super::LocalGitRepository;
use super::commands::git_stdout_arguments;

#[path = "diff_parser.rs"]
mod parser;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalGitDiff {
    repository: LocalGitRepository,
    selector: DiffSelector,
    paths: Vec<PathBuf>,
    detect_renames: bool,
    context_lines: usize,
}

impl LocalGitDiff {
    pub(super) fn make(repository: LocalGitRepository) -> Self {
        Self {
            repository,
            selector: DiffSelector::Working,
            paths: Vec::new(),
            detect_renames: false,
            context_lines: 3,
        }
    }

    pub fn working(mut self) -> Self {
        self.selector = DiffSelector::Working;
        self
    }

    pub fn staged(mut self) -> Self {
        self.selector = DiffSelector::Staged;
        self
    }

    pub fn commit(mut self, sha: impl Into<String>) -> Self {
        self.selector = DiffSelector::Commit(sha.into());
        self
    }

    pub fn range(mut self, base: impl Into<String>, head: impl Into<String>) -> Self {
        self.selector = DiffSelector::Range(base.into(), head.into());
        self
    }

    pub fn paths(mut self, paths: impl IntoIterator<Item = impl Into<PathBuf>>) -> Self {
        self.paths.extend(paths.into_iter().map(Into::into));
        self
    }

    pub fn detect_renames(mut self, enabled: bool) -> Self {
        self.detect_renames = enabled;
        self
    }

    pub fn context_lines(mut self, lines: usize) -> Self {
        self.context_lines = lines;
        self
    }

    pub fn compute(self) -> CognitionResult<DiffModel> {
        let output = git_stdout_arguments(&self.repository.path, &self.arguments())?;

        Ok(DiffModel {
            files: parser::diff_files(&output),
        })
    }

    fn arguments(&self) -> Vec<String> {
        let mut arguments = vec![
            "diff".into(),
            "--no-color".into(),
            format!("--unified={}", self.context_lines),
        ];

        self.append_selector(&mut arguments);
        self.append_rename_detection(&mut arguments);
        self.append_paths(&mut arguments);
        arguments
    }

    fn append_selector(&self, arguments: &mut Vec<String>) {
        match &self.selector {
            DiffSelector::Working => {}
            DiffSelector::Staged => arguments.push("--cached".into()),
            DiffSelector::Commit(sha) => arguments.push(format!("{sha}^..{sha}")),
            DiffSelector::Range(base, head) => arguments.push(format!("{base}...{head}")),
        }
    }

    fn append_rename_detection(&self, arguments: &mut Vec<String>) {
        if self.detect_renames {
            arguments.push("-M".into());
        }
    }

    fn append_paths(&self, arguments: &mut Vec<String>) {
        if self.paths.is_empty() {
            return;
        }

        arguments.push("--".into());
        arguments.extend(
            self.paths
                .iter()
                .map(|path| path.to_string_lossy().to_string()),
        );
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum DiffSelector {
    Working,
    Staged,
    Commit(String),
    Range(String, String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiffModel {
    pub files: Vec<DiffFile>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiffFile {
    pub old_path: Option<PathBuf>,
    pub new_path: Option<PathBuf>,
    pub change: ChangeKind,
    pub hunks: Vec<Hunk>,
    pub additions: usize,
    pub deletions: usize,
    pub binary: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ChangeKind {
    Added,
    Deleted,
    Modified,
    Renamed { similarity: u8 },
    Copied,
    TypeChanged,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Hunk {
    pub old_start: usize,
    pub old_lines: usize,
    pub new_start: usize,
    pub new_lines: usize,
    pub lines: Vec<DiffLine>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiffLine {
    pub origin: LineOrigin,
    pub old_line_no: Option<usize>,
    pub new_line_no: Option<usize>,
    pub content: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LineOrigin {
    Context,
    Addition,
    Deletion,
}
