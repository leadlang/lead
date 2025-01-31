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
              println!("  {:<25} {}", $option, $description);
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
  "lead",
  "help" => "Prints this help message",
  "docs" => "Shows the docs [runs lead_docs]",
  "run [..args]" => "Run lead script based on metadata.json",
  {
    let mut chalk = Chalk::new();
    let val = chalk.underline().string(&"args");

    format!(" {val}")
  } => "",
  "  --prod" => "Same as --log --deny-full-access --no-sysinfo",
  "  --monochrome-logo" => "Enables monochrome variant of the lead logo",
  "  --no-sysinfo" => "Do not show sysinfo on load",
  "  --log" => "Log Full Heap Access events for packages mentioned in metadata",
  "  --allow-full-access" => "Silently allow Full Heap Access requests (NOT RECOMMENDED)",
  "  --warn-full-access" => "Warn on Full Heap Access requests for packages not mentioned in metadata",
  "  --deny-full-access" => "Deny Full Heap Access request for packages not mentioned in metadata"
);
