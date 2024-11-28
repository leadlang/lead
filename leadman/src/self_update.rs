use std::{
  env, fs,
  process::{self, Command},
};

use crate::{utils::CLIENT, LEAD_ROOT_DIR, TARGET};

pub async fn update() {
  let leadman = CLIENT
    .get(format!(
      "https://api.github.com/repos/ahq-softwares/lead/releases/download/latest/leadman_{}",
      TARGET
    ))
    .send()
    .await
    .unwrap()
    .bytes()
    .await
    .unwrap();

  let lead = format!("{}/new_leadman.exe", env::temp_dir().display());
  fs::write(&lead, leadman);

  Command::new(lead).arg("replace").spawn().unwrap();

  process::exit(0);
}
