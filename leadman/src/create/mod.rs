use std::{env::current_exe, fs, path::PathBuf};

use chalk_rs::Chalk;
use inquire::Select;

use crate::{install::install, utils::{get_latest_pre, get_releases, postinstall}};

use dirs::home_dir;

pub async fn create(chalk: &mut Chalk) {
  let mut dir = home_dir().unwrap();
  dir.push("leadLang");

  chalk.green().print(&">");
  println!(" Lead lang will install in {}", dir.display());

  let releases = get_releases().await;

  let (latest, prerelease) = get_latest_pre(releases).await;

  let stable = Select::new("Select your channel", vec![format!("Stable ({})", &latest.tag_name), format!("Prerelease ({})", &prerelease.tag_name)])
    .prompt()
    .expect("You must respond")
    .contains("Stable");

  let version = if stable {
    latest
  } else {
    prerelease
  };

  chalk.green().print(&">");

  let _ = fs::remove_dir_all(&dir);
  fs::create_dir_all(&dir).expect("Unable to process IO action");

  dir.push("versions");

  fs::create_dir_all(&dir).expect("Unable to process IO action");

  dir.pop();
  dir.push("current");

  fs::create_dir_all(&dir).expect("Unable to process IO action");

  dir.pop();
  dir.push("temp");

  fs::create_dir_all(&dir).expect("Unable to process IO action");

  dir.pop();
  
  #[cfg(windows)]
  dir.push("leadman.exe");

  #[cfg(not(windows))]
  dir.push("leadman");

  replicate(&dir);

  dir.pop();

  println!(" Installing Lead Language v{}", &version.tag_name);

  install(version, chalk).await;

  let dir = dir.to_str().unwrap();

  postinstall(&dir).await;
}

pub fn replicate(dest: &PathBuf) {
  let src = current_exe().unwrap();

  fs::copy(src, dest).expect("Unable to process IO Action");
}