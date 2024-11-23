use std::{fs, time::Duration};

use chalk_rs::Chalk;
use indicatif::ProgressBar;
use tokio::time::sleep;

use crate::LEAD_ROOT_DIR;

pub async fn clear(chalk: &mut Chalk) {
  chalk.green().print(&">");
  println!(" Clearing cache");

  let root = &*LEAD_ROOT_DIR;

  let spinner = ProgressBar::new_spinner()
    .with_message("Please wait...");

  spinner.enable_steady_tick(Duration::from_millis(20));

  fs::remove_dir_all(format!("{}/temp", root)).expect("Unable to process IO action");

  sleep(Duration::from_secs(3)).await;

  fs::create_dir_all(format!("{}/temp", root)).expect("Unable to process IO action");

  spinner.finish_and_clear();

  chalk.green().println(&"> Cache cleared");
}