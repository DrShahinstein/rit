use std::process::{Command, Stdio};
use std::io::Write;
use color_eyre::{Result, eyre::bail};

pub mod file {
  #[derive(Debug, Clone, Copy, PartialEq, Eq)]
  pub enum Status {
    Unmodified, // ' '
    Modified,   // 'M'
    Added,      // 'A'
    Deleted,    // 'D'
    Renamed,    // 'R'
    Copied,     // 'C'
    Unmerged,   // 'U'
    Untracked,  // '?'
    Other(char),
  }

  impl Status {
    pub fn from_char(c: char) -> Self {
      match c {
        ' ' => Self::Unmodified,
        'M' => Self::Modified,
        'A' => Self::Added,
        'D' => Self::Deleted,
        'R' => Self::Renamed,
        'C' => Self::Copied,
        'U' => Self::Unmerged,
        '?' => Self::Untracked,
         c  => Self::Other(c),
      }
    }
    pub fn as_char(self) -> char {
      match self {
        Self::Unmodified => ' ',
        Self::Modified   => 'M',
        Self::Added      => 'A',
        Self::Deleted    => 'D',
        Self::Renamed    => 'R',
        Self::Copied     => 'C',
        Self::Unmerged   => 'U',
        Self::Untracked  => '?',
        Self::Other(c)   => c,
      }
    }
  }

  #[derive(Debug, Clone, PartialEq, Eq)]
  pub struct Changed {
    pub path:     String,
    pub index:    Status, // X
    pub worktree: Status, // Y
  }

  impl Changed {
    pub fn is_staged(&self) -> bool { self.index    != Status::Unmodified && self.index    != Status::Untracked }
    pub fn is_dirty(&self)  -> bool { self.worktree != Status::Unmodified && self.worktree != Status::Untracked }
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

  for line in gitstatus.lines() {
    if line.is_empty() || line.len() < 4 { continue; }

    let x = line.chars().nth(0).unwrap();
    let y = line.chars().nth(1).unwrap();

    let raw_path = line[3..].trim();
    let path     = if let Some((_, new_path)) = raw_path.split_once(" -> ") {
      new_path.trim()
    } else {
      raw_path
    };

    changed_files.push(file::Changed {
      path:     path.to_string(),
      index:    file::Status::from_char(x),
      worktree: file::Status::from_char(y),
    });
  }

  Ok(changed_files)
}

pub fn stage(path: &str) -> Result<()> {
  run(&["add", path]).map(|_| ())
}

pub fn unstage(path: &str) -> Result<()> {
  run(&["restore", "--staged", path]).map(|_| ())
}

pub fn commit(message: &str) -> Result<()> {
  let mut child = Command::new("git")
    .args(&["commit", "-F", "-"])
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()?;

  if let Some(mut stdin) = child.stdin.take() {
    stdin.write_all(message.as_bytes())?;
  }

  let output = child.wait_with_output()?;

  if output.status.success() {
    Ok(())
  } else {
    bail!("{}", String::from_utf8_lossy(&output.stderr))
  }
}

/*
 *
 * pub fn get_changed_files()...
 * `gitstatus` gives something like
 *   "M src/tui/app.rs"
 *   "M src/tui/git.rs"
 * 
 * could be formulized as
 *   "XY <path>"
 *
 *   X: git index
 *   Y: git worktree
 *   
 *   see "git index vs worktree"
 *
 */
