use std::{env, fs, process};

use inquire::Select;
use lealang_chalk_rs::Chalk;

use crate::{
  utils::{get_release, get_releases, list_versions, ReleaseData, CLIENT},
  LEAD_ROOT_DIR,
};

mod build_1;

fn print_err<T: ToString>(s: T, chalk: &mut Chalk, kill: bool) {
  if kill { chalk.red() } else { chalk.yellow() }
    .bold()
    .println(&s);

  if kill {
    process::exit(1);
  }
}

pub fn set_current(ver: &str, lead_home: &str) {
  fs::write(format!("{lead_home}/versions/current"), ver).expect("Unable to write to file");
}

pub fn set_stable(ver: &str, lead_home: &str) {
  fs::write(format!("{lead_home}/versions/stable"), ver).expect("Unable to write to file");
}

pub fn set_nightly(ver: &str, lead_home: &str) {
  fs::write(format!("{lead_home}/versions/nightly"), ver).expect("Unable to write to file");
}

pub async fn install_cli(chalk: &mut Chalk) {
  let list = list_versions();

  let mut res = async {
    let id = env::var("LEAD_VERSION").ok()?;

    if list.contains(&id) {
      println!("Version {} is already installed", id);
      process::exit(0);
    }

    Some(get_release(&id).await)
  }
  .await;

  if res.is_none() {
    let data = async {
      let mut releases = get_releases().await;

      releases.retain(|x| !list.contains(&x.tag_name));

      Select::new("Select version to install", releases)
        .prompt()
        .expect("You must select a version")
    }
    .await;

    res = Some(data);
  }

  let res = res.unwrap();

  install(&res, &*LEAD_ROOT_DIR, chalk).await;

  chalk.green().print(&"> ");
  println!("Successfully installed lead language v{}", &res.tag_name);

  chalk.blue().print(&"> ");
  print!("You may run ");
  chalk.green().print(&"leadman use");
  println!(" to set the default version of lead");

  chalk.blue().print(&"> ");
  print!("You may run ");
  chalk.green().print(&format!("lead +{}", &res.tag_name));
  print!(" to use this version of lead without ");
  chalk.green().println(&"leadman use");
}

pub async fn install(release: &ReleaseData, lead_home: &str, chalk: &mut Chalk) {
  let ReleaseData { tag_name, .. } = release;

  let base = format!(
    "https://github.com/leadlang/lead/releases/download/{}",
    &tag_name
  );

  let build = format!("{base}/build");

  let build = CLIENT
    .get(&build)
    .send()
    .await
    .expect("This version cannot be installed!")
    .text()
    .await
    .expect("This version cannot be installed");

  match build.as_str() {
    // Build 6 introduces build version monitoring
    "6" | "7" | "8" | "9" | "10" | "11" | "12" | "13" | "14" | "15" => {
      build_1::install(&tag_name, lead_home).await;
      tokio::fs::write(
        format!("{lead_home}/versions/{tag_name}/.lbuild"),
        build.as_str(),
      )
      .await;
    }
    // Build 1 to 5 are the similar
    "1" | "2" | "3" | "4" | "5" => {
      build_1::install(&tag_name, lead_home).await;
    }
    "0" => {
      print_err(
        "> This version of lead language is made using build 0 which might not work as expected",
        chalk,
        false,
      );
      build_1::install(&tag_name, lead_home).await;
    }
    _ => {
      print_err(
        "This version of lead language cannot be installed by leadman",
        chalk,
        true,
      );
    }
  }
}
