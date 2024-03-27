mod fs;
use fs::*;
mod io;
use io::*;

use interpreter::generate;

generate!(Fs, IO);
