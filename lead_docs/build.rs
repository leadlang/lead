fn main() {
  #[cfg(windows)]
  {
    let mut res = tauri_winres::WindowsResource::new();
    res.set_icon("icon.ico")
      .compile()
      .unwrap();
  }
}