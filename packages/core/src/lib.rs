#![feature(vec_push_within_capacity)]

use interpreter::{
  error, function, generate, parse,
  types::{BufKeyVal, BufValue, Heap, Options},
  Package,
};

mod array;
mod type_conv;
pub use array::*;
pub use type_conv::*;

pub struct Core;

impl Package for Core {
  fn name(&self) -> &'static [u8] {
    "📦 Lead Programming Language / Core".as_bytes()
  }

  fn methods(
    &self,
  ) -> &'static [(
    &'static str,
    for<'a, 'b, 'c, 'd> fn(&'a Vec<String>, &'b mut Heap, &'c String, &'d mut Options),
  )] {
    &[
      function! {
        "unwrap",
        |args, heap, file, opt| {
          parse!(file + heap + args: -> val);

          match val {
            BufValue::Faillable(val) => match val {
              Ok(val) => {
                opt.set_return_val(
                  *val
                );
              }
              Err(err) => {
                error(&format!("{}", err), file);
              }
            }
            _ => error("Expected Faillable(Result<T, E>) in `-> val`", file)
          }
        }
      },
      ("malloc", malloc),
      ("drop", |args, heap, file, _| {
        let [_, var] = &args[..] else {
          error(
            r#"Invalid arguments in :drop
          Format ---
          - drop $in"#,file
          );
        };

        heap.remove(var);
      }),
      ("typeof", |args, heap, file, opt| {
        let [_, var,] = &args[..] else {
          error(
            r#"Invalid arguments in :typeof
          Format ---
          - typeof $input"#,
            file,
          );
        };

        match heap.get(var) {
          Some(v) => {
            opt.set_return_val(BufValue::Str(v.type_of()));
          }
          None => error(&format!("Variable {} not found", var), file),
        }
      }),
      ("comp", |args, val, file, opt| {
        let [_, a, f, b] = &args[..] else {
          error(
            r#"Invalid arguments in :comp
        Format ---
        - comp $1 = $2
        - comp $1 != $2
        - comp $1 < $2 (only if $1 $2 = number)
        - comp $1 <= $2 (only if $1 $2 = number)
        - comp $1 > $2 (only if $1 $2 = number)
        - comp $1 >= $2 (only if $1 $2 = number)
      "#,
            file,
          );
        };

        let a = val.get(a).expect("Unable to get value of 1st variable");
        let b = val.get(b).expect("Unable to get value of 2nd variable");

        opt.set_return_val(
          BufValue::Bool(match f.as_str() {
            "=" => a == b,
            "!=" => a != b,
            "<" => a.lt(&b),
            "<=" => a.lt(&b) || a == b,
            ">" => a.gt(&b) || a == b,
            ">=" => a.gt(&b) || a == b,
            e => error(&format!("Invalid operator {} in :comp", e), file),
          }),
        );
      }),
      ("mkptr", |args, heap, file, _| {
        let [_, var, point, p, pointer] = &args[..] else {
          error(
            r#"Invalid syntax
          
          Format ---
          - mkptr $arr 0 > *ptr
          - mkptr $map "test" > *ptr"#,
            file,
          );
        };

        if p != ">" {
          error("Invalid pipe operator", file);
        }

        match heap
          .get(var)
          .unwrap_or_else(|| error("Unable to get variable", file))
        {
          BufValue::Array(_) => {
            let ptr = point.parse::<usize>().unwrap_or_else(|_| {
              error("Unable to convert to a pointing", file);
            });
            heap.set_ptr(pointer.clone(), BufKeyVal::Array(ptr));
          }
          BufValue::Object(_) => {
            let _ = heap.set_ptr(pointer.into(), BufKeyVal::Map(point.clone()));
          }
          _ => error("Only ARRAY / OBJECT can be pointered", file),
        }
      }),
    ]
  }
}

fn malloc<'a, 'b, 'c, 'd>(
  args: &'a Vec<String>,
  _: &'b mut Heap,
  file: &'c String,
  opt: &'d mut Options,
) {
  let [_, typ, ..] = &args[..] else {
    error(
      r#"Invalid arguments in :malloc
Format ---
- malloc type data

Types ---
- bool Boolean (eg. true)
- int Integer (eg. 3)
- float Floating point number (eg. 1.04)
- string String (eg. Hello World)
"#,
      file,
    );
  };

  let data = args[3..].join(" ");

  opt.set_return_val(
    match typ.as_str() {
      "bool" => BufValue::Bool(&data == "true"),
      "int" => BufValue::Int(
        data
          .parse()
          .map_or_else(|_| error("Unable to convert to INTEGER", file), |x| x),
      ),
      "float" => BufValue::Float(
        data
          .parse()
          .map_or_else(|_| error("Unable to convert to FLOAT", file), |x| x),
      ),
      "string" => BufValue::Str(data),
      e => error(&format!("Invalid type, {e}"), file),
    },
  );
}

generate!(Core, Array, Types);
