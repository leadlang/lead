use std::sync::Arc;

use indicatif::MultiProgress;
use serde_json::from_str;
use sha256::digest;
use tokio::fs;

use crate::{utils::copy_dir, TARGET};

use super::{metadata::LibraryMeta, MetaPtr};

mod script;

pub enum ScriptClass {
  Pre,
  Post
}

pub async fn run_script(meta: Arc<MetaPtr>, class: ScriptClass, print: MultiProgress) {
  let meta = unsafe { &*meta.0 };

  for (k, v) in &meta.dependencies {
    let hash = digest(format!("{k}@{v}"));

    let cwd = format!("./.pkgcache/{hash}");

    let meta: LibraryMeta = from_str(&fs::read_to_string(format!("{cwd}/pkgcache")).await.expect("Unable to read metadata")).expect("Invalid Metadata");

    let script = match class {
      ScriptClass::Pre => meta.preinstall,
      ScriptClass::Post => meta.postinstall
    };

    if let Some(script) = script {
      script::run_script(&script, cwd, &print).await;
    }
  }
}

pub async fn link(meta: Arc<MetaPtr>, print: MultiProgress) {
  let meta = unsafe { &*meta.0 };

  let _ = fs::remove_dir_all("./lib").await;
  fs::create_dir_all("./lib").await.expect("Unable to rebuild links");

  for (k, v) in &meta.dependencies {
    let hash = digest(format!("{k}@{v}"));

    let platform_cwd = format!("./.pkgcache/{hash}/lib/{}", TARGET);

    if !fs::metadata(&platform_cwd).await.expect("Unable to get metadata").is_dir() {
      panic!("No Build for {k}@{v} is availble for {TARGET}");
    }

    copy_dir(&platform_cwd, format!("./lib/{hash}")).await;

    let doc = format!("./lib/{hash}/docs");

    match fs::metadata(&doc).await {
      Ok(m) => if m.is_file() {
        print.suspend(|| {
          println!("      ⚠️  Replacing file named docs in {k}@{v} for target {TARGET}");
        });
        fs::remove_file(&doc).await;
      }
      _ => {
        print.suspend(|| {
          println!("      ⚠️  No docs found for {k}@{v} for target {TARGET}, using generic docs");
        });
        fs::copy(format!("./.pkgcache/{hash}/docs"), doc).await.expect("Unable to copy docs");
      }
    }
  }
}