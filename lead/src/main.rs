#![feature(slice_ptr_len)]
mod app;

use clap::{arg, Parser, Subcommand};

#[derive(Clone, Subcommand, Debug)]
pub enum Command {
  /// Run a Lead app
  Run {
    /// Should it be production?
    #[arg(short, long)]
    prod: bool,
    #[arg(short, long)]
    dir: Option<String>,
  },
  /// Create a new Lead app project
  New { dir: String },
}

#[derive(Parser)]
pub struct Arguments {
  #[clap(subcommand)]
  cmd: Option<Command>,
}

fn main() {
  let subcommand: Command = Arguments::parse().cmd.unwrap_or(Command::Run {
    prod: false,
    dir: None,
  });

  match subcommand {
    Command::Run { prod, dir } => {
      let dir = dir.unwrap_or(".".into());
      app::run(dir, prod);
    }
    _ => todo!(),
  }
}
