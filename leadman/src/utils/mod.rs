use std::{
  fmt::{self, Display, Formatter},
  fs,
  path::{Path, PathBuf},
  process::Command,
  sync::LazyLock,
  time::{SystemTime, UNIX_EPOCH},
};

use reqwest::{Client, ClientBuilder};
use serde::{Deserialize, Serialize};

use crate::{BUILD, LEAD_ROOT_DIR};

#[cfg(not(windows))]
pub mod unix;

#[cfg(windows)]
pub mod windows;

pub async fn postinstall(path: &str) {
  #[cfg(not(windows))]
  return unix::postinstall(path).await;

  #[cfg(windows)]
  return windows::postinstall(path).await;
}

pub static CLIENT: LazyLock<Client> = LazyLock::new(|| {
  ClientBuilder::new()
    .user_agent("Lead Programming / Leadman")
    .build()
    .unwrap()
});

static RELEASES: &str = "https://api.github.com/repos/leadlang/lead/releases";

#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseData {
  pub tag_name: String,
  pub assets: Vec<Asset>,
  pub prerelease: bool,
}

impl Display for ReleaseData {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.tag_name)
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
  pub name: String,
  pub browser_download_url: String,
  pub size: u64,
}

#[allow(dead_code, unused_mut)]
pub fn bashrc() -> PathBuf {
  let mut home = dirs::home_dir().unwrap();

  #[cfg(target_os = "macos")]
  home.push(".bash_profile");

  #[cfg(target_os = "linux")]
  home.push(".bashrc");

  home
}

#[allow(dead_code)]
pub fn chmod(path: &str) {
  Command::new("chmod")
    .args(["777", path])
    .spawn()
    .unwrap()
    .wait()
    .unwrap();
}

pub async fn get_latest_pre(data: Vec<ReleaseData>) -> (ReleaseData, ReleaseData) {
  let mut latest: Option<ReleaseData> = None;
  let mut pre: Option<ReleaseData> = None;

  for version in data {
    if latest.is_none() && !version.prerelease {
      latest = Some(version);
    } else if pre.is_none() && version.prerelease {
      pre = Some(version);
    } else if latest.is_some() && pre.is_some() {
      break;
    }
  }

  (
    latest.expect("No latest version found!"),
    pre.expect("No prerelease found!"),
  )
}

pub async fn get_release(tag: &str) -> ReleaseData {
  let release = CLIENT
    .get(format!("{}/{}", RELEASES, tag))
    .send()
    .await
    .expect("Something went wrong!")
    .json::<ReleaseData>()
    .await
    .expect("Something went wrong while parsing it!");

  release
}

pub async fn get_releases() -> Vec<ReleaseData> {
  let release = CLIENT
    .get(RELEASES)
    .send()
    .await
    .expect("Something went wrong!")
    .json::<Vec<ReleaseData>>()
    .await
    .expect("Something went wrong while parsing it!");

  release
}

pub fn last_update_check_file() -> u64 {
  let data = fs::read_to_string(format!("{}/versions/last_update_check", &*LEAD_ROOT_DIR))
    .unwrap_or_default();

  data.parse().unwrap_or(0)
}

pub fn set_update_check(now: u64) {
  let _ = fs::write(
    format!("{}/versions/last_update_check", &*LEAD_ROOT_DIR),
    format!("{}", now),
  );
}

pub async fn check_update() -> bool {
  let now = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_secs();

  if last_update_check_file() < now {
    set_update_check(now + 3600);

    let Some(res) = CLIENT
      .get("https://github.com/leadlang/lead/releases/latest/download/build")
      .send()
      .await
      .map(|x| x.bytes())
      .ok()
    else {
      return false;
    };

    let bytes = res.await.unwrap();

    let bytes = bytes.to_vec();

    return String::from_utf8_lossy(&bytes)
      .parse::<u64>()
      .map_or(false, |x| x > BUILD);
  }

  false
}

pub fn list_versions() -> Vec<String> {
  fs::read_dir(format!("{}/versions", &*LEAD_ROOT_DIR))
    .unwrap()
    .map(|x| x.unwrap())
    .filter(|x| x.metadata().unwrap().is_dir())
    .map(|x| x.file_name().into_string().unwrap())
    .collect()
}

pub async fn copy_dir<T: AsRef<str>, U: AsRef<str>>(src: T, dest: U) {
  use tokio::fs; 

  let src = src.as_ref();
  let dest = dest.as_ref();

  let _ = fs::create_dir(&dest).await;

  let mut dir = fs::read_dir(src).await.expect("Unable to read dir");

  while let Some(entry) = dir.next_entry().await.expect("Error") {
    let name = entry.file_name();
    let name = name.into_string().expect("Unable to convert name into valid utf8 string");

    let meta = entry.metadata().await.expect("Unable to read metadata");

    if meta.is_dir() {
      Box::pin(copy_dir(format!("{src}/{name}"), format!("{dest}/{name}"))).await;
    } else if meta.is_file() {
      fs::copy(format!("{src}/{name}"), format!("{dest}/{name}")).await;
    }
  }
}