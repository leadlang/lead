#[cfg(windows)]
mod windows;

#[cfg(target_os="linux")]
mod linux;

use std::{io::Cursor, path::Path};

use indicatif::ProgressBar;
use reqwest::{Client, ClientBuilder};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Release {
  pub tag_name: String,
  pub assets: Vec<Asset>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
  pub name: String,
  pub browser_download_url: String,
}

lazy_static! {
  pub static ref CLIENT: Client = ClientBuilder::new()
    .user_agent("lead lang/init")
    .build()
    .unwrap();
}

pub async fn get_bin_zip() -> Release {
  CLIENT.get("https://api.github.com/repos/AHQ-Softwares/lead/releases/latest")
    .send()
    .await
    .unwrap()
    .json::<Release>()
    .await
    .unwrap()
}

pub async fn download_install_lead(url: &str, dir: &str) {
  let mut resp = CLIENT.get(url)
    .send()
    .await
    .unwrap();
  let len = resp.content_length().unwrap_or(0);

  let p_bar = ProgressBar::new(len);


  let mut file = vec![];
  while let Some(chunk) = resp.chunk().await.unwrap() {
    p_bar.inc(chunk.len() as u64);
    file.append(&mut chunk.to_vec());
  }

  p_bar.abandon();

  zip_extract::extract(Cursor::new(file), &Path::new(dir), false).unwrap();
}

pub async fn postinstall(path: &str) {
  #[cfg(windows)]
  windows::postinstall(path).await;
  #[cfg(target_os = "linux")]
  linux::postinstall(path).await;
}