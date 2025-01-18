use std::{collections::HashMap, fs::File};

use interpreter::{function, methods, module, parse, pkg_name, runtime::RuntimeValue, types::{AnyWrapper, BufValue, Heap, HeapWrapper, Options}};

module! {
  Fs,
  pkg_name! { "📦 Lead Programming Language / File System" }
  methods! {
    function! {
      "fs::open",
      |args, _heap, file, opt| {
        parse!(file + _heap + args: str path);

        let file = File::open(path);

        let mut resp = RuntimeValue::new("fs/file", {
          let mut map: HashMap<&'static _, (&'static _, for<'a, 'c, 'd, 'e> fn(&'a Vec<String>, &'c mut Heap, HeapWrapper, &'d String, &'e mut Options))> = HashMap::new();

          map.insert("print", ("", |_args, _inner, _outer, _, _| {
            println!();
          }));

          map
        });
        resp._inner.set("".into(), BufValue::Runtime(AnyWrapper(Box::new(file))));
        
        opt.set_r_runtime(resp);
      }
    },
    function! {
      "fs::create",
      |_, _, _, opt| {
        opt.set_return_val(BufValue::Str("Hello World".into()));
      }
    },
  }
}

// pub struct FS;
// impl Package for FS {
//   fn name(&self) -> &'static [u8] {
//     "📦 Lead Programming Language / IO".as_bytes()
//   }

//   fn methods(&self) -> MethodRes {
//     &[
//       ("print", |args, heap, _| {
//       }),
//       ("ask", |args, heap, _| {
//         let argv = &args[1..args.len() - 2].join(" ");
//         let val: &String = &args[args.len() - 1];
//         let val: String = val.into();

//         let bufval = Text::new(&argv).prompt().map_or_else(
//           |_| BufValue::Faillable(Err("".into())),
//           |val| BufValue::Faillable(Ok(Box::new(BufValue::Str(val)))),
//         );
//         heap.set(val, bufval);
//       }),
//     ]
//   }
// }
