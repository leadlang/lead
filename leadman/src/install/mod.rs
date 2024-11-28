use std::process;

use chalk_rs::Chalk;

use crate::{
  utils::{ReleaseData, CLIENT}, TARGET
};

fn print_err<T: ToString>(s: T, chalk: &mut Chalk) {
  chalk.yellow().bold().println(&s);

  process::exit(1);
}

pub async fn install(release: ReleaseData, chalk: &mut Chalk) {
  let ReleaseData { tag_name, .. } = release;
  
  let base = format!("https://github.com/ahq-softwares/lead/releases/download/{}", &tag_name);

  let build = format!("{base}/build");

  let build = CLIENT.get(&build)
    .send()
    .await
    .expect("This version cannot be installed!")
    .text()
    .await
    .expect("This version cannot be installed");

  match build.as_str() {
    "1" => {
      
    }
    _ => {
      print_err("This version of lead language cannot be installed by leadman", chalk);
    }
  }

  let bin = format!("{base}/binaries_{}", TARGET);
}