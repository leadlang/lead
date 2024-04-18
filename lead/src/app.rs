use interpreter::types::{DynMethodRes, MethodRes};
use interpreter::{error, Application};

use libloading::{library_filename, Library};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::{self, read_to_string, DirEntry};

#[cfg(not(debug_assertions))]
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

#[cfg(not(debug_assertions))]
use std::env::current_exe;

static mut LIBS: Option<Box<HashMap<u8, Library>>> = None;

#[derive(Debug, Serialize, Deserialize)]
pub struct PbJSON {
  pub dev: PbExec,
  pub prod: PbExec,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PbExec {
  pub cmd: String,
  pub args: Vec<String>,
}

pub fn run(dir: String, prod: bool) {
  let pb_json = read_to_string(format!("{}/pb.json", &dir)).unwrap_or_else(|_| {
    error("Invalid dir provided", "core:00");
  });

  let pb_json: PbJSON = from_str::<PbJSON>(&pb_json).unwrap_or_else(|_| {
    error("Invalid pb.json file / No pb.json file found!", "core:00");
  });

  let file = if prod { pb_json.prod } else { pb_json.dev };

  if &file.cmd == "%native" {
    let file = &file.args[0];
    run_inner(file, prod)
  } else {
    Command::new(file.cmd)
      .args(file.args)
      .spawn()
      .unwrap()
      .wait()
      .unwrap();
  }
}

fn run_inner(file: &String, prod: bool) {
  let mut app = Application::new(file.as_str());
  let mut dll: Vec<Library> = vec![];

  unsafe { LIBS = Some(Box::new(HashMap::new())) }

  #[cfg(not(debug_assertions))]
  let mut file: PathBuf = current_exe().unwrap();
  #[cfg(not(debug_assertions))]
  file.pop();
  #[cfg(not(debug_assertions))]
  file.push("lib");

  let mut load = |entry: DirEntry, exe_dir: bool| {
    let path = entry.path();
    let path = path.to_string_lossy();
    let name = entry.file_name();
    let name = name.to_string_lossy();

    let name = if exe_dir {
      OsString::from_str(&path).unwrap()
    } else {
      library_filename(format!("{}/{}", &path, &name))
    };

    dbg!("{:?}", &name);

    let lib = unsafe { Library::new(name) }.unwrap();
    dll.push(lib);
  };

  #[cfg(not(debug_assertions))]
  for file in fs::read_dir(file).unwrap() {
    let entry = file.unwrap();
    load(entry, true);
  }

  for entry in fs::read_dir("./lib").unwrap() {
    let entry = entry.unwrap();
    load(entry, false);
  }

  let mut index = 0u8;
  for lib in dll {
    index += 1;
    let map = unsafe { LIBS.as_mut().unwrap() };

    map.insert(index, lib);

    let lib = map.get_mut(&index).unwrap();
    let func =
      unsafe { lib.get::<fn() -> Vec<(&'static [u8], MethodRes, DynMethodRes)>>(b"modules") }
        .unwrap();

    let fun = func();

    for f in fun {
      app.add_pkg_raw(f.0, f.1, f.2);
    }
  }

  if !prod {
    app.list_cmds();
  }

  app.run();
}
