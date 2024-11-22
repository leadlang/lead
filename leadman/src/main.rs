use chrono::{Datelike, Local};
use std::env::args;

use chalk_rs::Chalk;

pub(crate) mod utils;

mod help;
use help::help;

mod create;
use create::create;

fn prefix(chalk: &mut Chalk) {
  chalk
    .yellow()
    .bold()
    .println(&format!("LeadMan v{}", env!("CARGO_PKG_VERSION")));
  chalk.default_color().bold().println(&format!(
    "©️ {} - Lead Programming Language \n",
    Local::now().year()
  ));
}

#[tokio::main]
async fn main() {
  let mut chalk = Chalk::new();

  let args = args().collect::<Vec<_>>();

  prefix(&mut chalk);
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
    e => {
      chalk.red().bold().println(&format!("Unknown command: {e}"));
      help();
    }
  }
}
