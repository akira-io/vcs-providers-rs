use std::path::PathBuf;

use crate::CognitionResult;

use super::LocalGitRepository;
use super::commands::git_stdout_arguments;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalGitBlame {
    repository: LocalGitRepository,
    path: PathBuf,
    revision: String,
}

impl LocalGitBlame {
    pub(super) fn make(repository: LocalGitRepository, path: impl Into<PathBuf>) -> Self {
        Self {
            repository,
            path: path.into(),
            revision: "HEAD".into(),
        }
    }

    pub fn at(mut self, revision: impl Into<String>) -> Self {
        self.revision = revision.into();
        self
    }

    pub fn compute(self) -> CognitionResult<Blame> {
        let output = git_stdout_arguments(&self.repository.path, &self.arguments())?;

        Ok(Blame {
            path: self.path,
            spans: blame_spans(&output),
        })
    }

    fn arguments(&self) -> Vec<String> {
        vec![
            "blame".into(),
            "--porcelain".into(),
            self.revision.clone(),
            "--".into(),
            self.path.to_string_lossy().to_string(),
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Blame {
    pub path: PathBuf,
    pub spans: Vec<BlameSpan>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BlameSpan {
    pub commit: String,
    pub author: String,
    pub start_line: usize,
    pub line_count: usize,
}

fn blame_spans(output: &str) -> Vec<BlameSpan> {
    let mut spans = Vec::new();
    let mut current_commit = String::new();
    let mut current_author = String::new();
    let mut current_start_line = 0;
    let mut current_line_count = 0;

    for line in output.lines() {
        if let Some((commit, start_line, line_count)) = header(line) {
            current_commit = commit;
            current_start_line = start_line;
            current_line_count = line_count;
            continue;
        }

        if let Some(author) = line.strip_prefix("author ") {
            current_author = author.to_owned();
            continue;
        }

        if !line.starts_with('\t') {
            continue;
        }

        spans.push(BlameSpan {
            commit: current_commit.clone(),
            author: current_author.clone(),
            start_line: current_start_line,
            line_count: current_line_count,
        });
    }

    spans
}

fn header(line: &str) -> Option<(String, usize, usize)> {
    let mut parts = line.split_whitespace();
    let commit = parts.next()?;

    if commit.len() < 7 {
        return None;
    }

    let _original_line = parts.next()?;
    let final_line = parts.next()?.parse().ok()?;
    let line_count = parts.next().unwrap_or("1").parse().ok()?;

    Some((commit.to_owned(), final_line, line_count))
}
