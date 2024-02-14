mod fns;
mod heap;
use std::collections::HashMap;

pub use fns::*;
pub use heap::*;

#[derive(Clone, PartialEq, Debug)]
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
  pub fn type_of(&self) -> String {
    match &self {
      BufValue::Array(_) => "array".to_string(),
      BufValue::Bool(_) => "bool".to_string(),
      BufValue::Float(_) => "float".to_string(),
      BufValue::Int(_) => "int".to_string(),
      BufValue::Object(_) => "object".to_string(),
      BufValue::Str(_) => "string".to_string(),
      BufValue::Faillable(res) => match res {
        Ok(t) => format!("<success {}>", t.type_of()),
        Err(t) => format!("<err {}>", &t),
      },
    }
  }

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
