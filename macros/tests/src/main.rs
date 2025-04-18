use interpreter::{
  module,
  pkg_name,
  types::BufValue
};
use lead_lang_macros::{define, define_prototypes};

fn main() {
  
}

module! {
  MyModule,
  pkg_name! { "📦 MyModule" }
  define_prototypes! {
    int: {
      print=print
    };
  }
}

#[define((
  desc: "Print two variables",
  usage: [
    (
      desc: "Print two variables",
      code: "$data: print $a $b"
    )
  ],
  root: Some("int")
))]
fn print() -> BufValue {
  BufValue::Bool(true)
}
