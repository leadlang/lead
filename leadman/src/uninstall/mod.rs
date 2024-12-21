use std::{env, fmt::format, fs};

use chalk_rs::Chalk;
use inquire::Select;

use crate::{utils::list_versions, LEAD_ROOT_DIR};

fn version() -> String {
  let versions = list_versions();

  let v = versions.iter().collect::<Vec<_>>();
  env::var("LEAD_VERSION").map_or_else(
    |_| {
      Select::new("Select version to uninstall", v)
        .prompt()
        .expect("You must select a version!")
        .clone()
    },
    |x| {
      if !versions.contains(&x) {
        panic!("Version {} is not installed!", x);
      }

      x
    },
  )
}

pub async fn uninstall(chalk: &mut Chalk) {
  let version = version();

  let cu_path = format!("{}/versions/current", &*LEAD_ROOT_DIR);
  let st_path = format!("{}/versions/stable", &*LEAD_ROOT_DIR);
  let nt_path = format!("{}/versions/nightly", &*LEAD_ROOT_DIR);

  let current = fs::read_to_string(&cu_path).unwrap_or_default();
  let stable = fs::read_to_string(&st_path).unwrap_or_default();
  let nightly = fs::read_to_string(&nt_path).unwrap_or_default();

  if &version == &current {
    fs::write(&cu_path, "").expect("Unable to write to file");
  }
  if &version == &stable {
    fs::write(&st_path, "").expect("Unable to write to file");
  }
  if &version == &nightly {
    fs::write(&nt_path, "").expect("Unable to write to file");
  }

  fs::remove_dir_all(format!("{}/versions/{}", &*LEAD_ROOT_DIR, &version))
    .expect("Unable to delete directory");
}
