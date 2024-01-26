mod fns;
mod heap;
use std::collections::HashMap;

pub use fns::*;
pub use heap::*;

#[derive(Clone, PartialEq)]
pub enum BufValue {
  Int(i64),
  Float(f64),
  Str(String),
  Bool(bool),
  Array(Vec<BufValue>),
  Object(HashMap<String, Box<BufValue>>),
  Faillable(Result<Box<BufValue>, String>),
}

impl BufValue {
  pub fn gt(&self, other: &BufValue) -> bool {
    match (self, other) {
      (BufValue::Int(a), BufValue::Int(b)) => a > b,
      (BufValue::Float(a), BufValue::Float(b)) => a > b,
      _ => false,
    }
  }

  pub fn lt(&self, other: &BufValue) -> bool {
    match (self, other) {
      (BufValue::Int(a), BufValue::Int(b)) => a < b,
      (BufValue::Float(a), BufValue::Float(b)) => a < b,
      _ => false,
    }
  }
}
