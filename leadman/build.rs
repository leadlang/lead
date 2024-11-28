use std::env;

fn main() {
  let target = env::var("TARGET").unwrap();

  println!("cargo::rustc-env=TARGET={target}");

  #[cfg(windows)]
  {
    let mut res = tauri_winres::WindowsResource::new();
    res.set_icon("icon.ico")
      .compile()
      .unwrap();
  }
}
