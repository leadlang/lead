use interpreter::types::BufValue;
use lead_lang_macros::define;

fn main() {
  println!("Hello, world!");
}

#[define((
  desc: "Print two variables",
  usage: [
    (
      desc: "Print two variables",
      code: "$data: print $a $b"
    )
  ],
  notes: Some("This is a simple print function for macro test")
))]
fn print(_a: &str, _b: &mut BufValue) -> BufValue {
  BufValue::Bool(true)
}
