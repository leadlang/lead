use interpreter::{module, pkg_name, types::BufValue};
use lead_lang_macros::{define, methods};

module! {
  Types,
  pkg_name! { "📦 Core / Types Conversion" }
  methods! {
    str::to_int=to_int
  }
}

#[define((
  desc: "Parse string into int",
  usage: [
    (
      desc: "",
      code: "$val: str::to_int ->$val\n$val: unwrap ->$val"
    )
  ],
  notes: Some("The result is a Faillable as the parsing might fail as well")
))]
fn to_int(val: BufValue) -> BufValue {
  BufValue::Faillable(
    {
      if let BufValue::Str(s) = val {
        s.parse::<i64>().map_or_else(|e| Err(e.to_string()), |x| Ok(Box::new(BufValue::Int(x))))
      } else {
        Err(format!("Expected string, found {}", val.type_of()))
      }
    }
  )
}