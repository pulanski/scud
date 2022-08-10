use::colored::Colorize;
use std::process::Command;

pub fn log_staged_status(dry_run: bool) {
  println!(
    "{}\n",
    "Repository status:".yellow()
  );

  let status_raw = Command::new("git")
    .arg("status")
    .arg("--short")
    .output()
    .expect("Failed to get git status").stdout;

  let mut status = String::from_utf8_lossy(&status_raw).to_string();

  if dry_run {
    status = status.replace("??", "A");
  }

  print!("{}", status.green());
}