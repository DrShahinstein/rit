use std::process::Command;
use color_eyre::{Result, eyre::bail};

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

  impl Status {
    pub fn from_char(c: char) -> Self {
      match c {
        'M' => Self::Modified,
        'A' => Self::Added,
        'D' => Self::Deleted,
        'R' => Self::Renamed,
        'C' => Self::Copied,
        'U' => Self::Unmerged,
        '?' => Self::Untracked,
         _  => Self::Untracked,
      }
    }
  }

  #[allow(dead_code)]
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

pub fn get_changed_files() -> Result<Vec<file::Changed>> {
  let mut changed_files = Vec::new();
  let gitstatus         = run(&["status", "--porcelain=v1"])?;

  for line in gitstatus.trim().lines() {
    let s: Vec<&str> = line.split_whitespace().collect();

    let path   = s[1].to_string();
    let status = file::Status::from_char(
      s[0]
       .chars()
       .next()
       .unwrap()
    );

    if s.len() >= 2 {
      changed_files.push(
        file::Changed {
          status, 
          path,
        }
      );
    }
  }


  Ok(changed_files)
}

/*
 *
 * pub fn get_changed_files()...
 * `gitstatus` gives something like
 * "M src/tui/app.rs"
 * "M src/tui/git.rs"
 *
 */
