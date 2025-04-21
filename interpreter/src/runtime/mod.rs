use std::collections::HashMap;

use crate::types::{HeapWrapper, Options};

pub mod _root_syntax;

pub trait RuntimeValue: Sync {
  fn doc(&self) -> HashMap<&'static str, &'static [&'static str; 3]>;

  fn name(&self) -> &'static str;

  fn call_ptr(
    &mut self,
    caller: &str,
    v: *const [&'static str],
    a: HeapWrapper,
    c: &str,
    o: &mut Options,
  ) -> Option<()>;
}
