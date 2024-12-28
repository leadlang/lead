use std::{io::{stderr, Write}, path::Path, process::Stdio};

use crate::packages::metadata::Script;

use indicatif::MultiProgress;
use tokio::{io::AsyncReadExt, process::Command};

pub async fn run_script<P: AsRef<Path>>(script: &Script, cwd: P, prog: &MultiProgress) {
  #[cfg(windows)]
  let shell = "powershell.exe";

  #[cfg(unix)]
  let shell = "sh";

  let mut child = Command::new(shell)
    .arg(if cfg!(windows) {
      &script.windows
    } else {
      &script.unix
    })
    .current_dir(cwd)
    .stdin(Stdio::inherit())
    .stdout(Stdio::inherit())
    .stderr(Stdio::piped())
    .spawn()
    .expect("Error while running script");

  let mut err = child.stderr.take().unwrap();

  let success = child.wait().await.expect("Error while waiting for process").success();

  if !success {
    prog.suspend(move || {
      let mut buf = vec![];
      err.read_buf(&mut buf);

      let _ = stderr().write_all(&buf);
    });
  }
}