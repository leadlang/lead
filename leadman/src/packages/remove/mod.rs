use indicatif::ProgressBar;
use tokio::{fs, time::sleep};
use std::{sync::Arc, time::Duration};

use sha256::digest;

use super::{metadata::Metadata, utils::spinner_style, MetaPtr};

pub async fn remove(meta: Arc<MetaPtr>, pkg: Arc<String>, bar: ProgressBar) -> Arc<String> {
  bar.set_style(spinner_style());
  bar.set_message(format!("Removing {}...", &pkg));

  let meta = unsafe { &*meta.0 };

  let ret_pkg = pkg.clone();

  let pkg: &String = &pkg;
  
  let version = meta.dependencies.get(pkg).expect("Unknown package");

  let version: &str = &version;

  let digest = digest(format!("{pkg}@{version}"));

  let _ = fs::remove_dir_all(format!("./.pkgcache/{digest}")).await;

  for _ in 1..5 {
    bar.tick();
    sleep(Duration::from_millis(20)).await;
  }

  ret_pkg
}
