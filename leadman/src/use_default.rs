use core::panic;
use std::{env, fs};

use inquire::Select;
use lealang_chalk_rs::Chalk;

use crate::{utils::list_versions, LEAD_ROOT_DIR};

pub fn use_default(chalk: &mut Chalk) {
  let channel = get_override();
  let version = get_version(&channel);

  fs::write(
    format!("{}/versions/{}", &*LEAD_ROOT_DIR, &channel),
    &version,
  )
  .unwrap();

  chalk.green().println(&format!(
    "> Successfully set {} as default version for {}",
    version, channel
  ));
}

fn get_version(channel: &str) -> String {
  let mut versions = list_versions();

  if channel == "stable" {
    versions.retain(|x| !x.contains("nightly"));
  } else if channel == "nightly" {
    versions.retain(|x| x.contains("nightly"));
  }

  if versions.is_empty() {
    panic!("No version found for channel {}", channel);
  }

  let mut options = vec![];
  for version in &versions {
    options.push(version);
  }
  env::var("LEAD_VERSION").map_or_else(
    |_| {
      Select::new("Select your version", options)
        .prompt()
        .unwrap()
        .clone()
    },
    |x| {
      if versions.contains(&x) {
        return x;
      }
      panic!("Invalid version {}", x);
    },
  )
}

fn get_override() -> String {
  let file = env::var("LEAD_OVERRIDE").unwrap_or_else(|_| {
    Select::new(
      "Select the channel to modify",
      vec!["current", "stable", "nightly"],
    )
    .prompt()
    .unwrap()
    .into()
  });

  match file.as_str() {
    "current" | "stable" | "nightly" => {
      return file;
    }
    e => {
      panic!("Invalid override {}", e);
    }
  }
}
