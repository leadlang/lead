use std::{env::current_exe, fs};

use crate::LEAD_ROOT_DIR;

pub fn replace() {
  let src = current_exe().unwrap();

  #[cfg(windows)]
  fs::copy(src, format!("{}/leadman.exe", &*LEAD_ROOT_DIR)).expect("Unable to process IO Action");

  #[cfg(unix)]
  fs::copy(src, format!("{}/leadman", &*LEAD_ROOT_DIR)).expect("Unable to process IO Action");

  fs::write(
    format!("{}/lead", &*LEAD_ROOT_DIR),
    include_bytes!("./lead"),
  )
  .expect("Could not update lead");
  fs::write(
    format!("{}/lead.ps1", &*LEAD_ROOT_DIR),
    include_bytes!("./lead.ps1"),
  )
  .expect("Could not update lead.ps1");
}
