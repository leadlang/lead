use std::sync::Arc;

use indicatif::{MultiProgress, ProgressBar};
use metadata::get_meta;
use chalk_rs::Chalk;

pub(super) mod utils;

pub mod add;
pub mod remove;
pub mod metadata;

pub enum PackageAction {
  Add,
  Remove
}

pub async fn handle(chalk: &mut Chalk, action: PackageAction, args: Vec<String>) {
  let mut metadata = get_meta().await;

  let mut bars = MultiProgress::new();

  match action {
    PackageAction::Add => {
      let args: Vec<Arc<String>> = args.into_iter().map(|x| Arc::new(x)).collect();
      println!(
        "{} 📦 Downloading packages...",
        chalk.bold().dim().string(&"[1/2]")
      );

      let mut handles = vec![];

      for pkg in &args {
        let pkg = pkg.clone();
        
        let bar = ProgressBar::no_length();
        let bar = bars.add(bar);
        handles.push(tokio::spawn(async move {
          add::install(pkg, bar).await;
        }));
      }

      for hwnd in handles {
        hwnd.await;
      }

      bars.suspend(|| {
        println!(
          "{} 🛠️  Linking dependencies...",
          chalk.bold().dim().string(&"[2/2]")
        );
      });
    },
    PackageAction::Remove => {
      remove::remove().await
    }
  }
}