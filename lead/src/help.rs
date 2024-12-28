use chalk_rs::Chalk;

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
          println!("  {}", $app_name);
          println!("  {} run", $app_name);
          println!("  {} run --prod", $app_name);
          println!("  {} docs", $app_name);
          println!("  {} help", $app_name);
      }
  };
}

generate_help!(
  env!("CARGO_BIN_NAME"),
  "help" => "Prints this help message",
  "docs" => "Shows the docs [runs lead_docs]",
  "run [--prod]" => "Run lead script based on metadata.json, `--prod` hides the lead logo and other debug information"
);
