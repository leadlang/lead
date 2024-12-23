use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use tokio::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
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
  pub tag_name: String,
  pub os: Vec<String>
}

pub async fn get_meta() -> Metadata {
  let meta = fs::read_to_string("./lead.json").await.unwrap_or_default();

  let meta: Metadata = from_str(&meta).unwrap_or_default();

  meta
}

pub async fn write_meta(meta: &Metadata) {
  fs::write("./lead.json", to_string_pretty(meta).expect("Unable to write metadata")).await.expect("Unable to write metadata")
}