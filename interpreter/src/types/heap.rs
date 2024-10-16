use std::{
  collections::HashMap,
  ops::{Deref, DerefMut},
};

use crate::runtime::RuntimeValue;

use super::{BufValue, Options};

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

pub static mut RUNTIME_VAL: Option<HashMap<String, (&'static str, RuntimeValue)>> = None;

fn get_ptr() -> &'static mut HashMap<String, (&'static str, RuntimeValue)> {
  #[allow(static_mut_refs)]
  unsafe { RUNTIME_VAL.as_mut().unwrap() }
}

pub fn set_runtime_val(key: String, module: &'static str, val: RuntimeValue) {
  let _ = get_ptr().insert(key, (module, val));
}

pub fn call_runtime_val(
  key: &str,
  v: &Vec<String>,
  a: &mut Heap,
  c: &String,
  o: &mut Options,
) -> Option<&'static str> {
  let ptr = get_ptr();

  let (key, caller) = key.split_once("::")?;
  let data = ptr.get_mut(key)?;
  
  data.1.call_ptr(caller, v, a, c, o)?;

  Some(data.0)
}

#[derive(Debug)]
pub struct Heap {
  data: HeapInnerMap,
  pointer: Pointer,
}

impl Heap {
  pub fn new() -> Self {
    unsafe { RUNTIME_VAL = Some(HashMap::new()) };

    Self {
      data: HashMap::new(),
      pointer: HashMap::new(),
    }
  }

  pub fn inner(&mut self) -> &mut HeapInnerMap {
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

  fn get_ptr<'a>(&'a self, key: &'a String) -> Option<(&'a String, &'a BufKeyVal)> {
    if key.starts_with("*") {
      let ptr = self.pointer.get(key)?;
      Some((&ptr.key, &ptr.val))
    } else {
      Some((key, &BufKeyVal::None))
    }
  }

  pub fn get(&self, key: &String) -> Option<&BufValue> {
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

  pub fn get_mut(&mut self, key: &String) -> Option<&mut BufValue> {
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

  pub fn remove(&mut self, key: &String) -> Option<Option<BufValue>> {
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
