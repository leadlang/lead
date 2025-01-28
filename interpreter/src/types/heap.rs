use std::{
  collections::HashMap,
  ops::{Deref, DerefMut},
};

use crate::{error, runtime::RuntimeValue};

use super::{BufValue, HeapWrapper, Options, PackageCallback};

#[derive(Debug)]
pub enum BufKeyVal {
  None,
  Array(usize),
  Map(String),
}

#[derive(Debug)]
pub struct PtrType {
  pub key: String,
  pub val: BufKeyVal,
}

pub type HeapInnerMap = HashMap<String, BufValue>;
pub type Pointer = HashMap<String, PtrType>;

#[derive(Debug)]
pub enum RawRTValue {
  RT(RuntimeValue),
  PKG(HashMap<String, PackageCallback>),
}

fn get_ptr(heap: &mut Heap) -> &mut HashMap<String, (&'static str, RawRTValue)> {
  &mut heap.runtimes
}

pub fn set_runtime_val(heap: &mut Heap, key: String, module: &'static str, val: RawRTValue) {
  let _ = get_ptr(heap).insert(key, (module, val));
}

pub fn call_runtime_val(
  heap: &mut Heap,
  key: &str,
  v: &Vec<*const str>,
  a: HeapWrapper,
  c: &String,
  o: &mut Options,
  file: &str,
) -> Option<&'static str> {
  let ptr = get_ptr(heap);

  let (key, caller) = key.split_once("::")?;
  let data = ptr.get_mut(key)?;

  match &mut data.1 {
    RawRTValue::RT(data) => data.call_ptr(caller, v, a, c, o)?,
    RawRTValue::PKG(pkg) => match pkg.get_mut(caller) {
      Some(x) => x.call_mut((v, a, c, o)),
      None => error(&format!("Unexpected `{}`", &caller), &file),
    },
  }

  Some(data.0)
}

#[derive(Debug)]
pub struct Heap {
  data: HeapInnerMap,
  pointer: Pointer,
  runtimes: HashMap<String, (&'static str, RawRTValue)>,
}

impl Heap {
  pub fn new() -> Self {
    Self {
      data: HashMap::new(),
      pointer: HashMap::new(),
      runtimes: HashMap::new(),
    }
  }

  pub fn clear(&mut self) {
    *self = Self::new();
  }

  #[deprecated]
  pub fn inner(&mut self) -> &mut HeapInnerMap {
    &mut self.data
  }

  pub fn inner_heap(&mut self) -> &mut HeapInnerMap {
    &mut self.data
  }

  pub fn set(&mut self, key: String, val: BufValue) -> Option<()> {
    if !key.starts_with("$") {
      return None;
    }
    self.data.insert(key, val);
    Some(())
  }

  pub fn set_ptr(&mut self, key: String, ptr_target: String, val: BufKeyVal) -> Option<()> {
    if !key.starts_with("*") {
      return None;
    }
    if let BufKeyVal::None = val {
      return None;
    }

    self.pointer.insert(
      key,
      PtrType {
        key: ptr_target,
        val,
      },
    );
    Some(())
  }

  fn get_ptr<'a>(&'a self, key: &'a str) -> Option<(&'a str, &'a BufKeyVal)> {
    if key.starts_with("*") {
      let ptr = self.pointer.get(key)?;
      Some((&ptr.key, &ptr.val))
    } else {
      Some((key, &BufKeyVal::None))
    }
  }

  pub fn get(&self, key: &str) -> Option<&BufValue> {
    let key = key.replacen("->", "", 1).replacen("&", "", 1);

    let (ky, typ) = self.get_ptr(&key)?;
    let val = self.data.get(ky)?;

    if let BufKeyVal::None = typ {
      Some(val)
    } else {
      match (val, typ) {
        (BufValue::Array(val), BufKeyVal::Array(index)) => val.get(*index),
        (BufValue::Object(val), BufKeyVal::Map(key)) => Some(Box::deref(val.get(key)?)),
        _ => None,
      }
    }
  }

  pub fn get_mut(&mut self, key: &str) -> Option<&mut BufValue> {
    if !key.starts_with("->&$") {
      return None;
    };

    let key = key.replacen("->", "", 1).replacen("&", "", 1);

    let (ky, typ) = if key.starts_with("*") {
      let ptr = self.pointer.get(&key)?;
      (&ptr.key, &ptr.val)
    } else {
      (&key, &BufKeyVal::None)
    };
    let val = self.data.get_mut(ky)?;

    if let BufKeyVal::None = typ {
      Some(val)
    } else {
      match (val, typ) {
        (BufValue::Array(val), BufKeyVal::Array(index)) => val.get_mut(*index),
        (BufValue::Object(val), BufKeyVal::Map(key)) => val
          .get_mut(key)
          .map_or_else(|| None, |x| Some(Box::deref_mut(x))),
        _ => None,
      }
    }
  }

  pub fn remove(&mut self, key: &str) -> Option<Option<BufValue>> {
    if !key.starts_with("->$") {
      return None;
    }

    let key = key.replacen("->", "", 1);

    if key.starts_with("*") {
      self.pointer.remove(&key);
      return Some(None);
    } else if key.starts_with("$") {
      return Some(self.data.remove(&key));
    }
    None
  }
}
