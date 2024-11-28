#![allow(unused)]

use chrono::{Datelike, Local};
use indicatif::ProgressBar;
use tokio::time::sleep;
use utils::check_update;
use std::{env::{self, args}, io::{stderr, Write}, panic, sync::LazyLock, time::Duration};

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

  panic::set_hook(Box::new(|info| {
    let mut chalk = Chalk::new();

    let info_pay = info.payload();
    
    let mut err = stderr();

    let err_str = chalk
      .red()
      .bold()
      .string(&"An error occured!");

    let _ = err.write_all(err_str.as_bytes());
    let _ = err.write_all(b"\n");

    if let Some(s) = info_pay.downcast_ref::<&str>() {
      let _ = err.write_all(format!("Error: {s}\n").as_bytes());
    } else if let Some(s) = info_pay.downcast_ref::<String>() {
      let _ = err.write_all(format!("Error: {s}\n").as_bytes());
    } else {
      let _ = err.write_all(b"Error: Unknown");
    }

    let err_str = chalk
      .red()
      .bold()
      .string(&"\n----- TRACE -------------");

    let _ = err.write_all(err_str.as_bytes());
    let _ = err.write_all(b"\n");

    let loc = info.location().map_or("".to_string(), |x| x.to_string());
    let _ = err.write_all(loc.as_bytes());

    let err_str = chalk
    .red()
    .bold()
    .string(&"\n----- FILE AN ISSUE -----");

    let _ = err.write_all(err_str.as_bytes());
    let _ = err.write_all(b"\nIf you are unable to understand the error, or if its some internal error, file an issue at: https://github.com/ahq-softwares/lead/issues\n\n");

    let _ = err.flush();
  }));

  let _: String = None.expect("Something went wrong");

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
