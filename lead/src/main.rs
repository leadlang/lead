#![allow(unused)]

use std::{env::args, panic, io::{Write, stderr}, process};
use chalk_rs::Chalk;

mod app;
mod docs;

mod help;

pub(crate) mod metadata;

/// The main entry point of the program.
///
/// This function will be called when the program starts, and is where program execution begins.
///
#[tokio::main]
async fn main() { 
  println!("⚠️ Under Construction ⚠️");

  let mut chalk: Chalk = Chalk::new();

  panic::set_hook(Box::new(|info| {
    let mut chalk = Chalk::new();

    let info_pay = info.payload();

    let mut err = stderr();

    let err_str = chalk.red().bold().string(&"-------------------------\n    An error occured!\n-------------------------");

    let _ = err.write_all(err_str.as_bytes());
    let _ = err.write_all(b"\n");

    if let Some(s) = info_pay.downcast_ref::<&str>() {
      let _ = err.write_all(format!("Error: {s}\n").as_bytes());
    } else if let Some(s) = info_pay.downcast_ref::<String>() {
      let _ = err.write_all(format!("Error: {s}\n").as_bytes());
    } else {
      let _ = err.write_all(b"Error: Unknown");
    }

    let err_str = chalk.red().bold().string(&"\n----- TRACE -------------");

    let _ = err.write_all(err_str.as_bytes());
    let _ = err.write_all(b"\n");

    let loc = info.location().map_or("".to_string(), |x| x.to_string());
    let _ = err.write_all(loc.as_bytes());

    let err_str = chalk.red().bold().string(&"\n----- FILE AN ISSUE -----");

    let _ = err.write_all(err_str.as_bytes());
    let _ = err.write_all(b"\nIf you are unable to understand the error, or if its some internal error, file an issue at: https://github.com/leadlang/lead/issues\n\n");

    let _ = err.flush();

    process::exit(1);
  }));

  let mut args: Vec<String> = args().collect();

  let cmd0: &str = &args[1];

  match cmd0 {
    "--prod" => {
      app::run(&args[1..], &mut chalk);
    }
    "run" => {
      app::run(&args[2..], &mut chalk);
    }
    "docs" => {
      let args = args.drain(2..).collect::<Vec<_>>();
      docs::run_docs(&args);
    }
    e => {
      if e != "help" {
        println!("Unknown command: {}", e);
      }

      help::help();
    }
  }
}
