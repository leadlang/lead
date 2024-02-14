use crate::{types::PackageCallback, Package};

#[derive(Default)]
pub struct ImplPackage {
  pub methods: &'static [(&'static str, PackageCallback)],
  pub name: &'static [u8],
  pub dyn_methods: Vec<(&'static str, PackageCallback)>,
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

  pub fn add_method(mut self, name: &'static str, callback: PackageCallback) -> Self {
    self.dyn_methods.push((name, callback));
    self
  }
}

impl Package for ImplPackage {
  fn name(&self) -> &'static [u8] {
    self.name
  }

  fn methods(&self) -> crate::types::MethodRes {
    self.methods
  }

  fn dyn_methods(&self) -> crate::types::DynMethodRes {
    self.dyn_methods.clone()
  }
}

#[macro_export]
macro_rules! generate {
  ($y:ident) => {
    ($y.name(), $y.methods(), $y.dyn_methods())
  };

  ($($x:ident),+) => {
    #[no_mangle]
    pub fn modules() -> Vec<(&'static [u8], interpreter::types::MethodRes, interpreter::types::DynMethodRes)> {
      use interpreter::Package;
      vec![$(generate!($x)),+]
    }
  };
}

#[macro_export]
macro_rules! package {
  ($name:expr, $call:expr) => {
    ImplPackage::new()
      .set_name("ImplPackage [macro]")
      .add_method($name, $call)
  };
}
