//mod fs;
//use fs::*;

mod io;
use io::*;

use interpreter::exports;

// generate!(Fs, IO, AHQ);
exports! {
  packages = IO;
  runtimes =  {

  }
}
