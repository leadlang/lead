use std::env;

fn main() {
  let target = option_env!("CFG_TARGET").map_or_else(|| env::var("TARGET").unwrap(), |x| x.to_string());
  
  println!("cargo:rustc-env=TARGET={target}");

  #[cfg(windows)]
  {
    let mut res = tauri_winres::WindowsResource::new();
    res.set_icon("icon.ico")
      .compile()
      .unwrap();
  }
}