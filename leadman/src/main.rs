#![allow(unused)]

use chrono::{Datelike, Local};
use indicatif::ProgressBar;
use tokio::time::sleep;
use utils::check_update;
use std::{env::{self, args}, sync::LazyLock, time::Duration};

use chalk_rs::Chalk;

pub(crate) mod utils;

mod help;
use help::help;

mod create;
use create::create;

mod clear;
pub(crate) mod install;

pub static LEAD_ROOT_DIR: LazyLock<String> = LazyLock::new(|| {
  env::var("LEAD_HOME").expect("LEAD_HOME environment variable not set! Please reinstall the application")
});

pub static TARGET: &'static str = env!("TARGET");

static BUILD: u64 = include!("../build");

fn prefix(chalk: &mut Chalk) {
  chalk
    .yellow()
    .bold()
    .println(&format!("LeadMan v{} : Build {BUILD}", env!("CARGO_PKG_VERSION")));
  chalk.default_color().bold().println(&format!(
    "©️ {} - Lead Programming Language \n",
    Local::now().year()
  ));
}

fn show_update_message(chalk: &mut Chalk) {
  chalk
    .blue()
    .bold()
    .println(&r#"----------------------------------------------------------------------------
| A newer build of leadman is available! Please update to the latest build |
|                                                                          |
| To update, run:                                                          |
|   leadman self-update                                                    |
----------------------------------------------------------------------------"#);
}

#[tokio::main]
async fn main() {
  let mut chalk = Chalk::new();

  let args = args().collect::<Vec<_>>();

  prefix(&mut chalk);

  let bar = ProgressBar::new_spinner()
    .with_message("Checking for self update...");

  bar.enable_steady_tick(Duration::from_millis(20));

  let update = check_update().await;

  bar.finish_and_clear();

  if update {
    show_update_message(&mut chalk);
  }

  if args.len() < 2 {
    help();
    return;
  }

  match args[1].as_str() {
    "help" => {
      help();
    }
    "create" => {
      create(&mut chalk).await;
    }
    "clear" => {
      clear::clear(&mut chalk).await;
    }
    e => {
      chalk.red().bold().println(&format!("Unknown command: {e}"));
      help();
    }
  }
}
