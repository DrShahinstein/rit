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

pub fn get_staged_indices() -> Result<HashSet<usize>, String> {
  let output = run_git(&["status", "--porcelain"])?;
  let mut staged = HashSet::new();

  for (i, line) in output.lines().enumerate() {
    let status: Vec<char> = line.chars().collect();
    
    if status.len() >= 2 {
      let index_status = status[0];
      let worktree_status = status[1];

      if index_status == '?' && worktree_status == '?' {
        continue;
      }

      if index_status != ' ' {
        staged.insert(i);
      }
    }
  }

  Ok(staged)
}

pub fn get_current_branch() -> Result<String, String> {
  run_git(&["branch", "--show-current"]).map(|s| s.trim().to_string())
}
