#![feature(vec_push_within_capacity)]
#![feature(concat_idents)]

use std::collections::HashMap;
use indoc::indoc;
use interpreter::{
  error, function, generate, hashmap, parse, types::{BufValue, HeapWrapper, MethodRes, Options}, Package
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

  fn doc(&self) -> HashMap<&'static str, &'static str> {
    hashmap! {
      "unwrap" => indoc! {"
        Unwraps a value in place

        ## Format:
        ```
        $val: unwrap ->$result
        ```
      "},
      "malloc" => indoc!{"
        Memory allocate

        ## Format:
        ```
        $val: malloc %type% %data%
        ```

        Types ---
          - bool Boolean (eg. true)
          - int Integer (eg. -3, 3)
          - u_int Unsigned Integer (eg. 3)
          - float Floating point number (eg. 1.04)
          - string String (eg. Hello World)
      "},
      "drop" => indoc! {"
        Drops a value

        ## Format:
        ```
        drop ->$val
        ```

        ## Note:
          - This function is not magic, it can be reproduced in lead lang too using lead interpreter modules. Example:
            ```
            __declare_global custom
              _fn drop ->$ap
              
              _end
            __end
            ```

            and can be called as such
            ```
            custom drop ->$val
            ```
          - This function is better optimized than the above
      "}
    }
  }

  fn methods(
    &self,
  ) -> MethodRes {
    &[
      function! {
        "unwrap",
        |args, mut heap, file, opt| {
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
      ("malloc",  malloc),
      ("drop",  |args, mut heap, file, _| {
        parse!(file + heap + args: -> var);
        drop(var);
      }),
      ("typeof",  |args, heap, file, opt| {
        let [_, var,] = &args[..] else {
          error(
            r#"Invalid arguments in :typeof
          Format ---
          - typeof $input"#,
            file,
          );
        };

        let var = unsafe { &**var };

        match heap.get(var) {
          Some(v) => {
            opt.set_return_val(BufValue::Str(v.type_of()));
          }
          None => error(&format!("Variable {} not found", var), file),
        }
      }),
      ("comp",  |args, val, file, opt| {
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

        let a = unsafe { &**a };
        let f = unsafe { &**f };
        let b = unsafe { &**b };

        let a = val.get(a).expect("Unable to get value of 1st variable");
        let b = val.get(b).expect("Unable to get value of 2nd variable");

        opt.set_return_val(
          BufValue::Bool(match f {
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

fn malloc<'a, 'c, 'd>(
  args: &'a Vec<*const str>,
  _: HeapWrapper,
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

  let typ = unsafe { &**typ };

  let data = args[2..].iter().map(|x| unsafe { &**x }).collect::<Vec<_>>().join(" ");

  opt.set_return_val(
    match typ {
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
      "string" => BufValue::Str(serde_json::from_str(&data).map_or_else(|_| error("Unable to convert to STRING", file), |x| x)),
      e => error(&format!("Invalid type, {e}"), file),
    },
  );
}

generate!(Core, Array, Types);