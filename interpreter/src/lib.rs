#![feature(fn_traits)]
#![feature(trait_alias)]

use std::{collections::HashMap, fs, process};

mod ipreter;
#[macro_use]
pub mod package;
pub mod types;
pub mod val;

use chalk_rs::Chalk;
pub use package::*;
use types::{DynMethodRes, Heap, LanguagePackages, MethodRes};
pub use val::*;

pub static NUMBER: u8 = 0;
pub static STRING: u8 = 1;
pub static BOOLEAN: u8 = 2;

pub trait Package {
  fn name(&self) -> &'static str;
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
  next_marker: bool,
  heap: Heap,
}

impl<'a> Application<'a> {
  pub fn new(file: String) -> Self {
    let main = fs::read_to_string(file).unwrap();

    let mut code = HashMap::new();
    code.insert(":entry".to_string(), main);
    Self {
      code,
      pkg: LanguagePackages::new(),
      heap: Heap::new(),
      next_marker: false,
    }
  }

  pub fn add_pkg<T: Package + 'static>(&mut self, package: T) -> &mut Self {
    self.pkg.import(package);
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
