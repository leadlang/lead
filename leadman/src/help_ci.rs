macro_rules! generate_help {
  ($app_name:expr, $($option:expr => $description:expr),*) => {
      pub fn help() {
          println!("Pass these variables to {} to skip prompts\nThese are case sensitive", $app_name);
          println!();
          println!("{:<22} Variables", "Command");
          $(
            println!("  {:<20} {}", $option, $description[0]);
            for desc in 1..$description.len() {
              println!("  {:<20} {}", "", $description[desc]);
            }
          )*
          println!();
      }
  };
}

generate_help!(
  env!("CARGO_PKG_NAME"),
  "[installing]" => ["LEAD_CHANNEL = \"Stable\" or \"Nightly\""],
  "install" => ["LEAD_VERSION"],
  "use, default" => ["LEAD_OVERRIDE = \"stable\" or \"nightly\" or \"current\"", "LEAD_VERSION"],
  "uninstall" => ["LEAD_VERSION"]
);
