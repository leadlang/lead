use std::{env::consts::{ARCH, OS, FAMILY}, io::Write};
use chalk_rs::Chalk;
use termcolor::{StandardStream, ColorChoice, ColorSpec, Color, WriteColor};

use sysinfo::{System, IS_SUPPORTED_SYSTEM};

fn get_sys_info() -> Vec<String> {
  let mut resp = vec![
    "System Specifications".into(),
    format!("{:<8}: {}", "OS", OS),
    format!("{:<8}: {}", "Arch", ARCH),
    format!("{:<8}: {}", "Family", FAMILY),
  ];

  if IS_SUPPORTED_SYSTEM {
    let mut sys = System::new_all();
    sys.refresh_all();

    let name = sys.cpus()[0].brand().trim();
    resp.push(format!("{:<8}: {}", "CPU", name));

    let free = sys.used_memory();
    let ram = sys.total_memory();
    resp.push(format!("{:<8}: {}/{} MB", "Memory", free/1000000, ram/1000000));
  }

  resp.push("".into());
  resp.push("Lead Language".into());
  resp.push(format!("{:<8}: {}", "Version", env!("CARGO_PKG_VERSION")));
  resp.push(format!("{:<8}: {}", "Target", env!("TARGET")));
  
  resp
}

pub fn render_lead_logo() {
  let mut stdout = StandardStream::stdout(ColorChoice::Always);

  let logo = include!("./logo.bin");

  // Full Box
  let fill = "██";
  
  let mut sysinfo = get_sys_info();
  for stream in logo {
    let mut dat = String::new();

    if !sysinfo.is_empty() {
      dat = sysinfo.remove(0);
    }

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White))).unwrap();
    write!(&mut stdout, "{:<60}", dat).unwrap();

    for [r, g, b] in stream {
      stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(r, g, b)))).unwrap();

      write!(&mut stdout, "{}", fill).unwrap();
    }

    write!(&mut stdout, "\n").unwrap();
  }

  stdout.set_color(ColorSpec::new().set_fg(None)).unwrap();
  writeln!(&mut stdout, "");
}