use inquire::Text;
use interpreter::{
  types::{BufValue, MethodRes},
  Chalk, Package,
};

pub struct FS;
impl Package for FS {
  fn name(&self) -> &'static [u8] {
    "📦 Lead Programming Language / IO".as_bytes()
  }

  fn methods(&self) -> MethodRes {
    &[
      ("print", |args, heap, _| {
        let args = &args[1..];
        let args = args
          .iter()
          .map(|x| {
            let mut chalk = Chalk::new();
            match heap.get(x) {
              Some(x) => match &x {
                &BufValue::Bool(x) => chalk.red().string(&format!("{x}")),
                &BufValue::Str(st) => chalk.green().string(&format!("\"{}\"", &st)),
                &BufValue::Int(x) => chalk.blue().string(&format!("{x}")),
                &BufValue::Float(x) => chalk.blue().string(&format!("{x}")),
                &BufValue::Array(x) => chalk.yellow().string(&format!("{:?}", &x)),
                &BufValue::Object(x) => chalk.yellow().string(&format!("{:?}", &x)),
                x => format!("{:?}", &x),
              },
              _ => format!("NONE"),
            }
          })
          .collect::<Vec<_>>();

        println!("{}", &args.join(", "));
      }),
      ("ask", |args, heap, _| {
        let argv = &args[1..args.len() - 2].join(" ");
        let val: &String = &args[args.len() - 1];
        let val: String = val.into();

        let bufval = Text::new(&argv).prompt().map_or_else(
          |_| BufValue::Faillable(Err("".into())),
          |val| BufValue::Faillable(Ok(Box::new(BufValue::Str(val)))),
        );
        heap.set(val, bufval);
      }),
    ]
  }
}
