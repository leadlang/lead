#![feature(fn_traits)]
#![feature(trait_alias)]

use std::{collections::HashMap, fs, process};

#[macro_use]
pub mod macros;

mod ipreter;
#[macro_use]
pub mod package;
pub mod types;
pub mod val;

pub use package::*;
use types::{DynMethodRes, Heap, LanguagePackages, MethodRes, PackageCallback};
pub use val::*;

pub use chalk_rs::Chalk;

pub static NUMBER: u8 = 0;
pub static STRING: u8 = 1;
pub static BOOLEAN: u8 = 2;

pub trait Package {
  fn name(&self) -> &'static [u8];
  fn methods(&self) -> MethodRes {
    &[]
  }
  fn dyn_methods(&self) -> DynMethodRes {
    vec![]
  }
}

pub struct Application<'a> {
  code: HashMap<String, String>,
  pkg: LanguagePackages<'a>,
  entry: &'a str,
  next_marker: bool,
  heap: Heap,
}

impl<'a> Application<'a> {
  pub fn new(file: &'a str) -> Self {
    let main = fs::read_to_string(&file).unwrap();

    let mut code = HashMap::new();
    code.insert(":entry".to_string(), main);
    Self {
      code,
      pkg: LanguagePackages::new(),
      heap: Heap::new(),
      entry: &file,
      next_marker: false,
    }
  }

  pub fn add_file(&mut self, name: String, file: String) -> &mut Self {
    self.code.insert(name, file);
    self
  }

  pub fn add_pkg<T: Package>(&mut self, package: T) -> &mut Self {
    self.pkg.import(package);
    self
  }

  pub fn add_pkg_raw(
    &mut self,
    name: &'static [u8],
    methods: &'static [(&'static str, PackageCallback)],
    dyn_methods: DynMethodRes,
  ) -> &mut Self {
    let mut pkg = ImplPackage::new();
    pkg.name = name;
    pkg.methods = methods;
    pkg.dyn_methods = dyn_methods;

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

  pub fn run(mut self) -> ! {
    ipreter::interpret(":entry", &mut self);
    process::exit(0);
  }
}
