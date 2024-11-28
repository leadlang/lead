use std::{
  fs::{self, File},
  io::{Cursor, Write},
  thread,
  time::Duration,
};

use indicatif::ProgressBar;
use zip::ZipArchive;

use crate::{utils::CLIENT, TARGET};

pub async fn install(tag: &str, lead_home: &str) {
  let download = format!(
    "https://github.com/ahq-softwares/lead/releases/download/{}",
    &tag
  );

  let download = format!("{download}/binaries_{TARGET}.zip");

  let mut resp = CLIENT.get(download).send().await.expect("");

  let len = resp
    .content_length()
    .expect("No response length was provided");

  let bar = ProgressBar::new(len);

  let ret = format!("{lead_home}/versions/{tag}");
  let _ = fs::create_dir_all(&ret);
  let mut file = Cursor::new(vec![]);

  while let Some(bytes) = resp.chunk().await.expect("Unable to get file") {
    bar.inc(bytes.len() as u64);
    file.write_all(&bytes).expect("Unable to write to file");
  }

  bar.finish_and_clear();

  let bar = ProgressBar::new_spinner().with_message("Extracting binary...");

  let handle = thread::spawn(|| {
    ZipArchive::new(file)
      .expect("Unable to open zip file")
      .extract(ret)
      .expect("Unable to extract zip file");
  });

  loop {
    if handle.is_finished() {
      bar.finish_and_clear();
      handle.join().expect("Unable to extract zip file");
      break;
    }
    bar.tick();
    thread::sleep(Duration::from_millis(20));
  }
}
