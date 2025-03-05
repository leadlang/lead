use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use tokio::fs;

const METADATA_VER: u16 = 1;

#[derive(Debug, Serialize, Deserialize)]
pub struct LibraryMeta {
  pub package: String,
  pub version: String,
  #[serde(default)]
  pub description: String,
  #[serde(default)]
  pub authors: Vec<String>,
  #[serde(default)]
  pub keywords: Vec<String>,
  /// We'll ignore it
  #[serde(default)]
  pub platforms: Vec<String>,
  #[serde(default)]
  pub uses_new: bool,

  pub preinstall: Option<Script>,
  pub compile: Option<Script>,
  pub postinstall: Option<Script>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Script {
  pub unix: String,
  pub windows: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
  #[serde(rename = "$schema")]
  pub schema: String,
  pub entry: String,
  pub name: String,
  pub version: String,
  pub description: String,
  pub authors: Vec<String>,
  pub keywords: Vec<String>,
  pub dependencies: HashMap<String, String>,

  pub pkver: u16,

  #[serde(rename = "allowFullAccessToPackagesNamed")]
  pub allow_full_access_to_packages_named: Vec<String>,
}

impl Default for Metadata {
  fn default() -> Self {
    Metadata {
      schema:
        "https://raw.githubusercontent.com/leadlang/lead/refs/heads/main/metadata.schema.json"
          .into(),
      entry: "./index.pb".into(),
      name: "package".into(),
      version: "1.0.0".into(),
      description: "".into(),
      pkver: METADATA_VER,
      authors: vec!["You".into()],
      keywords: vec![],
      dependencies: HashMap::new(),
      allow_full_access_to_packages_named: vec![],
    }
  }
}

pub async fn get_meta() -> Metadata {
  let meta = fs::read_to_string("./metadata.json")
    .await
    .unwrap_or_default();

  let meta: Metadata = from_str(&meta).unwrap_or_default();

  meta
}

pub async fn write_meta(meta: &Metadata) {
  fs::write(
    "./metadata.json",
    to_string_pretty(meta).expect("Unable to write metadata"),
  )
  .await
  .expect("Unable to write metadata")
}
