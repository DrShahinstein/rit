use std::collections::HashSet;
use std::process::Command;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileStatus {
  Untracked,
  Modified,
  Added,
  Deleted,
  Renamed,
  Copied,
  Unmerged,
}

#[derive(Debug, Clone)]
pub struct ChangedFile {
  pub path: String,
  pub status: FileStatus,
}

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

pub fn get_changed_files() -> Result<Vec<ChangedFile>, String> {
  let output = run_git(&["status", "--porcelain"])?;

  let files = output
    .lines()
    .filter_map(|line| {
      if line.len() < 4 {
        return None;
      }

      let status_chars: Vec<char> = line.chars().take(2).collect();
      let path = line[3..].to_string();

      let status_char = if status_chars[1] != ' ' {
        status_chars[1]
      } else {
        status_chars[0]
      };

      let status = match status_char {
        '?' => FileStatus::Untracked,
        'M' => FileStatus::Modified,
        'A' => FileStatus::Added,
        'D' => FileStatus::Deleted,
        'R' => FileStatus::Renamed,
        'C' => FileStatus::Copied,
        'U' => FileStatus::Unmerged,
        _ => return None,
      };

      Some(ChangedFile { path, status })
    })
    .collect();

  Ok(files)
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
