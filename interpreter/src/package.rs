use crate::{
  types::{DynMethodRes, MethodRes, PackageCallback},
  Package,
};

#[derive(Default)]
pub struct ImplPackage {
  pub methods: MethodRes,
  pub name: &'static [u8],
  pub dyn_methods: DynMethodRes,
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
    doc: &'static str,
    callback: PackageCallback,
  ) -> Self {
    self.dyn_methods.push((name, doc, callback));
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
macro_rules! docs {
  () => {};
}

#[macro_export]
macro_rules! generate {
  ($x:ident) => {
    #[no_mangle]
    pub fn modules() -> Vec<(&'static [u8], interpreter::types::MethodRes, interpreter::types::DynMethodRes)> {
      use interpreter::Package;
      vec![generate!(-> $x)]
    }
  };

  (-> $y:ident) => {
    ($y.name(), $y.methods(), $y.dyn_methods())
  };

  ($($x:ident),+) => {
    #[no_mangle]
    pub fn modules() -> Vec<(&'static [u8], interpreter::types::MethodRes, interpreter::types::DynMethodRes)> {
      use interpreter::Package;
      vec![$(generate!(-> $x)),+]
    }
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

#[macro_export]
macro_rules! doc {
  ($x:expr) => {
    fn main() {
      use std::{
        fs::{create_dir_all, remove_dir_all, write, File},
        io::Write,
      };
      let modules = $x;

      let _ = remove_dir_all("./docs");
      println!("📦 Creating docs dir...");
      create_dir_all("./docs").unwrap();

      let mut map = File::create("./docs/file").unwrap();
      let mut pkg = File::create("./docs/pkg").unwrap();

      let mut index: u64 = 0;
      for (name, s_method, dyn_method) in modules {
        index += 1;
        let name = String::from_utf8_lossy(name);

        let path = format!("./docs/{index}");
        create_dir_all(&path).unwrap();

        pkg
          .write_all(format!("{index}->{}/index.md\n", &path).as_bytes())
          .unwrap();

        write(
          format!("{path}/index.md"),
          format!(
            "# {}\n- {} &'static Methods\n- {} &'a dyn Methods",
            &name,
            s_method.len(),
            dyn_method.len()
          ),
        )
        .unwrap();

        let mut mem_len = 0;

        let mut mk_doc = |m_name: &str, doc: &str| {
          mem_len += 1;

          map
            .write_all(format!("{}->{}->{}/{}\n", &m_name, &index, &path, mem_len).as_bytes())
            .unwrap();

          write(
            format!("{path}/{mem_len}.md"),
            format!(
              "# {}\n- **From:** {}\n\n## Description\n{}",
              &m_name, &name, &doc
            ),
          )
          .unwrap();
        };

        for (m_name, doc, _) in s_method {
          mk_doc(m_name, &doc);
        }

        for (m_name, doc, _) in dyn_method {
          mk_doc(m_name, &doc);
        }
      }

      map.flush().unwrap();
      pkg.flush().unwrap();

      println!("🚀 Docs Generated at ./docs");
    }
  };
}
