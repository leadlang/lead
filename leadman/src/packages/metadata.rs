use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use tokio::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct LibraryMeta {
  pub name: String,
  pub version: String,
  pub description: String,
  pub authors: Vec<String>,
  pub keywords: Vec<String>,
  pub platforms: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
  #[serde(rename = "$schema")]
  pub schema: String,
  pub name: String,
  pub version: String,
  pub description: String,
  pub authors: Vec<String>,
  pub keywords: Vec<String>,
  pub dependencies: HashMap<String, Dependency>,
}

impl Default for Metadata {
  fn default() -> Self {
    Metadata {
      schema: "https://raw.githubusercontent.com/leadlang/lead/refs/heads/main/metadata.schema.json".into(),
      name: "package".into(),
      version: "1.0.0".into(),
      description: "".into(),
      authors: vec!["You".into()],
      keywords: vec![],
      dependencies: HashMap::new(),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Dependency {
  pub version: String,
  pub os: Vec<String>,
}

pub async fn get_meta() -> Metadata {
  let meta = fs::read_to_string("./metadata.json").await.unwrap_or_default();

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
