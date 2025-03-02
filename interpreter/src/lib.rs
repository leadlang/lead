#![feature(fn_traits)]
#![feature(trait_alias)]
#![feature(concat_idents)]
#![feature(macro_metavar_expr)]

use std::{
  collections::HashMap,
  process,
  time::{Duration, Instant},
};

#[macro_use]
pub mod macros;

pub mod runtime;

mod ipreter;
#[macro_use]
pub mod package;
pub mod types;
pub mod val;

pub use package::*;
use tokio::runtime::{Builder, Runtime};
use types::{DynMethodRes, Heap, LanguagePackages, MethodRes};
pub use val::*;
pub use tokio::*;

pub use lealang_chalk_rs::Chalk;

pub static VERSION_INT: u16 = 3;

pub trait Package {
  fn name(&self) -> &'static [u8];

  fn doc(&self) -> HashMap<&'static str, &'static str> {
    HashMap::new()
  }

  fn methods(&self) -> MethodRes {
    &[]
  }

  fn dyn_methods(&self) -> DynMethodRes {
    vec![]
  }
}

pub struct RespPackage {
  pub name: &'static [u8],
  pub methods: MethodRes,
  pub dyn_methods: DynMethodRes,
}

pub struct Application<'a> {
  code: HashMap<String, String>,
  pkg: LanguagePackages<'a>,
  entry: &'a str,
  heap: Heap,

  // Resolve files
  module_resolver: Box<dyn FnMut(&str) -> Vec<u8>>,
  // Resolve path from mod name
  pkg_resolver: Box<dyn FnMut(&str) -> RespPackage>,
  // Log in case of full access request
  log_info: Box<dyn FnMut(&str) -> ()>,
  pub(crate) runtime: Runtime,
  inst: Instant,
}

impl<'a> Application<'a> {
  pub fn new<
    T: FnMut(&str) -> Vec<u8> + 'static,
    F: FnMut(&str) -> RespPackage + 'static,
    R: FnMut(&str) -> () + 'static,
  >(
    file: &'a str,
    mut fs_resolver: T,
    dll_resolver: F,
    requested_perm: R,
  ) -> Self {
    let main = String::from_utf8(fs_resolver(file)).expect("Invalid utf8");

    let mut code = HashMap::new();
    code.insert(":entry".to_string(), main);
    Self {
      code,
      pkg: LanguagePackages::new(),
      heap: Heap::new(),
      entry: &file,
      module_resolver: Box::new(fs_resolver),
      pkg_resolver: Box::new(dll_resolver),
      log_info: Box::new(requested_perm),
      runtime: Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("Unable to build async runtime"),
      inst: Instant::now(),
    }
  }

  pub fn add_file(&mut self, name: String, file: String) -> &mut Self {
    self.code.insert(name, file);
    self
  }

  pub fn add_pkg<T: Package + 'static>(&mut self, package: T) -> &mut Self {
    self.pkg.import(package);
    self
  }

  pub fn add_pkg_box(&mut self, package: Box<dyn Package>) -> &mut Self {
    self.pkg.import_dyn(package);
    self
  }

  pub fn add_pkg_raw(
    &mut self,
    name: &'static [u8],
    methods: MethodRes,
    dyn_methods: DynMethodRes,
  ) -> &mut Self {
    let pkg = ImplPackage {
      name,
      methods,
      dyn_methods,
    };

    self.pkg.import(pkg);

    self
  }

  pub fn list_cmds(&mut self) -> &mut Self {
    let mut chalk = Chalk::new();
    chalk.red().bold();
    chalk.println(&"The Lead Programming Language");

    chalk.reset_weight().yellow().println(&"Interpreter");

    self.pkg.list(&mut chalk);
    self
  }

  /// ⚠️ This function still is panicking
  pub fn run_non(mut self) -> Duration {
    ipreter::interpret(":entry", &mut self);
    self.runtime.shutdown_background();

    self.inst.elapsed()
  }

  pub fn run(self, time: bool) -> ! {
    let dur = self.run_non();

    if time {
      println!("\nTime Elasped: {:?}", dur);
    }

    process::exit(0)
  }
}
