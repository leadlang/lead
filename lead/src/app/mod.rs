use interpreter::types::MethodRes;
use interpreter::{Application, Package as Pkg, RespPackage};
use lealang_chalk_rs::Chalk;
use std::env;
use std::env::consts::{DLL_EXTENSION, DLL_PREFIX};
use std::ptr::addr_of_mut;
use std::sync::{Arc, LazyLock, Mutex};
use std::thread::spawn;
use std::time::Instant;
use std::{collections::HashMap, fs};

use super::metadata;
use libloading::Library;

mod structure;

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
  time: bool,
  prod: bool,
}

mod logo;

static mut LIBS: Option<HashMap<usize, Package>> = None;
static PT_LIBS: LazyLock<Arc<Mutex<HashMap<String, Package>>>> =
  LazyLock::new(|| Arc::new(Mutex::new(HashMap::new())));

struct Package {
  modules: &'static [&'static dyn Pkg],
  _lib: Library,
}

impl Package {
  fn new(path: &str, prod: bool) -> Self {
    unsafe {
      let library = Library::new(path).expect("Unable to load library");

      // Ignore MAJOR INT checking during production
      if !prod {
        // Let us do some version checking
        let ver = library
          .get::<fn() -> u16>(b"ver")
          .expect("Unable to verify interpreter versions");

        let ver = ver();

        if ver != interpreter::VERSION_INT {
          panic!(
            "`{path}` uses v{ver} which is not compatible with `v{}` of LeadLang Interpreter",
            interpreter::VERSION_INT
          );
        }
      }

      let f = library
        .get::<fn() -> &'static [&'static dyn Pkg]>(b"modules")
        .expect("Unable to get module export");

      Self {
        modules: f(),
        _lib: library,
      }
    }
  }
}

pub async fn run(args: &[String], chalk: &mut Chalk) {
  let options = parse(args);

  if options.time {
    println!("🏃 Pre-processing code...");
  }

  unsafe {
    LIBS = Some(HashMap::new());
  }

  let instant = Instant::now();

  let data = metadata::get_meta().await;

  let structure = structure::parse(&data);

  if options.sysinfo {
    logo::render_lead_logo(options.monochrome);
  }

  let chalk_1_mut: *mut Chalk = unsafe { chalk as *mut _ };
  let chalk_2_mut: *mut Chalk = unsafe { chalk as *mut _ };
  let pkgmap = create_pkg_map();

  // We are guaranteed that the closures run in the single thread & NOT AT THE SAME TIME.
  let mut application = Application::new(
    move |name, extends| {
      let mut libs = PT_LIBS.lock().map_or_else(|e| e.into_inner(), |e| e);

      let mut out = vec![];

      let pkg = pkgmap.get(name).expect("Unable to get package name");

      if let Some(x) = libs.get(pkg) {
        for module in (x.modules) {
          out.push(RespPackage {
            methods: module.methods(),
            extends: if extends {
              Some(module.prototype())
            } else {
              None
            },
          });
        }
      } else {
        let pkg = Package::new(pkg, options.prod);

        for module in (pkg.modules) {
          out.push(RespPackage {
            methods: module.methods(),
            extends: if extends {
              Some(module.prototype())
            } else {
              None
            },
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
    || structure,
  );

  load_lib();

  let libs = unsafe { (&mut *addr_of_mut!(LIBS)).as_mut().unwrap() };

  for (_, a) in libs.iter() {
    let pkgs = (a.modules);

    for pkg in pkgs {
      application.add_pkg_static(*pkg);
    }
  }

  if options.time {
    let dur = instant.elapsed();
    println!("✅ Preprocessed in {:?}", dur);
  }

  let _ = spawn(move || { application.run(options.time) })
    .join();
}

fn load_lib() {
  let libs = unsafe { (&mut *addr_of_mut!(LIBS)).as_mut().unwrap() };

  let path = env::var("LEAD_HOME").expect("Unable to get LEAD_HOME, is lead installed?");

  let path = format!("{path}/versions/{}/lib/", env!("CARGO_PKG_VERSION"));

  for (k, entry) in fs::read_dir(path).expect("Path").enumerate() {
    let entry = entry.expect("OS Error").path();

    // There's no need to check these, they are already okay
    let val = Package::new(entry.to_str().expect("Unable to read as string"), true);

    libs.insert(k, val);
  }
}

fn create_pkg_map() -> HashMap<&'static str, String> {
  let mut pkgmap = HashMap::new();

  let dir = fs::read_dir("./.lead_libs");

  if let Ok(dir) = dir {
    for entry in dir {
      let entry = entry.expect("OS Error");

      let name = entry
        .file_name()
        .into_string()
        .expect("Error reading hash")
        .leak();

      let lookup = &name[65..];

      let mut path: std::path::PathBuf = entry.path();

      let libpath = format!("{DLL_PREFIX}{lookup}.{DLL_EXTENSION}");
      path.push(libpath);

      pkgmap.insert(
        lookup,
        path
          .into_os_string()
          .into_string()
          .expect("Unable to convert to string"),
      );
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
    time: true,
    prod: false,
  };

  args.iter().for_each(|v| match v.as_str() {
    "--prod" => {
      opt.prod = true;
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
