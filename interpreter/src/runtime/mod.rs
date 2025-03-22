use std::collections::HashMap;

use crate::types::{Args, Heap, HeapWrapper, Options};

pub type PackageCallback = fn(Args, &mut Heap, HeapWrapper, &String, &mut Options) -> ();
pub type RuntimeMethodRes = HashMap<&'static str, (&'static str, PackageCallback)>;

pub mod _root_syntax;

pub trait RuntimeValue: Sync {
  fn doc(&self) -> HashMap<&'static str, &'static [&'static str; 3]>;

  fn name(&self) -> &'static str;

  fn call_ptr(
    &mut self,
    caller: &str,
    v: *const [*const str],
    a: HeapWrapper,
    c: &String,
    o: &mut Options,
  ) -> Option<()>;
}

// #[derive(Debug)]
// pub struct RuntimeValue {
//   pub r#type: String,
//   pub _inner: Heap,
//   pub fn_ptr: RuntimeMethodRes,
// }

// impl RuntimeValue {
//   pub fn new(r#type: &str, fn_ptr: RuntimeMethodRes) -> Self {
//     Self {
//       r#type: format!("{}", r#type),
//       _inner: Heap::new(),
//       fn_ptr,
//     }
//   }

//   pub fn call_ptr(
//     &mut self,
//     caller: &str,
//     v: *const [*const str],
//     a: HeapWrapper,
//     c: &String,
//     o: &mut Options,
//   ) -> Option<()> {
//     let (_, f) = self.fn_ptr.get(caller)?;

//     f(v, &mut self._inner, a, c, o);
//     Some(())
//   }
// }
