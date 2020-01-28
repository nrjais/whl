use crate::Result;
use std::process::{Command, ExitStatus};

pub struct Executor;

impl Executor {
  pub fn new() -> Self {
    Executor
  }

  pub fn execute(&self, cmd: &str, dir: &Option<String>) -> Result<ExitStatus> {
    let mut command = Command::new("bash");

    if let Some(dir) = dir {
      command.current_dir(dir);
    }

    Ok(command.arg("-c").arg(cmd).spawn()?.wait()?)
  }
}
