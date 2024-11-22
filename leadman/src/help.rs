macro_rules! generate_help {
  ($app_name:expr, $($option:expr => $description:expr),*) => {
      pub fn help() {
          println!("Usage: {} [COMMAND]", $app_name);
          println!();
          println!("Command:");
          $(
              println!("  {:<20} {}", $option, $description);
          )*
          println!();
          println!("Example:");
          println!("  {} list", $app_name);
          println!("  {} use", $app_name);
      }
  };
}

generate_help!(
  env!("CARGO_PKG_NAME"),
  "clear" => "Clear downloaded cache",
  "help" => "Prints this help message",
  "install" => "Install a version of lead lang",
  "list" => "Lists the installed versions",
  "use, default" => "Set the default lead version",
  "uninstall" => "Uninstall a version of lead lang"
);
