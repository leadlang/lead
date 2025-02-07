use std::os::windows::ffi::OsStrExt;
use std::time::Duration;
use std::{ffi::OsString, str::FromStr};
use winreg::{
  enums::{RegType, HKEY_CURRENT_USER, KEY_READ, KEY_WRITE},
  RegKey, RegValue,
};

pub async fn postinstall(path: &str) {
  tokio::time::sleep(Duration::from_secs(3)).await;

  let hkcu = RegKey::predef(HKEY_CURRENT_USER);

  let env = hkcu
    .open_subkey_with_flags("Environment", KEY_READ | KEY_WRITE)
    .unwrap();

  {
    let path = OsString::from_str(&path).unwrap();
    let raw_path = RegValue {
      bytes: path
        .encode_wide()
        .flat_map(|v| vec![v as u8, (v >> 8) as u8])
        .collect(),
      vtype: RegType::REG_EXPAND_SZ,
    };
    env.set_raw_value("LEAD_HOME", &raw_path).unwrap();
  }

  if let Ok(x) = env.get_value::<String, &str>("PATH") {
    if !x.contains(path) {
      let val = format!("{};{}", &x, &path);
      let val = OsString::from_str(&val).unwrap();

      let val = RegValue {
        bytes: val
          .encode_wide()
          .flat_map(|v| vec![v as u8, (v >> 8) as u8])
          .collect(),
        vtype: RegType::REG_EXPAND_SZ,
      };
      env.set_raw_value("PATH", &val).unwrap();
    }
  } else {
    let val = OsString::from_str(path).unwrap();

    let val = RegValue {
      bytes: val
        .encode_wide()
        .flat_map(|v| vec![v as u8, (v >> 8) as u8])
        .collect(),
      vtype: RegType::REG_EXPAND_SZ,
    };
    env.set_raw_value("PATH", &val).unwrap();
  }
}
