use std::{
  env::consts::{DLL_PREFIX, DLL_SUFFIX},
  fmt::Display,
  fs,
  sync::LazyLock,
};
use serde::{Serialize, Deserialize};

static VERSION: &'static str = env!("CARGO_PKG_VERSION");
static LEAD_HOME: LazyLock<String> =
  LazyLock::new(|| std::env::var("LEAD_HOME").expect("LEAD_HOME must be set"));

#[derive(Serialize, Deserialize)]
pub struct PackageEntry {
  pub display: String,
  pub file: String,
}

impl Display for PackageEntry {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.display.fmt(f)
  }
}

pub fn lead_lib() -> Vec<PackageEntry> {
  let path = format!("{}/versions/{}/lib", &*LEAD_HOME, VERSION);

  let data = fs::read_dir(path)
    .expect("Error reading lead directory")
    .into_iter()
    .map(|x| {
      let file = x.unwrap().file_name().into_string().unwrap();

      PackageEntry {
        display: get_display(&file),
        file: format!("{}/versions/{VERSION}/lib/{file}", &*LEAD_HOME),
      }
    })
    .collect::<Vec<_>>();

  data
}

pub fn lead_ws() -> Vec<PackageEntry> {
  let path = format!("./.lead_libs");

  let data = fs::read_dir(path)
    .expect("Error reading workspace directory, does it exist?")
    .into_iter()
    .map(|x| {
      let dir = x.unwrap().file_name().into_string().unwrap();

      let f = fs::read_to_string(format!("./.lead_libs/{dir}/lead.lookup.lkp"))
        .expect("Error reading lookup file");

      PackageEntry {
        display: get_display(&dir),
        file: format!("./.lead_libs/{dir}/{}{f}{}", DLL_PREFIX, DLL_SUFFIX),
      }
    })
    .collect::<Vec<_>>();

  data
}

pub fn get_display(a: &str) -> String {
  match a {
    "liblead_core.so" | "lead_core.dll" | "liblead_core.dylib" => "📦 Lead Core".into(),
    "liblead_std.so" | "lead_std.dll" | "liblead_std.dylib" => "📦 Lead Std".into(),
    _ => format!("📦 {a}"),
  }
}
