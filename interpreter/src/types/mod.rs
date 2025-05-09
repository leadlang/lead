mod alloc;
mod fns;
mod heap;
mod heap_wrap;
use std::{
  collections::HashMap,
  fmt::Debug,
  ops::{Deref, DerefMut},
  sync::{Arc, Mutex},
  thread::JoinHandle,
};
pub use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub use alloc::*;
pub use fns::*;
pub use heap::*;
pub use heap_wrap::*;

use crate::runtime::RuntimeValue;

pub struct RetBufValue(pub BufValue);

impl From<BufValue> for RetBufValue {
  fn from(item: BufValue) -> Self {
    Self(item)
  }
}

pub struct Options {
  pub pre: *const str,
  pub r_val: Option<RetBufValue>,
  r_runtime: Option<Box<dyn RuntimeValue>>,
}

unsafe impl Send for Options {}
unsafe impl Sync for Options {}

impl Debug for Options {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("Options {{ <inner> }}"))
  }
}

impl Options {
  pub fn new() -> Self {
    Self {
      pre: "" as _,
      r_val: None,
      r_runtime: None,
    }
  }

  pub fn set_return_val(&mut self, val: BufValue) {
    self.r_val = Some(RetBufValue(val));
  }

  pub(crate) fn r_val(self) -> BufValue {
    let val = self.r_val;

    val.expect("Error").0
  }

  pub(crate) fn rem_r_runtime(&mut self) -> Option<Box<dyn RuntimeValue>> {
    self.r_runtime.take()
  }

  pub fn set_r_runtime(&mut self, val: Box<dyn RuntimeValue>) {
    self.r_runtime = Some(val);
  }
}

#[derive(PartialEq, Debug)]
pub struct StrPointer(pub *const str);

impl ToString for StrPointer {
  fn to_string(&self) -> String {
    unsafe { (*self.0).to_string() }
  }
}

macro_rules! extends {
    (
      $(
        $good:ident $x:ident => $y:ty
      ),*
    ) => {
      #[derive(Default, Clone, Debug)]
      pub(crate) struct ExtendsInternal {
        $(
          pub(crate) $x: HashMap<&'static str, fn(*mut $y, Args, HeapWrapper, &str, &mut Options) -> ()>
        ),*
      }

      #[derive(Default)]
      pub struct Extends {
        $(
          pub $x: &'static [(&'static str, fn(*mut $y, Args, HeapWrapper, &str, &mut Options) -> ())]
        ),*
      }

      #[derive(Default)]
      pub struct PrototypeDocs {
        $(
          pub $x: HashMap<&'static str, &'static [&'static str; 3]>
        ),*
      }

      pub(crate) fn get_handle_runtime_ptr<'a>(
        heap: &mut Heap,
        val: &BufValue,
        caller: &'a str,
      ) -> Option<*const ()> {
        let (ext, ar) = heap.get_extends();

        crate::paste! {
          match val {
            $(
              BufValue::[<$good>](_) => {
                let scope1 = &ext.$x;
                let scope2 = &ar.$x;

                // Optimized approach
                let f = match scope1.get(caller) {
                  Some(v) => v,
                  None => match scope2.get(caller) {
                      Some(v) => v,
                      None => { return None }
                  },
                };

                return Some(f as *const _ as *const ());
              }
            )*
            _ => return None
          }
        }
      }

      pub(crate) fn handle_runtime<'a>(
        heap: &mut Heap,
        val: &mut BufValue,
        caller: &'a str,
        v: &'a [&'static str],
        a: HeapWrapper,
        c: &str,
        o: &'a mut Options,
      ) -> Option<()> {
        let (ext, ar) = heap.get_extends();

        crate::paste! {
          match val {
            $(
              BufValue::[<$good>](data) => {
                let scope1 = &ext.$x;
                let scope2 = &ar.$x;

                // Optimized approach
                let f = match scope1.get(caller) {
                  Some(v) => v,
                  None => match scope2.get(caller) {
                      Some(v) => v,
                      None => { return None }
                  },
                };

                f(data as _, v, a, c, o);
              }
            )*
            _ => return None
          }
        }

        Some(())
      }

      pub(crate) fn set_into_extends(extends: Extends, extends_internal: &mut ExtendsInternal) {
        $(
          for (k, v) in extends.$x.into_iter() {
            if let Some(_) = extends_internal.$x.insert(k, *v) {
              panic!("{} already exists. Please ensure that you do not have two prototypes with the same name FOR THE WHOLE APPLICATION", k);
            }
          }
        )*
      }
    };
}

extends! {
  Int int => i64,
  U_Int uint => u64,
  Float float => f64,
  Str str_slice => String,
  Bool boolean => bool,
  Array array => Vec<BufValue>,
  Object object => HashMap<String, Box<BufValue>>,
  Faillable faillable => Result<Box<BufValue>, String>,
  StrPointer str_ptr => StrPointer,
  Pointer ptr => *const BufValue,
  PointerMut mut_ptr => *mut BufValue,
  AsyncTask async_task => AppliesEq<JoinHandle<BufValue>>,
  Sender sender => AppliesEq<UnboundedSender<BufValue>>,
  Listener listener => AppliesEq<UnboundedReceiver<BufValue>>,
  ArcPointer arc_ptr => Arc<Box<BufValue>>,
  ArcMutexPointer arc_mut_ptr => AppliesEq<Arc<Mutex<Box<BufValue>>>>
}

#[allow(non_camel_case_types)]
#[derive(PartialEq, Debug)]
pub enum BufValue {
  Int(i64),
  U_Int(u64),
  Float(f64),
  Str(String),
  Bool(bool),
  Array(Vec<Self>),
  Object(HashMap<String, Box<Self>>),
  Faillable(Result<Box<Self>, String>),
  StrPointer(StrPointer),
  Pointer(*const Self),
  PointerMut(*mut Self),
  ArcPointer(Arc<Box<Self>>),
  ArcMutexPointer(AppliesEq<Arc<Mutex<Box<Self>>>>),
  AsyncTask(AppliesEq<JoinHandle<Self>>),
  Sender(AppliesEq<UnboundedSender<Self>>),
  Listener(AppliesEq<UnboundedReceiver<Self>>),
  RuntimeRaw(AppliesEq<RawRTValue>),
}

unsafe impl Send for BufValue {}
unsafe impl Sync for BufValue {}

macro_rules! implement_buf {
  ($($i:ident => $x:ty),*) => {
    $(
      impl From<$x> for BufValue {
        fn from(item: $x) -> Self {
          Self::$i(item)
        }
      }
    )*
  };
}

implement_buf! {
  Str => String,
  Int => i64,
  U_Int => u64,
  Float => f64,
  Bool => bool,
  StrPointer => StrPointer,
  AsyncTask => AppliesEq<JoinHandle<Self>>
}

#[derive(Debug)]
pub struct AppliesEq<T>(pub T);

unsafe impl<T> Send for AppliesEq<T> {}
unsafe impl<T> Sync for AppliesEq<T> {}

impl<T> PartialEq for AppliesEq<T> {
  fn eq(&self, _: &Self) -> bool {
    false
  }
}

impl<T> Deref for AppliesEq<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<T> DerefMut for AppliesEq<T> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl BufValue {
  pub fn type_of(&self) -> String {
    match &self {
      BufValue::Array(_) => "array".to_string(),
      BufValue::Bool(_) => "bool".to_string(),
      BufValue::Float(_) => "float".to_string(),
      BufValue::Int(_) => "int".to_string(),
      BufValue::U_Int(_) => "u_int".to_string(),
      BufValue::Object(_) => "object".to_string(),
      BufValue::StrPointer(_) | BufValue::Str(_) => "string".to_string(),
      BufValue::Faillable(res) => match res {
        Ok(t) => format!("<success {}>", t.type_of()),
        Err(t) => format!("<err {}>", &t),
      },
      BufValue::Pointer(ptr) => {
        if ptr.is_null() {
          return "<ptr *ref NULL>".into();
        }

        unsafe { &**ptr }.type_of()
      }
      BufValue::PointerMut(ptr) => {
        if ptr.is_null() {
          return "<ptr *mut NULL>".into();
        }

        unsafe { &**ptr }.type_of()
      }
      BufValue::Sender(_) => "<sender ?event>".into(),
      BufValue::Listener(_) => "<listener ?event>".into(),
      BufValue::AsyncTask(t) => {
        if t.is_finished() {
          "<async recv...\\0>".into()
        } else {
          "<async pending...>".into()
        }
      }
      BufValue::RuntimeRaw(_) => "<runtime rt>".into(),
      BufValue::ArcPointer(a) => a.type_of(),
      BufValue::ArcMutexPointer(_) => format!("<mutex *>"),
    }
  }

  pub fn display(&self) -> String {
    match &self {
      BufValue::Array(c) => c.iter().map(|x| x.display()).collect::<Vec<_>>().join(", "),
      BufValue::Bool(a) => a.to_string(),
      BufValue::Float(f) => f.to_string(),
      BufValue::Int(i) => i.to_string(),
      BufValue::U_Int(u) => u.to_string(),
      BufValue::Object(c) => format!("{c:#?}"),
      BufValue::Str(c) => c.to_string(),
      BufValue::StrPointer(c) => c.to_string(),
      e => e.type_of(),
    }
  }

  pub fn get_vec_mut(&mut self) -> Option<&mut Vec<BufValue>> {
    match self {
      BufValue::Array(a) => Some(a),
      _ => None,
    }
  }

  pub fn gt(&self, other: &BufValue) -> bool {
    match (self, other) {
      (BufValue::Int(a), BufValue::Int(b)) => a > b,
      (BufValue::Int(a), BufValue::U_Int(b)) => (*a as i128) > (*b as i128),
      (BufValue::U_Int(a), BufValue::U_Int(b)) => a > b,
      (BufValue::U_Int(a), BufValue::Int(b)) => (*a as i128) > (*b as i128),
      (BufValue::Float(a), BufValue::Float(b)) => a > b,
      _ => false,
    }
  }

  pub fn lt(&self, other: &BufValue) -> bool {
    match (self, other) {
      (BufValue::Int(a), BufValue::Int(b)) => a < b,
      (BufValue::Int(a), BufValue::U_Int(b)) => (*a as i128) < (*b as i128),
      (BufValue::U_Int(a), BufValue::U_Int(b)) => a < b,
      (BufValue::U_Int(a), BufValue::Int(b)) => (*a as i128) < (*b as i128),
      (BufValue::Float(a), BufValue::Float(b)) => a < b,
      _ => false,
    }
  }

  pub fn eq(&self, other: &BufValue) -> bool {
    match (self, other) {
      (BufValue::Int(a), BufValue::U_Int(b)) => (*a as i128) == (*b as i128),
      (BufValue::U_Int(a), BufValue::Int(b)) => (*a as i128) == (*b as i128),
      _ => self == other,
    }
  }
}
