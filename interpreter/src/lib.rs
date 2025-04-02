#![feature(fn_traits)]
#![feature(trait_alias)]
#![feature(concat_idents)]
#![feature(macro_metavar_expr)]
#![feature(get_mut_unchecked)]

use std::{
  collections::HashMap, mem::zeroed, process, sync::LazyLock, time::{Duration, Instant}
};

pub use paste::paste;

#[macro_use]
pub mod macros;

pub mod runtime;
pub use runtime::RuntimeValue;

#[cfg(feature = "phf")]
pub use phf;

mod ipreter;
#[macro_use]
pub mod package;
pub mod types;
pub mod val;

pub(crate) use package::*;
use tokio::runtime::{Builder, Runtime};
use types::{Extends, ExtendsInternal, Heap, LanguagePackages, MethodRes, PrototypeDocs};
pub use val::*;

pub use tokio;

pub use lealang_chalk_rs::Chalk;

pub static VERSION_INT: u16 = 7;

static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| 
  Builder::new_multi_thread()
    .worker_threads(2)
    .enable_all()
    
    .build()
    .expect("Unable to build async runtime")
);

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
  pub extends: Option<Extends>
}

pub struct Application<'a> {
  code: HashMap<String, String>,
  pub(crate) pkg: LanguagePackages<'a>,
  entry: &'a str,
  heap: Heap,

  // Resolve files
  module_resolver: Box<dyn FnMut(&str) -> Vec<u8>>,
  // Resolve path from mod name
  pkg_resolver: Box<dyn FnMut(&str, bool) -> Vec<RespPackage>>,
  // Log in case of full access request
  log_info: Box<dyn FnMut(&str) -> ()>,
  pub(crate) runtime: &'static Runtime,
  inst: Instant,
}

unsafe impl Send for Application<'_> {}
unsafe impl Sync for Application<'_> {}

impl<'a> Application<'a> {
  pub fn new<
    T: FnMut(&str) -> Vec<u8> + 'static,
    F: FnMut(&str, bool) -> Vec<RespPackage> + 'static,
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
      // SAFETY: It is Initialized before being used
      // Basically its me being lazy & Wanting to reduce computation time
      #[allow(invalid_value)]
      heap: unsafe { zeroed() },
      entry: &file,
      module_resolver: Box::new(fs_resolver),
      pkg_resolver: Box::new(dll_resolver),
      log_info: Box::new(requested_perm),
      runtime: &*RUNTIME,
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

  pub fn add_pkg_static(&mut self, package: &'static dyn Package) -> &mut Self {
    self.pkg.import_static(package);
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
  ) -> &mut Self {
    let pkg = ImplPackage {
      name,
      methods,
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

    self.inst.elapsed()
  }

  pub fn run(mut self, time: bool) -> ! {
    self.heap = Heap::new(self.pkg.extends.clone());
    let dur = self.run_non();

    if time {
      println!("\nTime Elasped: {:?}", dur);
    }

    process::exit(0)
  }
}
