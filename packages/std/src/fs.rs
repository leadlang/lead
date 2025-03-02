use std::{collections::HashMap, fs::File};

use interpreter::{function, types::MethodRes, module, parse, pkg_name, runtime::RuntimeValue, types::{AnyWrapper, BufValue, Heap, HeapWrapper, Options}};

module! {
  Fs,
  pkg_name! { "📦 Lead Programming Language / File System" }
  fn methods(&self) -> MethodRes {
    &[function! {
      "fs::open",
      |args, _heap, file, opt| {
        parse!(file + _heap + args: str path);

        let file = File::open(path);

        let mut resp = RuntimeValue::new("fs/file", {
          let mut map: HashMap<&'static _, (&'static _, for<'c, 'd, 'e> fn(*const [*const str], &'c mut Heap, HeapWrapper, &'d String, &'e mut Options))> = HashMap::new();

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
    }]
  }
}
