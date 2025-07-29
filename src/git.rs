use std::process::{Command};

pub fn run_git(command: &str) -> Result<String, String> {
  let output = Command::new("git")
    .arg(command)
    .output()
    .map_err(|e| e.to_string())?;

  if output.status.success() {
    String::from_utf8(output.stdout).map_err(|e| e.to_string())
  } else {
    Err(String::from_utf8_lossy(&output.stderr).into())
  }
}
