use std::path::PathBuf;

use super::{ConflictKind, ConflictRegion};

pub(super) fn conflict_regions(
    path: &str,
    output: &str,
    fallback_kind: ConflictKind,
) -> Vec<ConflictRegion> {
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
