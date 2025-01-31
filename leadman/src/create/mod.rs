use std::{
  env::{self, current_exe},
  fs,
  path::PathBuf,
};

use lealang_chalk_rs::Chalk;
use inquire::Select;

use crate::{
  install::{install, set_current, set_nightly, set_stable},
  utils::{get_latest_pre, get_releases, postinstall},
};

use dirs::home_dir;

pub async fn create(chalk: &mut Chalk) {
  let mut dir = home_dir().unwrap();
  dir.push("leadLang");

  chalk.green().print(&">");
  println!(" Lead lang will install in {}", dir.display());

  let releases = get_releases().await;

  let (latest, prerelease) = get_latest_pre(releases).await;

  let stable = env::var("LEAD_CHANNEL")
    .unwrap_or_else(|_| {
      Select::new(
        "Select your channel",
        vec![
          format!("Stable ({})", &latest.tag_name),
          format!("Nightly ({})", &prerelease.tag_name),
        ],
      )
      .prompt()
      .expect("You must respond")
    })
    .contains("Stable");

  let version = if stable { latest } else { prerelease };

  chalk.green().print(&">");

  let _ = fs::remove_dir_all(&dir);
  fs::create_dir_all(&dir).expect("Unable to process IO action");

  dir.push("versions");

  fs::create_dir_all(&dir).expect("Unable to process IO action");

  dir.pop();
  dir.push("lead.ps1");

  fs::write(&dir, include_str!("../lead.ps1")).expect("Unable to add lead.ps1");

  dir.pop();
  dir.push("lead");

  fs::write(&dir, include_str!("../lead")).expect("Unable to add lead shell script");

  dir.pop();
  dir.push("leadc.ps1");

  fs::write(&dir, include_str!("../leadc.ps1")).expect("Unable to add lead.ps1");

  dir.pop();
  dir.push("leadc");

  fs::write(&dir, include_str!("../leadc")).expect("Unable to add lead shell script");

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

  let home_dir = dir.to_str().unwrap();
  install(&version, home_dir, chalk).await;

  chalk.green().print(&">");
  println!(" Performing post install steps...");

  set_current(&version.tag_name, home_dir);

  if stable {
    set_stable(&version.tag_name, home_dir);
  } else {
    set_nightly(&version.tag_name, home_dir);
  }

  let dir = dir.to_str().unwrap();

  postinstall(&dir).await;

  chalk.green().print(&">");
  println!(" Lead Language has been installed successfully!");
}

pub fn replicate(dest: &PathBuf) {
  let src = current_exe().unwrap();

  fs::copy(src, dest).expect("Unable to process IO Action");
}
