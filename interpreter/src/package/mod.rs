use crate::{
  types::MethodRes,
  Package,
};

#[derive(Default)]
/// ImplPackage is not meant to create a package out of
pub struct ImplPackage {
  pub name: &'static [u8],

  pub(crate) methods: MethodRes
}

impl Package for ImplPackage {
  fn name(&self) -> &'static [u8] {
    self.name
  }

  fn methods(&self) -> MethodRes {
    self.methods
  }
}

#[macro_export]
macro_rules! exports {
  (
    packages = $($x:ident),*;
    runtimes = {
      $($key:literal = $val:ident),*
    }
  ) => {
    #[no_mangle]
    pub fn ver() -> u16 {
      interpreter::VERSION_INT
    }

    #[no_mangle]
    pub fn modules() -> Vec<Box<dyn interpreter::Package>> {
      use interpreter::Package;
      vec![$(interpreter::generate!(-> $x),)*]
    }

    #[no_mangle]
    #[allow(unused_mut)]
    pub fn runtimes() -> std::collections::HashMap<&'static str, Box<dyn interpreter::runtime::RuntimeValue>> {
      let mut coll = std::collections::HashMap::new();

      $(
        coll.insert($key, $val);
      )*

      coll
    }
  };
}

#[macro_export]
macro_rules! generate {
  ($($x:ident),*) => {
    #[no_mangle]
    pub fn ver() -> u16 {
      interpreter::VERSION_INT
    }

    #[no_mangle]
    pub fn modules() -> Vec<Box<dyn interpreter::Package>> {
      use interpreter::Package;
      vec![$(generate!(-> $x)),+]
    }

    #[no_mangle]
    pub fn runtimes() -> std::collections::HashMap<&'static str, Box<dyn interpreter::runtime::RuntimeValue>> {
      std::collections::HashMap::new()
    }
  };

  (-> $x:ident) => {
    Box::new($x)
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
}
