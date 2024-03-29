use interpreter::{document, function, methods, module, pkg_name, types::BufValue};

module!(
  IO,
  pkg_name! {"📦 Lead Programming Language / IO"}
  methods! {
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
