use std::{
  io::{Cursor, Read},
  sync::Arc,
};

use indicatif::ProgressBar;
use serde_json::from_str;
use sha256::digest;
use std::fs;
use tokio::{sync::oneshot::Sender, task};
use zip::ZipArchive;

use crate::{
  packages::{metadata::LibraryMeta, utils::progress_bar},
  utils::CLIENT,
  OTHER_TARGET, TARGET,
};

pub async fn resolve(
  source: &str,
  owner: &str,
  repo: &str,
  version: &str,
  bar: ProgressBar,
  tx: Sender<()>,
) -> (String, String) {
  match source {
    "gh" => {
      let suffix = match version {
        "latest" => "latest/download",
        e => &format!("download/{e}"),
      };

      let url = format!("https://github.com/{owner}/{repo}/releases/{suffix}/leadpkg.zip");

      let mut r = CLIENT.get(url).send().await.expect("Unable to fetch");

      bar.set_message("Downloading...");
      bar.set_style(progress_bar());
      bar.set_length(r.content_length().unwrap_or(0));

      let mut j = vec![];

      while let Ok(Some(chunk)) = r.chunk().await {
        bar.inc(chunk.len() as u64);
        j.append(&mut chunk.into_iter().collect());
      }

      let j = Cursor::new(j);
      let _digest = Arc::new(digest(format!("{source}:{owner}/{repo}@{version}")));

      let dg = _digest.clone();
      let meta = task::spawn_blocking(move || {
        let mut archive = ZipArchive::new(j).expect("Unable to unzip");

        let mut pkg = archive
          .by_name("pkgcache")
          .expect("Unable to fetch the leadpkg file");

        let mut pkg_buf = String::new();
        pkg
          .read_to_string(&mut pkg_buf)
          .expect("Unable to read the leadpkg file");
        drop(pkg);

        let meta: LibraryMeta = from_str(&pkg_buf).expect("Unable to parse as Metadata");

        drop(pkg_buf);

        fs::create_dir_all("./.pkgcache").expect("IO Error");

        archive
          .extract(format!("./.pkgcache/{dg}"))
          .expect("Unable to extract the package");

        meta
      })
      .await
      .unwrap();

      if meta.uses_new {
        let (url, tg) = if meta.platforms.iter().any(|x| x.as_str() == TARGET) {
          (
            Some(format!(
              "https://github.com/{owner}/{repo}/releases/{suffix}/{}.zip",
              &*TARGET
            )),
            TARGET,
          )
        } else if meta.platforms.iter().any(|x| &x.as_str() == &*OTHER_TARGET) {
          (
            Some(format!(
              "https://github.com/{owner}/{repo}/releases/{suffix}/{}.zip",
              &*OTHER_TARGET
            )),
            *OTHER_TARGET,
          )
        } else {
          (None, TARGET)
        };

        if let Some(url) = url {
          bar.reset();
          let mut r = CLIENT.get(url).send().await.expect("Unable to fetch");

          bar.set_message("Downloading...");
          bar.set_style(progress_bar());
          bar.set_length(r.content_length().unwrap_or(0));

          let mut j = vec![];

          while let Ok(Some(chunk)) = r.chunk().await {
            bar.inc(chunk.len() as u64);
            j.append(&mut chunk.into_iter().collect());
          }

          let j = Cursor::new(j);

          task::spawn_blocking(move || {
            let mut archive = ZipArchive::new(j).expect("Unable to unzip");

            archive
              .extract(format!("./.pkgcache/{_digest}/lib/{tg}"))
              .expect("Unable to extract the package");

            meta
          })
          .await
          .unwrap();
        }
      }

      tx.send(());

      bar.finish_and_clear();

      let name = format!("{source}:{owner}/{repo}");

      (name, version.into())
    }
    e => {
      panic!("Source {e} not supported!");
    }
  }
}
