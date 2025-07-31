use std::collections::HashSet;
use std::process::Command;

pub fn run_git(commands: &[&str]) -> Result<String, String> {
  let output = Command::new("git")
    .args(commands)
    .output()
    .map_err(|e| e.to_string())?;

  if output.status.success() {
    String::from_utf8(output.stdout).map_err(|e| e.to_string())
  } else {
    Err(String::from_utf8_lossy(&output.stderr).into())
  }
}

pub fn get_changed_files() -> Result<Vec<String>, String> {
  let output = run_git(&["status", "--porcelain"]);
  output.map(|s| s.lines().map(|line| line[3..].to_string()).collect())
}

pub fn get_current_branch() -> Result<String, String> {
  run_git(&["branch", "--show-current"]).map(|s| s.trim().to_string())
}

pub fn get_staged_indices() -> Result<HashSet<usize>, String> {
  let output = run_git(&["diff", "--cached", "--name-only"]);
  output.map(|s| {
    s.lines()
      .enumerate()
      .filter_map(|(i, line)| if !line.is_empty() { Some(i) } else { None })
      .collect()
  })
}
