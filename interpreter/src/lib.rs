#![feature(fn_traits)]
#![feature(trait_alias)]
#![feature(concat_idents)]
#![feature(macro_metavar_expr)]
#![feature(get_mut_unchecked)]
#![feature(impl_trait_in_bindings)]

use std::{collections::HashMap, process, sync::Arc, time::Instant};

pub use paste::paste;

#[macro_use]
pub mod macros;

pub mod runtime;
pub use runtime::RuntimeValue;

#[cfg(feature = "phf")]
pub use phf;

mod parallel_ipreter;

mod scheduler;

pub use scheduler::Scheduler;

#[macro_use]
pub mod package;
pub mod types;
pub mod val;

pub(crate) use package::*;
use types::ExtendsInternal;
pub use types::{Extends, Heap, LanguagePackages, MethodRes, PrototypeDocs};
pub use val::*;

pub use lealang_chalk_rs::Chalk;

pub static VERSION_INT: u16 = 9;

pub trait Package: Sync {
  fn name(&self) -> &'static [u8];

  fn doc(&self) -> HashMap<&'static str, &'static [&'static str; 3]> {
    HashMap::new()
  }

  fn prototype_docs(&self) -> PrototypeDocs {
    PrototypeDocs::default()
  }

  fn prototype(&self) -> Extends {
    Extends::default()
  }

  fn methods(&self) -> MethodRes {
    &[]
  }
}

pub struct RespPackage {
  pub methods: MethodRes,
  pub extends: Option<Extends>,
}

pub struct Application<'a> {
  code: Arc<Structure>,
  pub(crate) pkg: LanguagePackages<'a>,
  // Resolve path from mod name
  pkg_resolver: Box<dyn FnMut(&str, bool) -> Vec<RespPackage>>,
  // Log in case of full access request
  log_info: Box<dyn FnMut(&str) -> ()>,
}

unsafe impl Send for Application<'_> {}
unsafe impl Sync for Application<'_> {}

pub type Args = Vec<&'static str>;

pub enum LeadCode {
  // Lead Modules will be lazily used
  LeadModule(&'static str),
  // Lead Code should be instantly made ready
  Code(Vec<Args>),
}

pub type Structure = HashMap<
  // File
  &'static str,
  // Code
  LeadCode,
>;

impl<'a> Application<'a> {
  pub fn new<
    T: FnOnce() -> Structure,
    F: FnMut(&str, bool) -> Vec<RespPackage> + 'static,
    R: FnMut(&str) -> () + 'static,
  >(
    dll_resolver: F,
    requested_perm: R,
    structure: T,
  ) -> Self {
    let code = Arc::new(structure());

    Self {
      code,
      pkg: LanguagePackages::new(),
      pkg_resolver: Box::new(dll_resolver),
      log_info: Box::new(requested_perm),
    }
  }

  pub fn add_pkg<T: Package + 'static>(&mut self, package: T) -> &mut Self {
    self.pkg.import(package);
    self
  }

  pub fn add_pkg_static(&mut self, package: &'static dyn Package) -> &mut Self {
    self.pkg.import_static(package);
    self
  }

  pub fn add_pkg_box(&mut self, package: Box<dyn Package>) -> &mut Self {
    self.pkg.import_dyn(package);
    self
  }

  pub fn add_pkg_raw(&mut self, name: &'static [u8], methods: MethodRes) -> &mut Self {
    let pkg = ImplPackage { name, methods };

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
  pub fn run_non(mut self) {
    parallel_ipreter::schedule(&mut self)
  }

  pub fn run(self, time: bool) -> ! {
    // Start the Timer NOW!!!
    let inst = Instant::now();

    self.run_non();

    let dur = inst.elapsed();

    if time {
      println!("\nTime Elasped: {:?}", dur);
    }

    process::exit(0)
  }
}
