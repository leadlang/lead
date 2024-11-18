use std::{
  fs::{self, create_dir_all},
  path::PathBuf,
  process::Command,
  str::FromStr,
};

fn main() {
  let target = env!("TARGET");

  #[cfg(not(debug_assertions))]
  fs::remove_dir_all("./build").unwrap_or(());
  fs::remove_file("./build.zip").unwrap_or(());
  #[cfg(not(debug_assertions))]
  fs::create_dir_all("./build/lib").unwrap();

  let path = "./packages";

  let mut dir = fs::read_dir(path)
    .unwrap()
    .into_iter()
    .map(|x| x.unwrap().path())
    .collect::<Vec<_>>();

  dir.push(PathBuf::from_str("./lead").unwrap());
  dir.push(PathBuf::from_str("./lead_docs").unwrap());

  for path in dir {
    let mut cmd = Command::new("rustup");
    let mut cmd = cmd.args([
      "run",
      "nightly",
      "cargo",
      {
        if path.to_string_lossy().contains("lead") {
          "build"
        } else {
          "run"
        }
      },
      #[cfg(not(debug_assertions))]
      "--release",
    ]);

    if path.to_string_lossy().contains("lead") {
      cmd = cmd.args(["--target", target]);
    }

    let cmd = cmd
      .current_dir(&path)
      .spawn()
      .unwrap()
      .wait()
      .unwrap()
      .success();

    if !cmd {
      panic!("Failed to build");
    }

    #[cfg(not(debug_assertions))]
    let typ = "release";

    #[cfg(debug_assertions)]
    let typ = "debug";

    let fs_dir = format!("{}/target/{}/{}", path.to_string_lossy(), target, &typ);

    for file in fs::read_dir(fs_dir).unwrap() {
      let file = file.unwrap();

      let name = file.file_name();
      let name = name.to_str().unwrap();
      let path = file.path();

      if name.starts_with("lead") && [4, 8, 9, 13].contains(&name.len()) {
        fs::copy(&path, format!("./build/{}", name)).unwrap();
      }

      if name.ends_with(".dll") || name.ends_with(".so") {
        #[cfg(debug_assertions)]
        fs::create_dir_all(format!("./build/lib/{}", &name.split_once(".").unwrap().0)).unwrap();
        #[cfg(debug_assertions)]
        fs::copy(
          &path,
          format!("./build/lib/{}/{}", &name.split_once(".").unwrap().0, name),
        )
        .unwrap();

        #[cfg(not(debug_assertions))]
        fs::copy(&path, format!("./build/lib/{}", name)).unwrap();
      }
    }
  }

  let pkg_docs = [
    ("./packages/core/docs/", "./build/docs/core"),
    ("./packages/std/docs/", "./build/docs/std"),
  ];

  use fs_extra::dir::{copy, CopyOptions};

  let mut options = CopyOptions::new();
  options.overwrite = true;
  options.content_only = true;
  options.copy_inside = true;

  for (pkg, out) in pkg_docs {
    create_dir_all(&out).unwrap();
    copy(&pkg, &out, &options).unwrap();
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
