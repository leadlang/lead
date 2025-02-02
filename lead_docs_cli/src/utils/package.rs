use interpreter::Package as TraitPackage;
use libloading::Library;
use serde::Serialize;
use std::collections::HashMap;

use super::docs::PackageEntry;

pub struct Package {
  pub name: String,
  pub doc: HashMap<String, HashMap<&'static str, &'static str>>,
  _inner: Library,
}

#[derive(Debug, Serialize)]
pub struct UnsafePkg<'a> {
  #[serde(borrow)]
  pub name: &'a str,
  #[serde(borrow)]
  pub doc: &'a HashMap<String, HashMap<&'static str, &'static str>>,
}

impl Serialize for Package {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: serde::Serializer {
      let pkg = UnsafePkg {
        doc: &self.doc,
        name: &self.name
      };

      pkg.serialize(serializer)
  }
}

impl Package {
  pub fn new(pkg: &PackageEntry) -> Self {
    unsafe {
      let path = &pkg.file;
      let library = Library::new(path).expect("Unable to load library");

      let pkgs = library
        .get::<fn() -> Vec<Box<dyn TraitPackage>>>(b"modules")
        .expect("Unable to load symbol")();

      let mut doc = HashMap::new();

      for pkg in pkgs {
        let name = String::from_utf8_lossy(pkg.name()).to_string();
        let docs = pkg.doc();

        doc.insert(name, docs);
      }

      Self {
        _inner: library,
        name: pkg.display.clone(),
        doc,
      }
    }
  }
}
