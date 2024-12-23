use std::{sync::Arc, time::{Duration, SystemTime, UNIX_EPOCH}};

use chalk_rs::Chalk;
use indicatif::{HumanDuration, MultiProgress, ProgressBar};
use metadata::{get_meta, write_meta, Dependency};
use tokio::time::sleep;
use utils::spinner_style;

pub(super) mod utils;

pub mod add;
pub mod metadata;
pub mod remove;

pub enum PackageAction {
  Add,
  Remove,
}

pub async fn handle(chalk: &mut Chalk, action: PackageAction, args: Vec<String>) {
  let mut metadata = get_meta().await;

  let mut bars = MultiProgress::new();

  let start = SystemTime::now();

  match action {
    PackageAction::Add => {
      let args: Vec<Arc<String>> = args.into_iter().map(|x| Arc::new(x)).collect();
      println!(
        "{} 📦 Downloading packages...",
        chalk.bold().dim().string(&"[1/3]")
      );

      let mut handles = vec![];

      for pkg in &args {
        let pkg = pkg.clone();

        let bar = ProgressBar::no_length();
        let bar = bars.add(bar);
        handles.push(tokio::spawn(async move {
          add::install(pkg, bar).await
        }));
      }

      for hwnd in handles {
        let (meta, name, version) = hwnd.await.expect("Error while installing");

        let overrid = format!("      ⚠️  Overriding existing dependency {name}");

        if let Some(_) = metadata.dependencies.insert(name, Dependency {
          os: meta.platforms,
          version
        }) {
          bars.suspend(|| {
            println!("{overrid}");
          });
        }
      }

      // It'll be already done by now...
      println!(
        "{} 🗃️  Updating metadata...",
        chalk.bold().dim().string(&"[2/3]")
      );
      
      write_meta(&metadata).await;

      sleep(Duration::from_secs(1)).await;
    }
    PackageAction::Remove => remove::remove().await,
  }

  bars.suspend(|| {
    println!(
      "{} 🛠️  Linking dependencies...",
      chalk.bold().dim().string(&"[2/3]")
    );
  });

  let bar = ProgressBar::new_spinner().with_style(spinner_style());
  let bar = bar.with_message("Please wait...");

  // 2seconds cuz we're too fast
  for _ in 0..100 {
    bar.tick();
    sleep(Duration::from_millis(20)).await;
  }

  let dur = SystemTime::now().duration_since(start).expect("Time is running backwards");
  let dur = HumanDuration(dur);
  
  bar.finish_and_clear();
  
  println!("      ✅ Installation done in {dur}");
}
