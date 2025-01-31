use lealang_chalk_rs::Chalk;

macro_rules! generate_help {
  ($app_name:expr, $($option:expr => $description:expr),*) => {
      pub fn help() {
          let mut chalk = Chalk::new();
          chalk.underline();

          println!("{} {} [COMMAND]", chalk.string(&"Usage:"), $app_name);
          println!();
          chalk.println(&"Commands");
          $(
              println!("  {:<20} {}", $option, $description);
          )*
          println!();
          chalk.println(&"Example");
          println!("  {} list", $app_name);
          println!("  {} use", $app_name);
          println!("  {} add gh:lead/xyz@1.0 gh:lead/std@latest", $app_name);
          println!("  {} remove gh:lead/xyz@1.0 gh:lead/std@latest", $app_name);
      }
  };
}

generate_help!(
  env!("CARGO_PKG_NAME"),
  {
    Chalk::new().underline().bold().string(&"GENERAL")
  } => "",
  " help" => "Prints this help message",
  " help-ci" => "Get environment variables that can be used by CI to skip the prompts",
  " install" => "Install a version of lead lang",
  " list" => "Lists the installed versions",
  " use, default" => "Set the default lead version",
  " uninstall" => "Uninstall a version of lead lang",
  "" => "",
  {
    Chalk::new().underline().bold().string(&"PACKAGE MANAGEMENT")
  } => "",
  " add [..packages]" => {
    let mut chalk = Chalk::new();
    chalk.bold();
    format!("Add lead packages\n  {:<20} Use {} to specify the version, use space to specify multiple packages", "", chalk.string(&"@"))
  },
  " remove [..packages]" => {
    let mut chalk = Chalk::new();
    chalk.bold();
    format!("Remove lead packages\n  {:<20} Use {} to specify the version, use space to specify multiple packages", "", chalk.string(&"@"))
  },
  " reinstall, rei, i, install" => "Reinstall dependencies by downloading & relinking",
  " link" => "Link dependencies... aka relink using data from .pkgcache",
  " packages, pkgs" => "List the packages that are installed"
);
