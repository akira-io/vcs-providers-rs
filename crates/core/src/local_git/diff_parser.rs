use std::path::PathBuf;

use super::{ChangeKind, DiffFile, DiffLine, Hunk, LineOrigin};

pub(super) fn diff_files(output: &str) -> Vec<DiffFile> {
    let mut files = Vec::new();
    let mut current = None;
    let mut old_line = 0;
    let mut new_line = 0;

    for line in output.lines() {
        if let Some(file) = line.strip_prefix("diff --git ") {
            push_current(&mut files, &mut current);
            current = Some(diff_file(file));
            continue;
        }

        parse_file_metadata(line, &mut current);
        parse_hunk_header(line, &mut current, &mut old_line, &mut new_line);
        parse_hunk_line(line, &mut current, &mut old_line, &mut new_line);
    }

    push_current(&mut files, &mut current);
    files
}

fn push_current(files: &mut Vec<DiffFile>, current: &mut Option<DiffFile>) {
    if let Some(file) = current.take() {
        files.push(file);
    }
}

fn diff_file(value: &str) -> DiffFile {
    let mut paths = value.split_whitespace();

    DiffFile {
        old_path: paths.next().map(clean_diff_path),
        new_path: paths.next().map(clean_diff_path),
        change: ChangeKind::Modified,
        hunks: Vec::new(),
        additions: 0,
        deletions: 0,
        binary: false,
    }
}

fn clean_diff_path(path: &str) -> PathBuf {
    let clean = path
        .strip_prefix("a/")
        .or_else(|| path.strip_prefix("b/"))
        .unwrap_or(path);

    PathBuf::from(clean)
}

fn parse_file_metadata(line: &str, current: &mut Option<DiffFile>) {
    let Some(file) = current.as_mut() else {
        return;
    };

    if line.starts_with("new file mode ") {
        file.old_path = None;
        file.change = ChangeKind::Added;
        return;
    }

    if line.starts_with("deleted file mode ") {
        file.new_path = None;
        file.change = ChangeKind::Deleted;
        return;
    }

    if let Some(similarity) = line.strip_prefix("similarity index ") {
        file.change = ChangeKind::Renamed {
            similarity: similarity.trim_end_matches('%').parse().unwrap_or_default(),
        };
        return;
    }

    if line.starts_with("copy from ") {
        file.change = ChangeKind::Copied;
        return;
    }

    if line.starts_with("old mode ") {
        file.change = ChangeKind::TypeChanged;
        return;
    }

    if line.starts_with("Binary files ") {
        file.binary = true;
    }
}

fn parse_hunk_header(
    line: &str,
    current: &mut Option<DiffFile>,
    old_line: &mut usize,
    new_line: &mut usize,
) {
    if !line.starts_with("@@ ") {
        return;
    }

    let Some(file) = current.as_mut() else {
        return;
    };
    let Some(hunk) = hunk(line) else {
        return;
    };

    *old_line = hunk.old_start;
    *new_line = hunk.new_start;
    file.hunks.push(hunk);
}

fn hunk(line: &str) -> Option<Hunk> {
    let mut parts = line.split_whitespace();
    let _marker = parts.next()?;
    let old = range(parts.next()?)?;
    let new = range(parts.next()?)?;

    Some(Hunk {
        old_start: old.0,
        old_lines: old.1,
        new_start: new.0,
        new_lines: new.1,
        lines: Vec::new(),
    })
}

fn range(value: &str) -> Option<(usize, usize)> {
    let range = value.trim_start_matches(['-', '+']);
    let mut parts = range.split(',');
    let start = parts.next()?.parse().ok()?;
    let lines = parts.next().unwrap_or("1").parse().ok()?;

    Some((start, lines))
}

fn parse_hunk_line(
    line: &str,
    current: &mut Option<DiffFile>,
    old_line: &mut usize,
    new_line: &mut usize,
) {
    let Some(file) = current.as_mut() else {
        return;
    };
    let Some(first) = line.chars().next() else {
        return;
    };

    match first {
        '+' => addition(line, file, old_line, new_line),
        '-' => deletion(line, file, old_line, new_line),
        ' ' => context(line, file, old_line, new_line),
        _ => {}
    }
}

fn addition(line: &str, file: &mut DiffFile, old_line: &mut usize, new_line: &mut usize) {
    let Some(hunk) = file.hunks.last_mut() else {
        return;
    };

    file.additions += 1;

    hunk.lines.push(DiffLine {
        origin: LineOrigin::Addition,
        old_line_no: None,
        new_line_no: Some(*new_line),
        content: line[1..].to_owned(),
    });
    *new_line += 1;
    let _ = old_line;
}

fn deletion(line: &str, file: &mut DiffFile, old_line: &mut usize, new_line: &mut usize) {
    let Some(hunk) = file.hunks.last_mut() else {
        return;
    };

    file.deletions += 1;

    hunk.lines.push(DiffLine {
        origin: LineOrigin::Deletion,
        old_line_no: Some(*old_line),
        new_line_no: None,
        content: line[1..].to_owned(),
    });
    *old_line += 1;
    let _ = new_line;
}

fn context(line: &str, file: &mut DiffFile, old_line: &mut usize, new_line: &mut usize) {
    let Some(hunk) = file.hunks.last_mut() else {
        return;
    };

    hunk.lines.push(DiffLine {
        origin: LineOrigin::Context,
        old_line_no: Some(*old_line),
        new_line_no: Some(*new_line),
        content: line[1..].to_owned(),
    });
    *old_line += 1;
    *new_line += 1;
}
