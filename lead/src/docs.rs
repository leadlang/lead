use std::{env, process::{self, Command}};

pub fn run_docs(args: &[String]) {
  let home = env::var("LEAD_HOME").expect("Broken Lead Installation, LEAD_HOME is necessary");
  let version = env!("CARGO_PKG_VERSION");

  #[cfg(unix)]
  let lead_docs = format!("{home}/versions/{version}/lead_docs");

  #[cfg(windows)]
  let lead_docs = format!("{home}\\versions\\{version}\\lead_docs.exe");

  let status = Command::new(lead_docs).args(args).spawn().expect("Unable to run lead docs").wait().expect("Unable to run").success();

  process::exit(if status { 0 } else { 1 })
}
