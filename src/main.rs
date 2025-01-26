use std::{
  fs,
  path::PathBuf,
  process::Command,
  str::FromStr,
};

fn main() {
  let c_target = env!("TARGET");

  let target = if c_target == "x86_64-20.04-linux-gnu" {
    "x86_64-unknown-linux-gnu"
  } else {
    c_target
  };

  let cross = option_env!("USE_CROSS").map_or(false, |_| true);

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

  if c_target.contains("windows")
    || c_target.contains("apple")
    || c_target.contains("x86_64-unknown-linux-gnu")
    || c_target.contains("aarch64-unknown-linux-gnu")
  {
    dir.push(PathBuf::from_str("./lead_docs").unwrap());
  } else {
    dir.push(PathBuf::from_str("./lead_docs_cli").unwrap());
  }

  for path in dir {
    let mut cmd = Command::new(if cross { "cross" } else { "rustup" });
    let cmd = if !cross {
      cmd.args([
        "run",
        "1.86.0-2025-01-25",
        "cargo",
        "build",
        #[cfg(not(debug_assertions))]
        "--release",
      ])
    } else {
      cmd.args([
        "+1.86.0-2025-01-25",
        "build",
        #[cfg(not(debug_assertions))]
        "--release",
      ])
    };
    cmd.args(["--target", target]);

    if target.contains("musl") {
      cmd.env("RUSTFLAGS", "-C target-feature=-crt-static");
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

    println!("{}", fs_dir);

    for file in fs::read_dir(fs_dir).unwrap() {
      let file = file.unwrap();

      let name = file.file_name();
      let name = name.to_str().unwrap();
      let path = file.path();

      if name.starts_with("lead") && [4, 8, 9, 13].contains(&name.len()) {
        println!("Copying {} -> ./build/{}", path.display(), name);
        fs::copy(&path, format!("./build/{}", name)).unwrap();
      }

      if name.ends_with(".dll") || name.ends_with(".so") || name.ends_with(".dylib") {
        #[cfg(debug_assertions)]
        fs::create_dir_all(format!("./build/lib/{}", &name.split_once(".").unwrap().0)).unwrap();

        #[cfg(debug_assertions)]
        fs::copy(
          &path,
          format!("./build/lib/{}/{}", &name.split_once(".").unwrap().0, name),
        )
        .unwrap();

        #[cfg(not(debug_assertions))]
        {
          if name.ends_with("_lib.so") || name.ends_with("_lib.dll") || name.ends_with("_lib.dll") {
            fs::copy(&path, format!("./build/{}", name)).unwrap();
          } else {
            fs::copy(&path, format!("./build/lib/{}", name)).unwrap();
          }
        }
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
