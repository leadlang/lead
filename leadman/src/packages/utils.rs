use indicatif::ProgressStyle;

pub fn progress_bar() -> ProgressStyle {
  ProgressStyle::with_template("      {msg} {wide_bar} {pos}/{len}").unwrap()
}

pub fn spinner_style() -> ProgressStyle {
  ProgressStyle::with_template("      {spinner}  {msg}").unwrap()
}