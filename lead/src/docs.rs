use std::env;

pub fn run_docs() {
  let home = env::var("LEAD_HOME").expect("Broken Lead Installation, LEAD_HOME is necessary");
  let version = env!("CARGO_PKG_VERSION");

  println!("Home: {home}, Version {version}");
}