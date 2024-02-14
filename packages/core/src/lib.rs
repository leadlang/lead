use interpreter::{
  error, generate,
  types::{BufKeyVal, BufValue, Heap},
  Package,
};

mod array;
pub use array::Array;

pub struct Core;

impl Package for Core {
  fn name(&self) -> &'static [u8] {
    "📦 Lead Programming Language / Core".as_bytes()
  }

  fn methods(
    &self,
  ) -> &'static [(
    &'static str,
    for<'a, 'b, 'c> fn(&'a Vec<String>, &'b mut Heap, &'c mut bool),
  )] {
    &[
      ("malloc", malloc),
      ("drop", |args, heap, _| {
        let [_, var] = &args[..] else {
          error(
            r#"Invalid arguments in :drop
          Format ---
          - drop $in"#,
          );
        };

        heap.remove(var);
      }),
      ("typeof", |args, heap, _| {
        let [_, var, set] = &args[..] else {
          error(
            r#"Invalid arguments in :typeof
          Format ---
          - typeof $in $result"#,
          );
        };

        match heap.get(var) {
          Some(v) => {
            let _ = heap.set(set.clone(), BufValue::Str(v.type_of()));
          }
          None => error(&format!("Variable {} not found", var)),
        }
      }),
      ("comp", |args, val, _| {
        let [_, a, f, b, pipe, resp] = &args[..] else {
          error(
            r#"Invalid arguments in :comp
        Format ---
        - comp $1 = $2 > $res
        - comp $1 != $2 > res
        - comp $1 < $2 > $res (only if $1 $2 = number)
        - comp $1 <= $2 > $res (only if $1 $2 = number)
        - comp $1 > $2 > $res (only if $1 $2 = number)
        - comp $1 >= $2 > $res (only if $1 $2 = number)
      "#,
          );
        };

        if pipe != ">" {
          error("Invalid pipe operator");
        }

        let a = val.get(a).expect("Unable to get value of 1st variable");
        let b = val.get(b).expect("Unable to get value of 2nd variable");

        val.set(
          resp.into(),
          BufValue::Bool(match f.as_str() {
            "=" => a == b,
            "!=" => a != b,
            "<" => a.lt(&b),
            "<=" => a.lt(&b) || a == b,
            ">" => a.gt(&b) || a == b,
            ">=" => a.gt(&b) || a == b,
            e => error(&format!("Invalid operator {} in :comp", e)),
          }),
        );
      }),
      ("mkptr", |args, heap, _| {
        let [_, var, point, p, pointer] = &args[..] else {
          error(
            r#"Invalid syntax
          
          Format ---
          - mkptr $arr 0 > *ptr
          - mkptr $map "test" > *ptr"#,
          );
        };

        if p != ">" {
          error("Invalid pipe operator");
        }

        match heap
          .get(var)
          .unwrap_or_else(|| error("Unable to get variable"))
        {
          BufValue::Array(_) => {
            let ptr = point.parse::<usize>().unwrap_or_else(|_| {
              error("Unable to convert to a pointing");
            });
            heap.set_ptr(pointer.clone(), BufKeyVal::Array(ptr));
          }
          BufValue::Object(_) => {
            let _ = heap.set_ptr(pointer.into(), BufKeyVal::Map(point.clone()));
          }
          _ => error("Only ARRAY / OBJECT can be pointered"),
        }
      }),
    ]
  }
}

fn malloc<'a, 'b, 'c>(args: &'a Vec<String>, val: &'b mut Heap, _: &'c mut bool) {
  let [_, var, typ, ..] = &args[..] else {
    error(
      r#"Invalid arguments in :malloc
Format ---
- malloc $var type data

Types ---
- bool i.e. boolean
- int Integer (not Decimal)
- float Floating point number (eg. 1.04)
- string String (eg. "Hello World")
"#,
    );
  };

  let data = args[3..].join(" ");

  val.set(
    var.into(),
    match typ.as_str() {
      "bool" => BufValue::Bool(&data == "true"),
      "int" => BufValue::Int(
        data
          .parse()
          .map_or_else(|_| error("Unable to convert to INTEGER"), |x| x),
      ),
      "float" => BufValue::Float(
        data
          .parse()
          .map_or_else(|_| error("Unable to convert to FLOAT"), |x| x),
      ),
      "string" => BufValue::Str(data),
      _ => error("Invalid type"),
    },
  );
}

generate!(Core, Array);
