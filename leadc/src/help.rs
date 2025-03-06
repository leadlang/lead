use lealang_chalk_rs::Chalk;

macro_rules! generate_help {
  ($app_name:expr, $($option:expr => $description:expr),*) => {
      pub fn help() {
          let mut chalk = Chalk::new();
          chalk.underline();

          chalk.println(&"Lead Language Compliance Tool");

          println!("{} {} [COMMAND]", chalk.string(&"Usage:"), $app_name);
          println!();
          chalk.println(&"Commands");
          $(
              println!("  {:<20} {}", $option, $description);
          )*
          println!();
          chalk.println(&"Example");
          println!("  {} verify", $app_name);
      }
  };
}

// {
//   Chalk::new().underline().bold().string(&"GENERAL")
// } => "",

generate_help!(
  env!("CARGO_PKG_NAME"),
  "help" => "Prints this help message",
  "verify" => "Verify a lead file"
);
