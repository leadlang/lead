use std::{collections::HashMap, ops::Deref};

use super::BufValue;

pub enum BufKeyVal {
  None,
  Array(usize),
  Map(String),
}

pub struct PtrType {
  pub key: String,
  pub val: BufKeyVal,
}

pub type HeapInnerMap = HashMap<String, BufValue>;
pub type Pointer = HashMap<String, PtrType>;

pub struct Heap {
  data: HeapInnerMap,
  pointer: Pointer,
}

impl Heap {
  pub fn new() -> Self {
    Heap {
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

  pub fn set_ptr(&mut self, key: String, val: BufKeyVal) -> Option<()> {
    if !key.starts_with("*") {
      return None;
    }
    if let BufKeyVal::None = val {
      return None;
    }
    self.pointer.insert(key.clone(), PtrType { key, val });
    Some(())
  }

  pub fn get(&self, key: &String) -> Option<&BufValue> {
    let (ky, typ) = if key.starts_with("*") {
      let ptr = self.pointer.get(key)?;
      (&ptr.key, &ptr.val)
    } else {
      (key, &BufKeyVal::None)
    };

    let val = self.data.get(ky)?;

    if let BufKeyVal::None = typ {
      Some(val)
    } else {
      match (val, typ) {
        (BufValue::Array(val), BufKeyVal::Array(index)) => Some(&val[*index]),
        (BufValue::Object(val), BufKeyVal::Map(key)) => {
          val.get(key).map_or_else(|| None, |x| Some(x.deref()))
        }
        _ => None,
      }
    }
  }

  pub fn remove(&mut self, key: String) -> Option<()> {
    if key.starts_with("*") {
      self.pointer.remove(&key);
      return Some(());
    } else if key.starts_with("$") {
      self.data.remove(&key);
      return Some(());
    }
    None
  }
}
