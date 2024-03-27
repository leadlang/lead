use std::{fmt::Display, process};

use lazy_static::lazy_static;

lazy_static! {
  static ref INFO: String = String::from_utf8_lossy(&[
    27, 91, 51, 52, 109, 27, 91, 49, 109, 73, 78, 70, 79, 58, 32, 27, 91, 109,
  ])
  .to_string();
  static ref WARN: String = String::from_utf8_lossy(&[
    27, 91, 51, 51, 109, 27, 91, 49, 109, 87, 65, 82, 78, 58, 32, 27, 91, 109,
  ])
  .to_string();
  static ref ERROR: String = String::from_utf8_lossy(&[
    27, 91, 51, 49, 109, 27, 91, 49, 109, 69, 82, 82, 58, 32, 27, 91, 109,
  ])
  .to_string();
}

pub fn info<T: Display>(msg: T) {
  println!("{}{msg}", *INFO);
}

pub fn warn<T: Display>(msg: T) {
  println!("{}{msg}", *WARN);
}

fn gen_build() -> usize {
  let [major, minor, patch] = env!("CARGO_PKG_VERSION").split(".").collect::<Vec<_>>()[..] else {
    return 0;
  };

  let major = major.parse::<usize>().unwrap_or(0);
  let minor = minor.parse::<usize>().unwrap_or(0);
  let patch = patch.parse::<usize>().unwrap_or(0);

  (major * 1000) + (minor * 100) + patch
}

pub fn error<T: Display>(msg: T, file: T) -> ! {
  println!("{}{msg}", *ERROR);
  println!("{}--------    TRACE    --------", *ERROR);
  println!("{} File: {file}", *ERROR);
  println!("{} Edition {}", *ERROR, env!("CARGO_PKG_VERSION").split_at(1).0);
  println!("{} Lead v{}", *ERROR, env!("CARGO_PKG_VERSION"));
  println!("{} Build #{}", *ERROR, gen_build());
  println!("{} Compiled with Rust Nightly", *ERROR);
  println!("{}-----------------------------", *ERROR);
  process::exit(1);
}
