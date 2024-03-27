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

pub fn error<T: Display>(msg: T, file: T) -> ! {
  println!("{}{msg}", *ERROR);
  println!("{}{file}", *ERROR);
  process::exit(1);
}
