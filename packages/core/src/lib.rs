#![feature(vec_push_within_capacity)]
#![feature(concat_idents)]

use interpreter::{
  error, generate, pkg_name, module,
  types::{BufValue, HeapWrapper, Options},
};
use lead_lang_macros::{define, methods, gendoc};

mod array;
mod type_conv;

use array::Array;
use type_conv::Types;

module! {
  Core,
  pkg_name! { "📦 Core / Memory" }
  methods! {
    malloc=malloc, 
    unwrap=unwrap,
    drop=memclear,
    comp=comp,
    typeof=kindof
  }
}

#[gendoc((
  desc: "Allocate Memory",
  usage: [
    (
      desc: "Allocating String",
      code: "$val: malloc string \"Hello World\""
    ),
    (
      desc: "Allocating Number",
      code: "$val: malloc string \"Hello World\""
    ),
  ],
  notes: Some("Available Types ---
- bool Boolean (eg. malloc bool true)
- int 64-Bit Signed Integer (eg. -3, 3)
- u_int 64-Bit Unsigned Integer (eg. 0, 3)
- float Floating point number (eg. 1.04, 10.6)
- string String (eg. \"Hello World\")")
))]
fn malloc(args: *const [*const str], _: HeapWrapper, file: &String, opt: &mut Options) {
  let [_, typ, ..] = &(unsafe { &*args })[..] else {
    error(
      r#"Invalid arguments in :malloc
Format ---
- malloc type data

Types ---
- bool Boolean (eg. true)
- int Integer (eg. -3, 3)
- u_int Unsigned Integer (eg. 3)
- float Floating point number (eg. 1.04)
- string String (eg. "Hello World")
"#,
      file,
    );
  };

  let typ = unsafe { &**typ };

  let data = unsafe { &*args }[2..].iter().map(|x| unsafe { &**x }).collect::<Vec<_>>().join(" ");

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

#[define((
  desc: "Unwrap value",
  usage: [
    (
      desc: "",
      code: "$val: unwrap ->$val"
    )
  ],
  notes: None
))]
fn unwrap(val: BufValue) -> BufValue {
  match val {
    BufValue::Faillable(val) => match val {
      Ok(val) => {
        *val
      }
      Err(err) => {
        error(&format!("{}", err), file);
      }
    }
    _ => error("Expected Faillable(Result<T, E>) in `-> val`", file)
  }
}

#[define((
  desc: "Removes the value from memory",
  usage: [
    (
      desc: "Example",
      code: "memclear ->$val"
    )
  ],
  notes: None
))]
fn memclear(_val: BufValue) {}

#[define((
  desc: "Finds the type of a variable",
  usage: [
    (
      desc: "typeof String",
      code: "$val: malloc string \"Hello World\"\n$typeof: typeof $val"
    )
  ],
  notes: None
))]
fn kindof(val: &BufValue) -> BufValue {
  BufValue::Str(val.type_of())
}

#[define((
  desc: "Compare",
  usage: [],
  notes: Some(r"Important examples
- comp $1 == $2
- comp $1 != $2
- comp $1 < $2 (only if $1 $2 = number)
- comp $1 <= $2 (only if $1 $2 = number)        
- comp $1 > $2 (only if $1 $2 = number)
- comp $1 >= $2 (only if $1 $2 = number)")
))]
fn comp(a: &BufValue, f: &str, b: &BufValue) -> BufValue {
  BufValue::Bool(match f {
    "==" => a.eq(b),
    "!=" => !a.eq(b),
    "<" => a.lt(&b),
    "<=" => a.lt(&b) || a.eq(b),
    ">" => a.gt(&b),
    ">=" => a.gt(&b) || a.eq(b),
    e => error(&format!("Invalid operator {} in :comp", e), file),
  })
}

generate!(Core, Array, Types);