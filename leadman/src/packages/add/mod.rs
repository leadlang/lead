use std::{future::IntoFuture, sync::Arc, time::Duration};

use indicatif::ProgressBar;
use tokio::time::sleep;

use super::{metadata::LibraryMeta, utils::{progress_bar, spinner_style}};

mod resolve;

use resolve as res;

async fn resolve(pkg: Arc<String>, bar: ProgressBar) -> (LibraryMeta, String, String) {
  let pkg: &str = &pkg;

  let (source, pkg) = pkg.split_once(":").unwrap_or(("gh", pkg));

  let (pkg, version) = pkg.split_once("@").unwrap_or((pkg, "latest"));

  let (owner, repo) = pkg
    .split_once("/")
    .expect("Please follow the correct format! [source:]owner/repo[@version]");

  bar.set_message("Downloading...");
  bar.set_style(progress_bar());
  res::resolve(source, owner, repo, version, bar).await
}

pub async fn install(pkg: Arc<String>, bar: ProgressBar) -> (LibraryMeta, String, String) {
  bar.set_style(spinner_style());
  bar.set_message(format!("Resolving {}...", &pkg));

  for _ in 1..100 {
    bar.tick();
    sleep(Duration::from_millis(20)).await;
  }

  resolve(pkg, bar).await
}
