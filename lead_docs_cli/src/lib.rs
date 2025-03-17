pub mod utils;

pub mod viewer;

#[no_mangle]
pub fn run() {
  viewer::run_cursive();
  //make_sel();
}
