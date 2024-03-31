#![feature(vec_push_within_capacity)]
#![feature(concat_idents)]

use interpreter::{
  document, error, function, generate, parse, types::{BufKeyVal, BufValue, Heap, Options}, Package
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
    &'static str,
    for<'a, 'b, 'c, 'd> fn(&'a Vec<String>, &'b mut Heap, &'c String, &'d mut Options),
  )] {
    &[
      function! {
        "unwrap",
        document!(""),
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
      ("malloc", document!(""), malloc),
      ("drop", document!(""), |args, heap, file, _| {
        parse!(file + heap + args: -> var);
        drop(var);
      }),
      ("typeof", document!(""), |args, heap, file, opt| {
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
      ("comp", document!(""), |args, val, file, opt| {
        let [_, a, f, b] = &args[..] else {
          error(
            r#"Invalid arguments in :comp
        Format ---
        - comp $1 == $2
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
            "==" => a.eq(b),
            "!=" => !a.eq(b),
            "<" => a.lt(&b),
            "<=" => a.lt(&b) || a.eq(b),
            ">" => a.gt(&b),
            ">=" => a.gt(&b) || a.eq(b),
            e => error(&format!("Invalid operator {} in :comp", e), file),
          }),
        );
      })
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
- int Integer (eg. -3, 3)
- u_int Unsigned Integer (eg. 3)
- float Floating point number (eg. 1.04)
- string String (eg. Hello World)
"#,
      file,
    );
  };

  let data = args[2..].join(" ");

  opt.set_return_val(
    match typ.as_str() {
      "bool" => BufValue::Bool(&data == "true"),
      "int" => BufValue::Int(
        data
          .parse()
          .map_or_else(|_| error("Unable to convert to INTEGER", file), |x| x),
      ),
      "u_int" => BufValue::U_Int(
        data
          .parse()
          .map_or_else(|_| error("Unable to convert to UNSIGNED INTEGER", file), |x| x),
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