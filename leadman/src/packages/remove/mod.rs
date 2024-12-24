use indicatif::ProgressBar;
use tokio::time::sleep;
use std::{sync::Arc, time::Duration};

use super::{metadata::{Dependency, Metadata}, utils::spinner_style};

pub async fn remove(meta: Arc<Metadata>, pkg: Arc<String>, bar: ProgressBar) {
  bar.set_style(spinner_style());
  bar.set_message(format!("Removing {}...", &pkg));

  let pkg: &String = &pkg;
  
  let Dependency { version, .. } = meta.dependencies.get(pkg).expect("Unknown package");

  let version: &str = &version;

  for _ in 1..100 {
    bar.tick();
    sleep(Duration::from_millis(20)).await;
  }

}
