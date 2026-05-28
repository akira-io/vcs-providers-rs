use std::path::PathBuf;

use crate::CognitionResult;

use super::LocalGitRepository;
use super::commands::git_stdout_arguments;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StatusEntry {
    pub path: PathBuf,
    pub staged: FileState,
    pub unstaged: FileState,
    pub original_path: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FileState {
    Unmodified,
    Modified,
    Added,
    Deleted,
    Renamed,
    Copied,
    Untracked,
    Ignored,
}

pub(super) fn status(repository: &LocalGitRepository) -> CognitionResult<Vec<StatusEntry>> {
    let output = git_stdout_arguments(
        &repository.path,
        &["status".into(), "--porcelain=v2".into()],
    )?;
    let entries = output
        .lines()
        .filter(|line| !line.is_empty())
        .filter_map(status_entry)
        .collect();

    Ok(entries)
}

fn status_entry(line: &str) -> Option<StatusEntry> {
    if let Some(path) = line.strip_prefix("? ") {
        return Some(StatusEntry {
            path: PathBuf::from(path),
            staged: FileState::Untracked,
            unstaged: FileState::Untracked,
            original_path: None,
        });
    }

    if let Some(path) = line.strip_prefix("! ") {
        return Some(StatusEntry {
            path: PathBuf::from(path),
            staged: FileState::Ignored,
            unstaged: FileState::Ignored,
            original_path: None,
        });
    }

    let mut parts = line.splitn(9, ' ');
    let record = parts.next()?;

    if record != "1" {
        return None;
    }

    let mut states = parts.next()?.chars();
    let staged = file_state(states.next()?);
    let unstaged = file_state(states.next()?);
    let path = parts.last()?;

    Some(StatusEntry {
        path: PathBuf::from(path),
        staged,
        unstaged,
        original_path: None,
    })
}

fn file_state(state: char) -> FileState {
    match state {
        'M' => FileState::Modified,
        'A' => FileState::Added,
        'D' => FileState::Deleted,
        'R' => FileState::Renamed,
        'C' => FileState::Copied,
        '?' => FileState::Untracked,
        '!' => FileState::Ignored,
        _ => FileState::Unmodified,
    }
}
