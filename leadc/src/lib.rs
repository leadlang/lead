use tokio::runtime::Builder;
use lealang_chalk_rs::Chalk;

mod help;

fn prefix(chalk: &mut Chalk) {
    chalk.yellow().bold().println(&format!(
      "LeadC v{}",
      env!("CARGO_PKG_VERSION")
    ));
    chalk.default_color().bold().println(&"©️  Lead Programming Language \n");
  }
  

pub fn app(args: Vec<String>) {
  Builder::new_current_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(async move {
      let mut chalk = Chalk::new();

      prefix(&mut chalk);

      let arg0 = if args.len() > 1 {
        &args[1] as &str
      } else {
        "help"
      };

      match arg0 {
        "help" => help::help(),
        e => {
            chalk.red().bold().print(&"Unknown command: ");
            println!("{e}");
            
            help::help();
        }
      }
    });
}