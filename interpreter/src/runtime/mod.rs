use std::collections::HashMap;

use crate::types::{Args, Heap, Options};

pub type PackageCallback = fn(&Args, &mut Heap, &mut Heap, &String, &mut Options) -> ();
pub type RuntimeMethodRes = HashMap<&'static str, (&'static str, PackageCallback)>;

pub struct RuntimeValue {
  pub _inner: Heap,
  pub fn_ptr: RuntimeMethodRes,
}

impl RuntimeValue {
  pub fn new(fn_ptr: RuntimeMethodRes) -> Self {
    Self {
      _inner: Heap::new(),
      fn_ptr,
    }
  }

  pub fn call_ptr(
    &mut self,
    caller: &str,
    v: &Vec<String>,
    a: &mut Heap,
    c: &String,
    o: &mut Options,
  ) -> Option<()> {
    let (_, f) = self.fn_ptr.get(caller)?;

    f(v, &mut self._inner, a, c, o);
    Some(())
  }
}
