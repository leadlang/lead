use interpreter::{document, function, methods, module, pkg_name, runtime::RuntimeValue, types::{BufValue, *}, Chalk};
use std::collections::HashMap;

module!(
  IO,
  pkg_name! {"📦 Lead Programming Language / IO"}
  methods! {
    function!("print", document!(), |args, heap, _, _| {
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
    function! {
      "os::name",
      document!(""),
      |_, _, _, opt| {
        opt.set_return_val(if cfg!(windows) {
          BufValue::Str("Win32".into())
        } else if cfg!(target_os = "macos") {
          BufValue::Str("MacOS".into())
        } else if cfg!(target_os = "linux") {
          BufValue::Str("Linux".into())
        } else {
          BufValue::Str("Unknown".into())
        });
      }
    }
  }
);

module!(
  AHQ,
  pkg_name! {"📦 Lead Programming Language / AHQ"}
  methods! {
    function! {
      "ahq::mk",
      document!(""),
      |_, _, _, opt| {
        opt.set_r_runtime(RuntimeValue::new("core/str_string", {
          let mut map: HashMap<&'static _, (&'static _, for<'a, 'b, 'd, 'e> fn(&'a Vec<String>, &'b mut Heap, HeapWrapper, &'d String, &'e mut Options))> = HashMap::new();

          map.insert("test", ("", |_, _, _, _, _| {
            println!("This is a test");
          }));

          map
        }));
      }
    }
  }
);