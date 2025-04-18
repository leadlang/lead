use std::{borrow::Cow, collections::HashMap, future::Future, pin::Pin};

use crate::{
  error,
  runtime::_root_syntax::insert_into_application,
  types::{
    call_runtime_val, mkbuf, set_runtime_val, BufValue, Heap, HeapWrapper, Options, Output,
    RawRTValue,
  },
  Application,
};

pub fn schedule<'a>(mut app: &mut Application<'a>) {}
