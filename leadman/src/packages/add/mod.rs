use std::{future::IntoFuture, sync::Arc, time::Duration};

use indicatif::ProgressBar;
use tokio::{spawn, sync::oneshot::{self, error::TryRecvError, Sender}, time::sleep};

use super::utils::spinner_style;

mod resolve;

use resolve as res;

async fn resolve(pkg: Arc<String>, bar: ProgressBar, tx: Sender<()>) -> (String, String) {
  let pkg: &str = &pkg;

  let (source, pkg) = pkg.split_once(":").unwrap_or(("gh", pkg));

  let (pkg, version) = pkg.split_once("@").unwrap_or((pkg, "latest"));

  let (owner, repo) = pkg
    .split_once("/")
    .expect("Please follow the correct format! [source:]owner/repo[@version]");

  res::resolve(source, owner, repo, version, bar, tx).await
}

pub async fn install(pkg: Arc<String>, bar: ProgressBar) -> (String, String) {
  bar.set_style(spinner_style());
  bar.set_message(format!("Resolving {}...", &pkg));

  let (tx, mut rx) = oneshot::channel::<()>();

  {
    let bar = bar.clone();
    spawn(async move {
      loop {
        match rx.try_recv() {
          Err(e) => match e {
            TryRecvError::Closed => break,
            _ => {}
          }
          _ => break
        };

        sleep(Duration::from_millis(20)).await;
        bar.tick();
      }
    });
  }

  resolve(pkg, bar, tx).await
}
