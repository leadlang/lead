use std::{path::PathBuf, process::Command, sync::LazyLock};

use reqwest::{Client, ClientBuilder};
use serde::{Deserialize, Serialize};

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

static CLIENT: LazyLock<Client> = LazyLock::new(|| {
  ClientBuilder::new()
    .user_agent("Lead Programming / Leadman")
    .build()
    .unwrap()
});

static RELEASES: &str = "https://api.github.com/repos/ahq-softwares/lead/releases";

#[derive(Debug, Serialize, Deserialize)]
pub struct ReleaseData {
  pub tag_name: String,
  pub assets: Vec<Asset>,
  pub prerelease: bool,
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
