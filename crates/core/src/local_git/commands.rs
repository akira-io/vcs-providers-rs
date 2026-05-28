use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use crate::{CognitionResult, error};

pub(super) fn git_stdout<const SIZE: usize>(
    path: &Path,
    args: [&str; SIZE],
) -> CognitionResult<String> {
    let output = run_git(path, args)?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

pub(super) fn git_stdout_optional<const SIZE: usize>(
    path: &Path,
    args: [&str; SIZE],
) -> CognitionResult<Option<String>> {
    let mut command = Command::new("git");
    command.current_dir(path).args(args);
    let output = command
        .output()
        .map_err(|io_error| error().invalid_input(io_error.to_string()))?;

    if !output.status.success() {
        return Ok(None);
    }

    Ok(Some(String::from_utf8_lossy(&output.stdout).to_string()))
}

pub(super) fn run_git<const SIZE: usize>(
    path: &Path,
    args: [&str; SIZE],
) -> CognitionResult<Output> {
    let mut command = Command::new("git");
    command.current_dir(path).args(args);
    command_output(command)
}

pub(super) fn git_stdout_arguments(path: &Path, args: &[String]) -> CognitionResult<String> {
    let output = run_git_arguments(path, args)?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

pub(super) fn run_git_arguments(path: &Path, args: &[String]) -> CognitionResult<Output> {
    let mut command = Command::new("git");
    command.current_dir(path).args(args);
    command_output(command)
}

pub(super) fn run_git_without_repository<const SIZE: usize>(
    args: [&str; SIZE],
    paths: [PathBuf; 2],
) -> CognitionResult<Output> {
    let mut command = Command::new("git");
    command.args(args).args(paths);
    command_output(command)
}

fn command_output(mut command: Command) -> CognitionResult<Output> {
    let output = command
        .output()
        .map_err(|io_error| error().invalid_input(io_error.to_string()))?;

    if output.status.success() {
        return Ok(output);
    }

    Err(error().invalid_input(String::from_utf8_lossy(&output.stderr).trim().to_owned()))
}
