use std::{env::consts::EXE_SUFFIX, fs};

use crate::{utils::list_versions as ls_ver, LEAD_ROOT_DIR};

pub fn list_versions() {
  println!("{:<32} {:<9} {:<10} Usage", "Version", "Build", "LeadC *");
  println!("{:<32} {:<9} {:<10} -----", "-------", "-----", "-------");

  let stable =
    fs::read_to_string(format!("{}/versions/stable", &*LEAD_ROOT_DIR)).unwrap_or_default();
  let nightly =
    fs::read_to_string(format!("{}/versions/nightly", &*LEAD_ROOT_DIR)).unwrap_or_default();
  let current =
    fs::read_to_string(format!("{}/versions/current", &*LEAD_ROOT_DIR)).unwrap_or_default();

  let mut is = false;
  for name in ls_ver() {
    is = true;

    let mut build = fs::read_to_string(format!("{}/versions/{name}/.lbuild", &*LEAD_ROOT_DIR))
      .unwrap_or_else(|_| "< 6".into());

    // 7 digit build is not at all possible
    if build.len() > 7 {
      build = "INVALID".into();
    }

    let leadc = if fs::exists(format!(
      "{}/versions/{name}/leadc{}",
      &*LEAD_ROOT_DIR, EXE_SUFFIX
    ))
    .unwrap_or(false)
    {
      "Yes"
    } else {
      "No"
    };

    if !["stable", "nightly", "current"].contains(&name.as_str()) {
      print!("{:<32} {:<9} {:<10}", name, build, leadc);

      if &current == &name {
        println!(" lead +current [...args]");
      } else if &stable == &name {
        println!(" lead +stable [...args]");
      } else if &nightly == &name {
        println!(" lead +nightly [...args]");
      } else {
        println!(" lead +{} [...args]", &name);
      }
    }
  }

  if !is {
    println!("No version has been installed (yet)!");
  } else {
    println!(
      "\nNote: `[...args]` refers to additional commands or flags that can be passed to lead"
    );
    println!("    * `LeadC` support indicates the presence of leadc{EXE_SUFFIX} in the build");
  }
}
