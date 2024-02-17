use interpreter::{function, methods, module, parse, pkg_name, types::BufValue};

module!(
  IO,
  pkg_name! {"📦 Lead Programming Language / IO"}
  methods! {
    function! {
      "os::name",
      |args, heap, _| {
        parse!(heap + args: > val);

        if cfg!(windows) {
          heap.set(val.clone(), BufValue::Str("Win32".into()));
        } else if cfg!(target_os = "macos") {
          heap.set(val.clone(), BufValue::Str("MacOS".into()));
        } else if cfg!(target_os = "linux") {
          heap.set(val.clone(), BufValue::Str("Linux".into()));
        } else {
          heap.set(val.clone(), BufValue::Str("Unknown".into()));
        }
      }
    }
  }
);
