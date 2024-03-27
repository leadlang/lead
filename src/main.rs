use std::{fs, path::PathBuf, process::Command, str::FromStr};

fn main() {
  #[cfg(not(debug_assertions))]
  fs::remove_dir_all("./build").unwrap_or(());
  fs::remove_file("./build.zip").unwrap_or(());
  #[cfg(not(debug_assertions))]
  fs::create_dir_all("./build/lib").unwrap();

  #[cfg(windows)]
  let path = ".\\packages";
  #[cfg(not(windows))]
  let path = "./packages";

  let mut dir = fs::read_dir(path)
    .unwrap()
    .into_iter()
    .map(|x| x.unwrap().path())
    .collect::<Vec<_>>();

  #[cfg(windows)]
  dir.push(PathBuf::from_str(".\\lead").unwrap());

  #[cfg(not(windows))]
  dir.push(PathBuf::from_str("./lead").unwrap());

  for path in dir {
    let cargo_path = path.join("Cargo.toml");
    let cargo_path = cargo_path.to_str().unwrap();

    if !Command::new("rustup")
      .args([
        "run",
        "nightly",
        "cargo",
        "build",
        #[cfg(not(debug_assertions))]
        "--release",
        "--manifest-path",
        cargo_path,
      ])
      .spawn()
      .unwrap()
      .wait()
      .unwrap()
      .success()
    {
      panic!("Failed to build");
    }

    #[cfg(not(debug_assertions))]
    let typ = "release";

    #[cfg(debug_assertions)]
    let typ = "debug";

    #[cfg(windows)]
    let fs_dir = format!("{}\\target\\{}", path.to_string_lossy(), &typ);

    #[cfg(not(windows))]
    let fs_dir = format!("{}/target/{}", path.to_string_lossy(), &typ);

    for file in fs::read_dir(fs_dir).unwrap() {
      let file = file.unwrap();

      let name = file.file_name();
      let name = name.to_str().unwrap();
      let path = file.path();

      if name.starts_with("lead") && [4, 8].contains(&name.len()) {
        fs::copy(&path, format!("./build/{}", name)).unwrap();
      }

      if name.ends_with(".dll") || name.ends_with(".so") {
        #[cfg(debug_assertions)]
        fs::create_dir_all(format!("./build/lib/{}", &name.split_once(".").unwrap().0)).unwrap();
        #[cfg(debug_assertions)]
        fs::copy(&path, format!("./build/lib/{}/{}", &name.split_once(".").unwrap().0, name)).unwrap();

        #[cfg(not(debug_assertions))]
        fs::copy(&path, format!("./build/lib/{}", name)).unwrap();
      }
    }
  }

  #[cfg(windows)]
  Command::new("powershell")
    .args(["compress-archive", "./build/*, ./templates", "./build.zip"])
    .spawn()
    .unwrap()
    .wait()
    .unwrap();

  #[cfg(not(windows))]
  Command::new("cp")
    .args(["-r", "templates", "./build/templates/"])
    .spawn()
    .unwrap()
    .wait()
    .unwrap();

  #[cfg(not(windows))]
  Command::new("zip")
    .args(["-r", "../build.zip", "./"])
    .current_dir("./build")
    .spawn()
    .unwrap()
    .wait()
    .unwrap();
}
