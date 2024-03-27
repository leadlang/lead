mod fns;
mod heap;
use std::{collections::HashMap, fmt::Debug};

pub use fns::*;
pub use heap::*;

pub struct Options {
  pub marker: bool,
  pub r_val: Option<BufValue>,
  pub r_ptr: BufKeyVal,
}

impl Debug for Options {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!("Options {{ marker: {:?}, r_val: {:?}, r_ptr: {} }}", &self.marker, &self.r_val, match &self.r_ptr {
      BufKeyVal::None => "None",
      BufKeyVal::Array(_) => "Pending<Array>",
      BufKeyVal::Map(_) => "Pending<Object>",
    }))
  }
}

impl Options {
  pub fn new() -> Self {
    Self {
      marker: false,
      r_ptr: BufKeyVal::None,
      r_val: None
    }
  }

  pub fn set_marker(&mut self) {
    self.marker = true;
  }

  pub fn set_return_val(&mut self, val: BufValue) {
    self.r_val = Some(val);
  }

  pub fn set_return_ptr(&mut self, ptr: BufKeyVal) {
    self.r_ptr = ptr;
  }
}

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

  pub fn get_vec_mut(&mut self) -> Option<&mut Vec<BufValue>> {
    match self {
      BufValue::Array(a) => Some(a),
      _ => None,
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
