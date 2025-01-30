use std::sync::Arc;

use indicatif::MultiProgress;
use serde_json::from_str;
use sha256::digest;
use tokio::fs;

use crate::{utils::copy_dir, OTHER_TARGET, TARGET};

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

  let is0 = meta.pkver == 0;

  let _ = fs::remove_dir_all("./.lead_libs").await;
  fs::create_dir_all("./.lead_libs").await.expect("Unable to rebuild links");

  for (k, v) in &meta.dependencies {
    let hash = digest(format!("{k}@{v}"));

    let meta = format!("./.pkgcache/{hash}/pkgcache");

    let resp: LibraryMeta = from_str(&fs::read_to_string(meta).await.expect("Error reading")).expect("Unable to parse");

    let mut platform_cwd = format!("./.pkgcache/{hash}/lib/{}", TARGET);

    if !fs::metadata(&platform_cwd).await.expect("Unable to get metadata").is_dir() {
      platform_cwd = format!("./.pkgcache/{hash}/lib/{}", *OTHER_TARGET);
      if !fs::metadata(&platform_cwd).await.expect("Unable to get metadata").is_dir() {
        panic!("No Build for {k}@{v} is availble for {TARGET}");
      }
    }

    copy_dir(&platform_cwd, format!("./.lead_libs/{hash}")).await;
    fs::write(format!("./.lead_libs/{hash}/lead.lookup.lkp"), resp.package).await;

    if is0 {
      
      let doc = format!("./.lead_libs/{hash}/docs");

      match fs::metadata(&doc).await {
        Ok(m) => if m.is_file() {
          print.suspend(|| {
            println!("      ⚠️  Replacing file named docs in {k}@{v} for target {TARGET}");
          });
          fs::remove_file(&doc).await;
          fs::copy(format!("./.pkgcache/{hash}/docs"), doc).await.expect("Unable to copy docs");
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
}