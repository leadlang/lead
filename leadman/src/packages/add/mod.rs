use std::{future::IntoFuture, sync::Arc, time::Duration};

use indicatif::ProgressBar;
use tokio::time::sleep;

use super::utils::spinner_style;

pub async fn resolve(pkg: Arc<String>) {
  let pkg: &str = &pkg;
}

pub async fn install(pkg: Arc<String>, bar: ProgressBar) {
  bar.set_style(spinner_style());
  bar.set_message(format!("Resolving {}...", &pkg));

  let task = tokio::spawn(resolve(pkg.clone()));
  
  for _ in 1..1000 {
    bar.tick();
    sleep(Duration::from_millis(20)).await;
  }

  sleep(Duration::from_secs(5)).await;
}