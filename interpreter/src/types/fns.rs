use crate::Package;
use lealang_chalk_rs::Chalk;
use std::{collections::HashMap, sync::Arc};

use super::{set_into_extends, ExtendsInternal, HeapWrapper, Options};

//pub trait DynPackageCallback = FnMut(&Args, &mut Heap, &mut bool);
pub type Args = *const [*const str];
pub type PackageCallback = fn(Args, HeapWrapper, &String, &mut Options) -> ();

pub type MethodRes = &'static [(&'static str, PackageCallback)];

pub struct LanguagePackages<'a> {
  pub inner: HashMap<&'static str, (&'a str, PackageCallback)>,
  pub(crate) extends: Arc<ExtendsInternal>,
}

impl<'a> LanguagePackages<'a> {
  pub fn new() -> Self {
    Self {
      inner: HashMap::new(),
      extends: Arc::new(ExtendsInternal::default()),
    }
  }

  pub fn import_dyn(&mut self, func: Box<dyn Package>) -> &mut Self {
    let name = String::from_utf8_lossy(func.name());
    let name: &'static mut str = name.to_string().leak::<'static>();
    for (key, val) in func.methods() {
      self.inner.insert(key, (name, *val));
    }

    // SAFETY: These functions cannot be called when the Arc is in use
    let ext = unsafe { Arc::get_mut_unchecked(&mut self.extends) };

    set_into_extends(func.prototype(), ext);

    self
  }

  pub fn import_static(&mut self, func: &'static dyn Package) -> &mut Self {
    let name = String::from_utf8_lossy(func.name());
    let name: &'static mut str = name.to_string().leak::<'static>();
    for (key, val) in func.methods() {
      self.inner.insert(key, (name, *val));
    }

    // SAFETY: These functions cannot be called when the Arc is in use
    let ext = unsafe { Arc::get_mut_unchecked(&mut self.extends) };

    set_into_extends(func.prototype(), ext);

    self
  }

  pub fn import<T: Package + 'static>(&mut self, func: T) -> &mut Self {
    self.import_dyn(Box::new(func))
  }

  pub fn list(&self, chalk: &mut Chalk) {
    println!(
      "{} {}",
      chalk.reset_weight().blue().string(&"Total Commands:"),
      self.inner.len()
    );
    chalk.reset_weight().green().println(&"Commands:");

    self
      .inner
      .iter()
      .enumerate()
      .for_each(|(no, (syntax, (name, _)))| {
        chalk.red().print(&format!("{}- ", no + 1));
        chalk.yellow().bold().print(&syntax);
        print!(" from ");
        chalk.reset_weight().blue().println(&name);
      });
  }
}
