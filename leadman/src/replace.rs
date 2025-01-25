use std::{env::current_exe, fs};

use crate::LEAD_ROOT_DIR;

pub fn replace() {
  let src = current_exe().unwrap();

  #[cfg(windows)]
  fs::copy(src, format!("{}/leadman.exe", &*LEAD_ROOT_DIR)).expect("Unable to process IO Action");

  #[cfg(unix)]
  fs::copy(src, format!("{}/leadman", &*LEAD_ROOT_DIR)).expect("Unable to process IO Action");

  let files = [
    ("lead", include_str!("./lead")), 
    ("leadc", include_str!("./leadc")), 
    ("lead.ps1", include_str!("./lead.ps1")), 
    ("leadc.ps1", include_str!("./leadc.ps1")),
  ];

  for (exec, data) in files {
    fs::write(
      format!("{}/{exec}", &*LEAD_ROOT_DIR),
      data,
    )
    .expect("Could not update scripts");
  }
}
