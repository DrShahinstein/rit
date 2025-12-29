use std::process::Command;
use color_eyre::{Result, eyre::bail};

#[allow(dead_code)]
pub mod file {
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub enum Status {
    Modified,  // 'M'
    Added,     // 'A'
    Deleted,   // 'D'
    Renamed,   // 'R'
    Copied,    // 'C'
    Unmerged,  // 'U'
    Untracked, // '?'
  }

  #[derive(Debug, Clone)]
  pub struct Changed {
    pub path:   String,
    pub status: Status,
  }
}

pub fn run(commands: &[&str]) -> Result<String> {
  let output = Command::new("git").args(commands).output()?;

  if output.status.success() {
    Ok(String::from_utf8(output.stdout)?)
  } else {
    bail!("{}", String::from_utf8_lossy(&output.stderr))
  }
}

pub fn get_branch() -> Result<String> {
  run(&["branch", "--show-current"])
    .map(|s| s.trim().to_string())
}

pub fn _get_changed_files() -> Result<Vec<file::Changed>> {
  let _gitstatus = run(&["status", "--porcelain=v1"])?;
  Ok(Vec::new())
}
