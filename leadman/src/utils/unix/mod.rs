use std::fs::{read_to_string, write};
use super::bashrc;

pub async fn postinstall(path: &str) {
  super::chmod(path);

  #[cfg(any(target_os = "linux", target_os = "macos"))]
  install_path(path);

  #[cfg(not(any(target_os = "linux", target_os = "macos")))]
  {
    println!("Add {:?} and \"{}/current\" to your PATH environment variable", &path);
    println!("Set {:?} as LEAD_HOME environment variable", &path);
  }
}

fn install_path(path: &str) {
  let bash = bashrc();

  let mut data = read_to_string(&bash).unwrap();

  if !data.contains("# Lead Language Setup") {
    data.push_str(&format!("\n# Lead Language Setup\nexport LEAD_HOME=\"{}\"\nexport PATH=\"{}:{}/current:$PATH\"", &path, &path, &path));
  }

  write(&bash, data).unwrap();
}