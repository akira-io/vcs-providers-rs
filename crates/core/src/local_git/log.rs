use std::path::PathBuf;

use crate::{CognitionResult, Commit};

use super::LocalGitRepository;
use super::commands::git_stdout_arguments;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalGitLog {
    repository: LocalGitRepository,
    range: Option<(String, String)>,
    since_reference: Option<String>,
    paths: Vec<PathBuf>,
    limit: Option<usize>,
    skip: Option<usize>,
}

impl LocalGitLog {
    pub(super) fn make(repository: LocalGitRepository) -> Self {
        Self {
            repository,
            range: None,
            since_reference: None,
            paths: Vec::new(),
            limit: None,
            skip: None,
        }
    }

    pub fn range(self) -> LocalGitLogRange {
        LocalGitLogRange {
            log: self,
            base: None,
        }
    }

    pub fn between(mut self, base: impl Into<String>, head: impl Into<String>) -> Self {
        self.range = Some((base.into(), head.into()));
        self
    }

    pub fn since_ref(mut self, reference: impl Into<String>) -> Self {
        self.since_reference = Some(reference.into());
        self
    }

    pub fn paths(mut self, paths: impl IntoIterator<Item = impl Into<PathBuf>>) -> Self {
        self.paths.extend(paths.into_iter().map(Into::into));
        self
    }

    pub fn limit(mut self, max: usize) -> Self {
        self.limit = Some(max);
        self
    }

    pub fn skip(mut self, count: usize) -> Self {
        self.skip = Some(count);
        self
    }

    pub fn commits(self) -> CognitionResult<Vec<Commit>> {
        let output = git_stdout_arguments(&self.repository.path, &self.arguments(false))?;

        Ok(output.lines().map(Commit::make).collect())
    }

    pub fn graph(self) -> CognitionResult<CommitGraph> {
        let output = git_stdout_arguments(&self.repository.path, &self.arguments(true))?;
        let rows = graph_rows(output.lines().map(raw_graph_row).collect());

        Ok(CommitGraph { rows })
    }

    fn arguments(&self, graph: bool) -> Vec<String> {
        let mut arguments = vec!["log".into(), "--format=%H%x00%P%x00%D".into()];
        self.append_revision(&mut arguments);
        self.append_pagination(&mut arguments);
        self.append_paths(&mut arguments);

        if graph {
            return arguments;
        }

        arguments[1] = "--format=%H".into();
        arguments
    }

    fn append_revision(&self, arguments: &mut Vec<String>) {
        if let Some((base, head)) = &self.range {
            arguments.push(format!("{base}..{head}"));
            return;
        }

        if let Some(reference) = &self.since_reference {
            arguments.push(format!("{reference}..HEAD"));
        }
    }

    fn append_pagination(&self, arguments: &mut Vec<String>) {
        if let Some(limit) = self.limit {
            arguments.push(format!("--max-count={limit}"));
        }

        if let Some(skip) = self.skip {
            arguments.push(format!("--skip={skip}"));
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
pub struct LocalGitLogRange {
    log: LocalGitLog,
    base: Option<String>,
}

impl LocalGitLogRange {
    pub fn base(mut self, base: impl Into<String>) -> Self {
        self.base = Some(base.into());
        self
    }

    pub fn head(mut self, head: impl Into<String>) -> LocalGitLog {
        self.log.range = self.base.map(|base| (base, head.into()));
        self.log
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommitGraph {
    pub rows: Vec<GraphRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GraphRow {
    pub commit: Commit,
    pub lane: usize,
    pub parents: Vec<String>,
    pub refs: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RawGraphRow {
    commit: String,
    parents: Vec<String>,
    refs: Vec<String>,
}

fn raw_graph_row(line: &str) -> RawGraphRow {
    let mut parts = line.split('\0');
    let commit = parts.next().unwrap_or_default().to_owned();
    let parents = parts
        .next()
        .unwrap_or_default()
        .split_whitespace()
        .map(ToOwned::to_owned)
        .collect();
    let refs = parts
        .next()
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|reference| !reference.is_empty())
        .map(ToOwned::to_owned)
        .collect();

    RawGraphRow {
        commit,
        parents,
        refs,
    }
}

fn graph_rows(rows: Vec<RawGraphRow>) -> Vec<GraphRow> {
    let mut lanes: Vec<String> = Vec::new();
    rows.into_iter()
        .map(|row| {
            let lane = lane_for(&row.commit, &lanes);
            lanes = next_lanes(lanes.clone(), lane, &row.parents);

            GraphRow {
                commit: Commit::make(row.commit),
                lane,
                parents: row.parents,
                refs: row.refs,
            }
        })
        .collect()
}

fn lane_for(commit: &str, lanes: &[String]) -> usize {
    lanes
        .iter()
        .position(|lane_commit| lane_commit == commit)
        .unwrap_or(lanes.len())
}

fn next_lanes(mut lanes: Vec<String>, lane: usize, parents: &[String]) -> Vec<String> {
    if parents.is_empty() {
        return remove_lane(lanes, lane);
    }

    if lane >= lanes.len() {
        lanes.push(parents[0].clone());
        lanes.extend(parents.iter().skip(1).cloned());
        return lanes;
    }

    lanes[lane] = parents[0].clone();
    for parent in parents.iter().skip(1).rev() {
        lanes.insert(lane + 1, parent.clone());
    }

    lanes
}

fn remove_lane(mut lanes: Vec<String>, lane: usize) -> Vec<String> {
    if lane < lanes.len() {
        lanes.remove(lane);
    }

    lanes
}

#[cfg(test)]
mod tests {
    use super::{RawGraphRow, graph_rows};

    #[test]
    fn graph_rows_assign_lanes_from_active_parent_topology() {
        let rows = graph_rows(vec![
            raw("merge", ["ours", "theirs"]),
            raw("ours", ["base"]),
            raw("theirs", ["base"]),
            raw("base", []),
        ]);
        let lanes = rows.iter().map(|row| row.lane).collect::<Vec<_>>();

        assert_eq!(lanes, [0, 0, 1, 0]);
    }

    fn raw<const SIZE: usize>(commit: &str, parents: [&str; SIZE]) -> RawGraphRow {
        RawGraphRow {
            commit: commit.into(),
            parents: parents.into_iter().map(str::to_owned).collect(),
            refs: Vec::new(),
        }
    }
}
