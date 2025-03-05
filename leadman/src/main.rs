use std::env::args;

fn main() {
  leadman_lib::run(args().collect::<Vec<String>>());
}
