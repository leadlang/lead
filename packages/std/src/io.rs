use interpreter::{types::MethodRes, function, module, pkg_name, runtime::RuntimeValue, types::{BufValue, *}, Chalk};
use std::{collections::HashMap, env::consts::{OS, ARCH}};

module!(
  IO,
  pkg_name! {"📦 Lead Programming Language / IO"}
  fn methods(&self) -> MethodRes {
    &[function!("print", |args, heap, _, _| {
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

              &BufValue::AsyncTask(x) => chalk.green().string(&format!("<async finished={}>", x.is_finished())),

              &BufValue::Pointer(x) => chalk.blue().string(&format!("<* const null={}>", x.is_null())),
              &BufValue::PointerMut(x) => chalk.blue().string(&format!("<* mut null={}>", x.is_null())),

              &BufValue::Listener(x) => chalk.blue().string(&format!("<listener closed={} empty={}>", x.is_closed(), x.is_empty())),
              &BufValue::Runtime(_) | &BufValue::RuntimeRaw(_, _) => chalk.blue().string(&"<runtime *>"),
              x => chalk.cyan().string(&format!("{:?}", &x)),
            },
            _ => chalk.red().string(&"null"),
          }
        })
        .collect::<Vec<_>>();

      println!("{}", &args.join(", "));
    }),
    function! {
      "os::name",
      |_, _, _, opt| {
        opt.set_return_val(BufValue::Str(format!("{OS}_{ARCH}")));
      }
    }]
  }
);

module!(
  AHQ,
  pkg_name! {"📦 Lead Programming Language / AHQ"}
  fn methods(&self) -> MethodRes {
    &[function! {
      "ahq::mk",
      |_, _, _, opt| {
        opt.set_r_runtime(RuntimeValue::new("core/str_string", {
          let mut map: HashMap<&'static _, (&'static _, for<'b, 'd, 'e> fn(*const [*const str], &'b mut Heap, HeapWrapper, &'d String, &'e mut Options))> = HashMap::new();

          map.insert("test", ("", |_, _, _, _, _| {
            println!("This is a test");
          }));

          map
        }));
      }
    }]
  }
);