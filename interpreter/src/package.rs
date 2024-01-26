use crate::{types::PackageCallback, Package};

#[derive(Default)]
pub struct ImplPackage {
  name: &'static str,
  method: Vec<(&'static str, PackageCallback)>,
}

impl ImplPackage {
  pub fn new() -> Self {
    Self {
      name: "💻 ImplPackage [struct]",
      ..Self::default()
    }
  }

  pub fn set_name(mut self, name: &'static str) -> Self {
    self.name = name;
    self
  }

  pub fn add_method(mut self, name: &'static str, callback: PackageCallback) -> Self {
    self.method.push((name, callback));
    self
  }
}

impl Package for ImplPackage {
  fn name(&self) -> &'static str {
    self.name
  }

  fn dyn_methods(&self) -> crate::types::DynMethodRes {
    self.method.clone()
  }
}

#[macro_export]
macro_rules! package {
  ($name:expr, $call:expr) => {
    ImplPackage::new()
      .set_name("💻 ImplPackage [macro]")
      .add_method($name, $call)
  };
}
