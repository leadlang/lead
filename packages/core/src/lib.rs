use interpreter::{
  error,
  types::{BufValue, Heap},
  Package,
};

pub struct Core;

impl Package for Core {
  fn name(&self) -> &'static str {
    "📦 Lead Programming Language / Core"
  }

  fn methods(
    &self,
  ) -> &'static [(
    &'static str,
    for<'a, 'b, 'c> fn(&'a Vec<String>, &'b mut Heap, &'c mut bool),
  )] {
    &[
      (":malloc", malloc),
      (":array::malloc", |args, val, _| {
        let [_, a] = &args[..] else {
          error(
            r#"Invalid arguments in :array::malloc
        Format ---
        - :array::malloc $1"#,
          )
        };

        val.set(a.clone(), BufValue::Array(vec![]));
      }),
      (":comp", |args, val, _| {
        let [_, a, f, b, _, resp] = &args[..] else {
          error(
            r#"Invalid arguments in :comp
        Format ---
        - :comp $1 = $2 @ $res
        - :comp $1 != $2 @ res
        - :comp $1 < $2 @ $res (only if $1 $2 = number)
        - :comp $1 <= $2 @ $res (only if $1 $2 = number)
        - :comp $1 > $2 @ $res (only if $1 $2 = number)
        - :comp $1 >= $2 @ $res (only if $1 $2 = number)
      "#,
          );
        };

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
    ]
  }
}

fn malloc<'a, 'b, 'c>(args: &'a Vec<String>, val: &'b mut Heap, _: &'c mut bool) {
  let [_, var, typ, ..] = &args[..] else {
    error(
      r#"Invalid arguments in :malloc
Format ---
- :malloc $var type data

Types ---
- 0 f64 i.e. float
- 1 String
- 2 Boolean; data can only be `true` / `false`
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
