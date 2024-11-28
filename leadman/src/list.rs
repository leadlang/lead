use std::fs;

use crate::{utils::list_versions as ls_ver, LEAD_ROOT_DIR};

pub fn list_versions() {
  println!("{:<35} Usage", "Version");
  println!("{:<35} -----", "-------");

  let stable =
    fs::read_to_string(format!("{}/versions/stable", &*LEAD_ROOT_DIR)).unwrap_or_default();
  let nightly =
    fs::read_to_string(format!("{}/versions/nightly", &*LEAD_ROOT_DIR)).unwrap_or_default();
  let current =
    fs::read_to_string(format!("{}/versions/current", &*LEAD_ROOT_DIR)).unwrap_or_default();

  let mut is = false;
  for name in ls_ver() {
    is = true;

    if !["stable", "nightly", "current"].contains(&name.as_str()) {
      print!("{:<35}", name);

      if &current == &name {
        println!(" lead +current [args]");
      } else if &stable == &name {
        println!(" lead +stable [args]");
      } else if &nightly == &name {
        println!(" lead +nightly [args]");
      } else {
        println!(" lead +{} [args]", &name);
      }
    }
  }

  if !is {
    println!("No version has been installed (yet)!");
  } else {
    println!("\nNote: `[args]` refers to additional commands or flags that can be passed to lead");
  }
}
