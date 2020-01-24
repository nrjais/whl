use crate::config::Command;
use crate::Result;
use std::process::{Command as StdCommand, ExitStatus};

pub struct Executor;

impl Executor {
  pub fn new() -> Self {
    Executor
  }

  pub fn execute(&self, cmd: &Command, dir: &str) -> Result<ExitStatus> {
    match cmd {
      Command::Bin(c) => Self::spawn(c, dir),
      //TODO: Handle script execution
      Command::Script(s) => Self::spawn(s.get(0).unwrap(), dir),
    }
  }

  fn spawn(c: &String, dir: &str) -> Result<ExitStatus> {
    let mut iterator = c.split(" ").filter(|s| !s.is_empty());
    let mut cmd = StdCommand::new(iterator.next().unwrap());
    cmd.current_dir(dir);
    for p in iterator {
      cmd.arg(p);
    }

    Ok(cmd.spawn()?.wait()?)
  }
}
