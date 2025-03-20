use interpreter::{phf, Package as TraitPackage, RuntimeValue};
use libloading::Library;
use serde::Serialize;
use std::collections::HashMap;

use super::docs::PackageEntry;

pub struct Package {
  pub name: String,
  pub doc: HashMap<String, HashMap<&'static str, &'static str>>,
  pub runtimes: HashMap<&'static str, (&'static str, HashMap<&'static str, &'static str>)>,
  _inner: Library,
}

#[derive(Debug, Serialize)]
pub struct UnsafePkg<'a> {
  #[serde(borrow)]
  pub name: &'a str,
  #[serde(borrow)]
  pub doc: &'a HashMap<String, HashMap<&'static str, &'static str>>,
  #[serde(borrow)]
  pub runtimes: &'a HashMap<&'static str, (&'static str, HashMap<&'static str, &'static str>)>,
}

impl Serialize for Package {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let pkg = UnsafePkg {
      doc: &self.doc,
      name: &self.name,
      runtimes: &self.runtimes,
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
        .get::<fn() -> &'static [&'static dyn TraitPackage]>(b"modules")
        .expect("Unable to load symbol")();

      let mut doc = HashMap::new();

      for pkg in pkgs {
        let name = String::from_utf8_lossy(pkg.name()).to_string();
        let docs = pkg.doc();

        let docs = docs.into_iter()
          .map(|(k, v)| (k, &v[2] as &'static str))
          .collect();

        doc.insert(name, docs);
      }

      let mut runtimes = HashMap::new();

      let pkgs = library
        .get::<fn() -> phf::map::Entries<'static, &'static str, &'static dyn RuntimeValue>>(b"runtimes")
        .expect("Unable to load symbol")();

      for (key, val) in pkgs {
        let name = val.name();
        let docs = val.doc();

        let docs: HashMap<&'static str, &'static str> = docs.entries()
          .map(|(k, v)| (*k, &v[2] as &'static str))
          .collect();

        runtimes.insert(*key, (name, docs));
      }

      Self {
        _inner: library,
        name: pkg.display.clone(),
        doc,
        runtimes
      }
    }
  }
}
