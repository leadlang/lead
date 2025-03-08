use interpreter::{module, pkg_name, types::{BufValue, HeapWrapper, Options}, Chalk};

use lead_lang_macros::{gendoc, methods};

module!(
  IO,
  pkg_name! {"📦 Lead Programming Language / IO"}
  methods! {
    print=print
  }
);

#[gendoc((
  desc: "Prints values, also colorizing them based on their types",
  params: [
    r"\$(\w+) ?"
  ],
  returns: Some("")
))]
fn print(args: *const [*const str], heap: HeapWrapper, _: &std::string::String, _: &mut Options) {
  let args = &(unsafe { &*args })[1..];
  let args = args
    .iter()
    .map(|x| {
      let x = unsafe { &**x };
      let mut chalk = Chalk::new();

      match heap.get(x) {
        Some(x) => match &x {
          &BufValue::Bool(x) => chalk.yellow().string(&format!("{x}")),
          &BufValue::Str(st) => chalk.green().string(&format!("\"{}\"", &st)),

          &BufValue::Int(x) => chalk.yellow().string(&format!("{x}")),
          &BufValue::U_Int(x) => chalk.yellow().string(&format!("{x}")),
          &BufValue::Float(x) => chalk.yellow().string(&format!("{x}")),

          &BufValue::Array(x) => format!("{:?}", &x),
          &BufValue::Object(x) => format!("{:?}", &x),

          &BufValue::AsyncTask(x) => chalk
            .green()
            .string(&format!("<async finished={}>", x.is_finished())),

          &BufValue::Pointer(x) => chalk
            .blue()
            .string(&format!("<* const null={}>", x.is_null())),
          &BufValue::PointerMut(x) => chalk
            .blue()
            .string(&format!("<* mut null={}>", x.is_null())),

          &BufValue::Listener(x) => chalk.blue().string(&format!(
            "<listener closed={} empty={}>",
            x.is_closed(),
            x.is_empty()
          )),
          &BufValue::Runtime(_) | &BufValue::RuntimeRaw(_, _) => {
            chalk.blue().string(&"<runtime *>")
          }
          x => chalk.cyan().string(&format!("{:?}", &x)),
        },
        _ => chalk.red().string(&"null"),
      }
    })
    .collect::<Vec<_>>();

  println!("{}", &args.join(", "));
}
