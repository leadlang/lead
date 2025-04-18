use crate::{types::MethodRes, Package};

#[derive(Default)]
/// ImplPackage is meant to create a package out of Box<dyn Package>
pub(crate) struct ImplPackage {
  pub(crate) name: &'static [u8],

  pub(crate) methods: MethodRes,
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
/// Exports a leadlang package
///
/// ```rust
/// use interpreter::exports;
///
/// exports! {
///   packages = MyPkg1,MyPkg2;
///   runtimes = {
///     "rt1" = MyRuntime1
///   }
/// }
/// ```
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

    static MODULES: &[&dyn interpreter::Package] = &[
      $(interpreter::generate!(-> $x)),+
    ];

    interpreter::paste! {
      $(
        static [<$val _STATIC>]: &'static dyn interpreter::runtime::RuntimeValue = &$val::new_const();
      )*

      static RUNTIMES: interpreter::phf::Map<&'static str, &'static dyn interpreter::runtime::RuntimeValue> = interpreter::phf::phf_map! {
        $($key => [<$val _STATIC>]),*
      };
    }

    #[no_mangle]
    pub fn modules() -> &'static [&'static dyn interpreter::Package] {
      MODULES
    }

    #[no_mangle]
    pub fn runtimes() -> interpreter::phf::map::Entries<'static, &'static str, &'static dyn interpreter::runtime::RuntimeValue> {
      RUNTIMES.entries()
    }

    #[no_mangle]
    pub fn runtime(id: &str) -> Option<&'static dyn interpreter::runtime::RuntimeValue> {
      let Some(rt) = RUNTIMES.get(id) else {
        return None;
      };

      Some(*rt)
    }
  };
}

#[macro_export]
/// Exports a leadlang package (module-only)
///
/// ```rust
/// use interpreter::generate;
///
/// generate! { Module1, Module2 }
/// ```
macro_rules! generate {
  ($($x:ident),*) => {
    interpreter::exports!(
      packages = $($x),*;
      runtimes = {}
    );
  };

  (-> $x:ident) => {
    &$x
  };
}
