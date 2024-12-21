use std::{
  env, fs,
  process::{self, Command},
};

use crate::{utils::CLIENT, LEAD_ROOT_DIR, TARGET};

pub async fn update() {
  let bin = if cfg!(windows) { ".exe" } else { "" };

  let leadman = CLIENT
    .get(format!(
      "https://github.com/ahq-softwares/lead/releases/latest/download/leadman_{}{bin}",
      TARGET
    ))
    .send()
    .await
    .unwrap()
    .bytes()
    .await
    .unwrap();

  #[cfg(windows)]
  let lead = format!("{}/new_leadman.exe", env::temp_dir().display());

  #[cfg(not(windows))]
  let lead = format!("{}/new_leadman", env::temp_dir().display());

  println!("Lead: {}", &lead);

  fs::write(&lead, leadman);

  #[cfg(not(windows))]
  {
    Command::new("chmod").arg("777").arg(&lead).spawn().unwrap();
  }

  Command::new(lead).arg("replace").spawn().unwrap();

  process::exit(0);
}
