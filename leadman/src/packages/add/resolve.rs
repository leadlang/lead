use std::io::{Cursor, Read};

use indicatif::ProgressBar;
use serde_json::from_str;
use sha256::digest;
use std::fs;
use tokio::task;
use zip::ZipArchive;

use crate::{packages::metadata::LibraryMeta, utils::CLIENT};

pub async fn resolve(source: &str, owner: &str, repo: &str, version: &str, bar: ProgressBar) -> (LibraryMeta, String, String) {
  match source {
    "gh" => {
      let suffix = match version {
        "latest" => "latest/download",
        e => &format!("download/{e}"),
      };

      let url = format!("https://github.com/{owner}/{repo}/releases/{suffix}/leadpkg.zip");

      let mut r = CLIENT
        .get(url)
        .send()
        .await
        .expect("Unable to fetch");

      bar.set_length(r.content_length().unwrap_or(0));

      let mut j = vec![];

      while let Ok(Some(chunk)) = r.chunk().await {
        bar.inc(chunk.len() as u64);
        j.append(&mut chunk.into_iter().collect());
      }

      let j = Cursor::new(j);
      let digest = digest(format!("{source}:{owner}/{repo}@{version}"));

      let pkg = task::spawn_blocking(move || {
        let mut archive = ZipArchive::new(j).expect("Unable to unzip");

        let mut pkg = archive
          .by_name(".pkgcache")
          .expect("Unable to fetch the leadpkg file");

        let mut pkg_buf = String::new();
        pkg
          .read_to_string(&mut pkg_buf)
          .expect("Unable to read the leadpkg file");
        drop(pkg);

        let pkg: LibraryMeta = from_str(&pkg_buf).expect("Unable to parse as Metadata");

        drop(pkg_buf);

        fs::create_dir_all("./.pkgcache").expect("IO Error");

        archive
          .extract(format!("./.pkgcache/{digest}"))
          .expect("Unable to extract the package");

        pkg
      })
      .await
      .unwrap();

      bar.finish_and_clear();

      let name = format!("{source}:{owner}/{repo}");

      (pkg, name, version.into())
    }
    e => {
      panic!("Source {e} not supported!");
    }
  }
}
