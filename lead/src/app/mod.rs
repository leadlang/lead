use std::env;
use std::ptr::addr_of_mut;
use std::{collections::HashMap, fs};
use std::env::consts::{DLL_PREFIX, DLL_EXTENSION};
use chalk_rs::Chalk;
use interpreter::types::{DynMethodRes, MethodRes};
use interpreter::Application;

use super::metadata;
use dlopen2::wrapper::{Container, WrapperApi};

enum FullAccessLevel {
  SilentlyAllow,
  Warn,
  Deny
}

struct Options {
  sysinfo: bool,
  full_access: FullAccessLevel,
  log: bool,
}

mod logo;

static mut LIBS: Option<HashMap<usize, Container<Package>>> = None;

#[derive(WrapperApi)]
struct Package {
  modules: fn() -> Vec<(&'static [u8], MethodRes, DynMethodRes)>,
}


pub async fn run(args: &[String], chalk: &mut Chalk) {
  unsafe {
    LIBS = Some(HashMap::new());
  }

  let options = parse(args);
  
  let data = metadata::get_meta().await;
  
  if options.sysinfo {
    logo::render_lead_logo();
  }

  let pkgmap = create_pkg_map();

  let mut application = Application::new(&data.entry, |path| fs::read(path).expect("Unable to read file"), |name| {
    todo!();
  }, move |pkg_name| {
    let mut chalk = Chalk::new();

    let pkg_name: &String = &pkg_name.into();

    if data.allow_full_access_to_packages_named.contains(pkg_name) && options.log {
      chalk.blue().print(&"[INFO] ");
      println!("{pkg_name} has been granted full heap access");
    }

    if !data.allow_full_access_to_packages_named.contains(pkg_name) {
      match options.full_access {
        FullAccessLevel::SilentlyAllow => {}
        FullAccessLevel::Warn => {
          chalk.blue().print(&"[WARN] ");
          println!("{pkg_name} was been granted full heap access");
        }
        FullAccessLevel::Deny => {
          chalk.blue().print(&"[ERRR] ");
          println!("{pkg_name} tried to get full heap access\n       Exiting");
        }
      }
    }
  });

  load_lib();

  let libs = unsafe {
    (&mut *addr_of_mut!(LIBS)).as_mut().unwrap()
  };

  for (_, a) in libs.iter() {
    let pkgs = a.modules();

    for pkg in pkgs {
      application.add_pkg_raw(pkg.0, pkg.1, pkg.2);
    }
  }

  application.run();
}

fn load_lib() {
  let libs = unsafe {
    (&mut *addr_of_mut!(LIBS)).as_mut().unwrap()
  };

  let path = env::var("LEAD_HOME").expect("Unable to get LEAD_HOME, is lead installed?");

  let path = format!("{path}/versions/{}/lib/", env!("CARGO_PKG_VERSION"));

  for (k, entry) in fs::read_dir(path).expect("Path").enumerate() {
    let entry = entry.expect("OS Error").path();

    let val: Container<Package> = unsafe { Container::load(entry).expect("Unable to load dll") };

    libs.insert(k, val);
  }
}

fn create_pkg_map() -> HashMap<String, String> {
  let mut pkgmap = HashMap::new();

  let dir = fs::read_dir("./lib");

  if let Ok(dir) = dir {
    for entry in dir {
      let entry = entry.expect("OS Error");

      let name = entry.file_name().into_string().expect("Error reading hash");

      let mut path = entry.path();

      let lookup = fs::read_to_string(format!("./lib/{name}/lead.lookup.lkp")).expect("Unable to process lead lookup file");

      let libpath = format!("{DLL_PREFIX}{lookup}.{DLL_EXTENSION}");
      path.push(libpath);

      pkgmap.insert(lookup, path.to_str().unwrap().into());
    }
  }

  pkgmap
}

fn parse(args: &[String]) -> Options {
  let mut opt = Options {
    sysinfo: true,
    log: false,
    full_access: FullAccessLevel::Warn
  };

  args.iter().for_each(|v| match v.as_str() {
    "--prod" => {
      opt.sysinfo = false;
      opt.log = true;
      opt.full_access = FullAccessLevel::Deny;
    }
    "--no-sysinfo" => opt.sysinfo = false,
    "--log" => opt.log = true,
    "--warn-full-access" => opt.full_access = FullAccessLevel::Warn,
    "--allow-full-access" => opt.full_access = FullAccessLevel::SilentlyAllow,
    "--deny-full-access" => opt.full_access = FullAccessLevel::Deny,
    _ => {
      println!("Unknown argument {}", v);
    }
  });

  opt
}