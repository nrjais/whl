use crate::config::Command;
use crate::Result;
use std::process::{Command as StdCommand, ExitStatus};

pub struct Executor;

impl Executor {
  pub fn new() -> Self {
    Executor
  }

  pub fn execute(&self, cmd: &Command, dir: &Option<String>) -> Result<ExitStatus> {
    match cmd {
      Command::Bin(c) => Self::spawn(c, dir),
      //TODO: Handle script execution
      Command::Script(s) => Self::spawn(s.get(0).unwrap(), dir),
    }
  }

  fn spawn(c: &String, dir: &Option<String>) -> Result<ExitStatus> {
    let mut cmd = StdCommand::new("bash");

    if let Some(dir) = dir {
      cmd.current_dir(dir);
    }

    Ok(cmd.arg("-c").arg(c).spawn()?.wait()?)
  }
}
