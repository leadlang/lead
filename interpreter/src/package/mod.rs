use crate::{types::{DynMethodRes, MethodRes, PackageCallback}, Package};


#[derive(Default)]
/// ImplPackage is not meant to create a package out of
pub struct ImplPackage {
  pub name: &'static [u8],

  pub(crate) methods: MethodRes,
  pub(crate) dyn_methods: DynMethodRes,
}

impl Package for ImplPackage {
  fn name(&self) -> &'static [u8] {
    self.name
  }

  fn methods(&self) -> MethodRes {
    self.methods
  }

  fn dyn_methods(&self) -> DynMethodRes {
    self.dyn_methods.clone()
  }
}

#[macro_export]
macro_rules! generate {
  ($x:ident) => {
    #[no_mangle]
    pub fn ver() -> u16 {
      interpreter::VERSION_INT
    }

    #[no_mangle]
    pub fn modules() -> Vec<Box<dyn interpreter::Package>> {
      use interpreter::Package;
      vec![generate!(-> $x)]
    }
  };

  ($($x:ident),+) => {
    #[no_mangle]
    pub fn ver() -> u16 {
      interpreter::VERSION_INT
    }
    
    #[no_mangle]
    pub fn modules() -> Vec<Box<dyn interpreter::Package>> {
      use interpreter::Package;
      vec![$(generate!(-> $x)),+]
    }
  };

  (-> $x:ident) => {
    Box::new($x)
  };
}

#[macro_export]
macro_rules! package {
  ($name:expr, $doc:expr, $call:expr) => {
    ImplPackage::new()
      .set_name("ImplPackage [macro]")
      .add_method($name, $doc, $call)
  };
}

impl ImplPackage {
  pub fn new() -> Self {
    Self {
      name: b"ImplPackage [struct]",
      ..Self::default()
    }
  }

  pub fn set_name(mut self, name: &'static str) -> Self {
    self.name = name.as_bytes();
    self
  }

  pub fn add_method(
    mut self,
    name: &'static str,
    callback: PackageCallback,
  ) -> Self {
    self.dyn_methods.push((name, callback));
    self
  }
}