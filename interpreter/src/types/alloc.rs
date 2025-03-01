use crate::error;

use super::BufValue;

pub fn mkbuf(data: &str, file: &str) -> BufValue {
  let (_, val) = data.split_at(1);

  let (header, value) = val.split_at(1);

  match header {
    "'" => {
      // Check if decimal
      if value.contains(".") {
        BufValue::Float(value.parse().unwrap())
      }
      // Check if unsigned
      else if let Ok(v) = value.parse() {
        BufValue::U_Int(v)
      }
      // Otherwise
      else {
        BufValue::Int(value.parse().unwrap())
      }
    }
    "f" => BufValue::Float(value.parse().unwrap()),
    "u" => BufValue::U_Int(value.parse().unwrap()),
    "i" => BufValue::Int(value.parse().unwrap()),
    "s" => {
      if !val.ends_with("\"") {
        error("Unknown String closing, expected `\"`", file);
      }
      if !val.starts_with("\"") {
        error("Unknown String starting, expected `\"`", file);
      }
      BufValue::Str(val[2..].into())
    }
    "1" => BufValue::Bool(true),
    "0" => BufValue::Bool(false),
    e => {
      error(format!("Unknown header `{}` used", e), file);
    }
  }
}
