use std::{borrow::Cow, env, fmt::format, fs};

use inquire::{
  validator::{ErrorMessage, Validation},
  MultiSelect,
};
use lealang_chalk_rs::Chalk;

use crate::{utils::list_versions, LEAD_ROOT_DIR};

fn version() -> Vec<String> {
  env::var("LEAD_VERSION").map_or_else(
    |_| {
      let versions = list_versions();

      MultiSelect::new("Select version to uninstall", versions)
        .with_validator(|v: &[inquire::list_option::ListOption<&String>]| {
          if v.len() == 0 {
            return Ok(Validation::Invalid(ErrorMessage::Custom(
              "Please select at least 1 version to uninstall".into(),
            )));
          }

          Ok(Validation::Valid)
        })
        .prompt()
        .expect("You must select a version!")
        .clone()
    },
    |x| {
      let versions = list_versions();

      if !versions.contains(&x) {
        panic!("Version {} is not installed!", x);
      }

      vec![x]
    },
  )
}

pub async fn uninstall(chalk: &mut Chalk) {
  let versions = version();

  for version in versions {
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
}
