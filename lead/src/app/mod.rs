use lealang_chalk_rs::Chalk;
use interpreter::types::{DynMethodRes, MethodRes};
use interpreter::{Application, Package as Pkg, RespPackage};
use std::env;
use std::env::consts::{DLL_EXTENSION, DLL_PREFIX};
use std::ptr::addr_of_mut;
use std::sync::{Arc, LazyLock, Mutex};
use std::{collections::HashMap, fs};

use super::metadata;
use libloading::Library;

enum FullAccessLevel {
  SilentlyAllow,
  Warn,
  Deny,
}

struct Options {
  sysinfo: bool,
  monochrome: bool,
  full_access: FullAccessLevel,
  log: bool,
  time: bool
}

mod logo;

static mut LIBS: Option<HashMap<usize, Package>> = None;
static PT_LIBS: LazyLock<Arc<Mutex<HashMap<String, Package>>>> = LazyLock::new(|| Arc::new(Mutex::new(HashMap::new())));

struct Package {
  modules: fn() -> Vec<Box<dyn Pkg>>,
  _lib: Library,
}

impl Package {
  fn new(path: &str) -> Self {
    unsafe {
      let library = Library::new(path).expect("Unable to load library");
      let f = library
        .get::<fn() -> Vec<Box<dyn Pkg>>>(b"modules")
        .expect("Unable to get module export");

      Self {
        modules: *f,
        _lib: library,
      }
    }
  }
}

pub async fn run(args: &[String], chalk: &mut Chalk) {
  unsafe {
    LIBS = Some(HashMap::new());
  }

  let options = parse(args);

  let data = metadata::get_meta().await;

  if options.sysinfo {
    logo::render_lead_logo(options.monochrome);
  }

  let chalk_1_mut: *mut Chalk = unsafe { chalk as *mut _ };
  let chalk_2_mut: *mut Chalk = unsafe { chalk as *mut _ };
  let pkgmap = create_pkg_map();

  // We are guaranteed that the closures run in the single thread & NOT AT THE SAME TIME.
  let mut application = Application::new(
    &data.entry,
    |path| fs::read(path).expect("Unable to read file"),
    move |name| {
      let mut libs = PT_LIBS.lock()
        .map_or_else(|e| e.into_inner(), |e| e);

      let mut out = vec![];

      let pkg = pkgmap.get(name)
        .expect("Unable to get package name");

      if let Some(x) = libs.get(pkg) {
        for module in (x.modules)() {
          out.push(RespPackage { 
            name: b"imported", 
            methods: module.methods(), 
            dyn_methods: module.dyn_methods()
          });
        }
      } else {
        let pkg = Package::new(pkg);

        for module in (pkg.modules)() {
          out.push(RespPackage { 
            name: b"imported", 
            methods: module.methods(), 
            dyn_methods: module.dyn_methods()
          });
        }

        libs.insert(name.to_string(), pkg);
      }

      out
    },
    move |pkg_name| {
      let chalk = unsafe { &mut *chalk_1_mut };

      let pkg_name: &String = &pkg_name.into();

      if data.allow_full_access_to_packages_named.contains(pkg_name) && options.log {
        chalk.blue().print(&"[INFO] ");
        chalk.blue().print(&format!("{pkg_name} "));

        println!("has been granted full heap access");
      }

      if !data.allow_full_access_to_packages_named.contains(pkg_name) {
        match options.full_access {
          FullAccessLevel::SilentlyAllow => {}
          FullAccessLevel::Warn => {
            chalk.yellow().print(&"[WARN] ");
            chalk.blue().print(&format!("{pkg_name} "));

            println!("was been granted full heap access");
          }
          FullAccessLevel::Deny => {
            chalk.red().print(&"[ERRR] ");
            chalk.blue().print(&format!("{pkg_name} "));

            println!(" tried to get full heap access\n       ❌ Access Denied, Exiting");
            std::process::exit(1);
          }
        }
      }
    },
  );

  load_lib();

  let libs = unsafe { (&mut *addr_of_mut!(LIBS)).as_mut().unwrap() };

  for (_, a) in libs.iter() {
    let pkgs = (a.modules)();

    for pkg in pkgs {
      application.add_pkg_box(pkg);
    }
  }

  application.run(options.time);
}

fn load_lib() {
  let libs = unsafe { (&mut *addr_of_mut!(LIBS)).as_mut().unwrap() };

  let path = env::var("LEAD_HOME").expect("Unable to get LEAD_HOME, is lead installed?");

  let path = format!("{path}/versions/{}/lib/", env!("CARGO_PKG_VERSION"));

  for (k, entry) in fs::read_dir(path).expect("Path").enumerate() {
    let entry = entry.expect("OS Error").path();

    let val = Package::new(entry.to_str().expect("Unable to read as string"));

    libs.insert(k, val);
  }
}

fn create_pkg_map() -> HashMap<String, String> {
  let mut pkgmap = HashMap::new();

  let dir = fs::read_dir("./.lead_libs");

  if let Ok(dir) = dir {
    for entry in dir {
      let entry = entry.expect("OS Error");

      let name = entry.file_name().into_string().expect("Error reading hash");

      let mut path = entry.path();

      let lookup = fs::read_to_string(format!("./.lead_libs/{name}/lead.lookup.lkp"))
        .expect("Unable to process lead lookup file");

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
    full_access: FullAccessLevel::Warn,
    monochrome: false,
    time: true
  };

  args.iter().for_each(|v| match v.as_str() {
    "--prod" => {
      opt.sysinfo = false;
      opt.log = true;
      opt.full_access = FullAccessLevel::Deny;
    }
    "--monochrome-logo" => opt.monochrome = true,
    "--no-sysinfo" => opt.sysinfo = false,
    "--log" => opt.log = true,
    "--warn-full-access" => opt.full_access = FullAccessLevel::Warn,
    "--allow-full-access" => opt.full_access = FullAccessLevel::SilentlyAllow,
    "--deny-full-access" => opt.full_access = FullAccessLevel::Deny,
    "--no-time" => opt.time = false,
    _ => {
      println!("Unknown argument {}", v);
    }
  });

  opt
}
