//mod fs;
//use fs::*;

mod io;
use io::*;

use interpreter::{phf, exports};

// generate!(Fs, IO, AHQ);
exports! {
  packages = IO;
  runtimes =  {

  }
}
