use std::{ptr::addr_of, sync::Arc, time::{Duration, SystemTime, UNIX_EPOCH}};

use chalk_rs::Chalk;
use indicatif::{HumanDuration, MultiProgress, ProgressBar};
use linker::ScriptClass;
use metadata::{get_meta, write_meta, Metadata};
use tokio::{fs, time::sleep};
use utils::spinner_style;

pub(super) mod utils;

pub mod add;
pub mod metadata;
pub mod remove;

pub mod linker;

pub enum PackageAction {
  Add,
  Remove,
}

pub struct MetaPtr(*const Metadata);

unsafe impl Sync for MetaPtr {}
unsafe impl Send for MetaPtr {}

pub async fn list(chalk: &mut Chalk) {
  let metadata = get_meta().await;

  if metadata.dependencies.is_empty() {
    println!("No package installed! Use {} to add packages", chalk.green().string(&"leadman add"));
    return;
  }

  chalk.underline();

  println!("{name}{:<20}{}", "", chalk.string(&"Version"), name = chalk.string(&"Package"));
  for (k, v) in metadata.dependencies {
    println!("{k}{:<12}{v}", "");
  }
}

pub async fn link(chalk: &mut Chalk) {
  let mut metadata = get_meta().await;

  let mut bars = MultiProgress::new();

  let start = SystemTime::now();

  let bar = ProgressBar::new_spinner().with_style(spinner_style());
  let bar = bar.with_message("Please wait...");

  let bar = bars.add(bar);

  bars.suspend(|| {
    println!(
      "{} 📜 Running preinstall scripts...",
      chalk.bold().dim().string(&"[1/3]")
    );
  });

  let metadata_arc = Arc::new(MetaPtr(addr_of!(metadata)));

  let task = tokio::task::spawn(linker::run_script(metadata_arc.clone(), ScriptClass::Pre, bars.clone()));

  loop {
    bar.tick();

    if task.is_finished() {
      task.await;
      break
    }

    sleep(Duration::from_millis(20)).await;
  }

  bars.suspend(|| {
    println!(
      "{} 🛠️  Linking dependencies...",
      chalk.bold().dim().string(&"[2/3]")
    );
  });

  let task = tokio::task::spawn(linker::link(metadata_arc.clone(), bars.clone()));

  loop {
    bar.tick();
    
    if task.is_finished() {
      task.await;
      break
    }

    sleep(Duration::from_millis(20)).await;
  }

  bars.suspend(|| {
    println!(
      "{} 📜 Running postinstall scripts...",
      chalk.bold().dim().string(&"[3/3]")
    );
  });

  let task = tokio::task::spawn(linker::run_script(metadata_arc.clone(), ScriptClass::Post, bars.clone()));

  loop {
    bar.tick();

    if task.is_finished() {
      task.await;
      break
    }

    sleep(Duration::from_millis(20)).await;
  }

  bar.finish_and_clear();
  
  let dur = SystemTime::now().duration_since(start).expect("Time is running backwards");
  let dur = HumanDuration(dur);
  
  println!("      ✅ Linking done in {dur}");
}

pub async fn install(chalk: &mut Chalk) {
  fs::remove_dir_all("./.pkgcache").await;
  fs::remove_dir_all("./lib").await;

  let mut metadata = get_meta().await;

  let mut apps = vec![];

  metadata.dependencies.into_iter().for_each(|(k, v)| {
    apps.push(format!("{k}@{v}"));
  });

  handle(chalk, PackageAction::Add, apps).await;
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
        chalk.bold().dim().string(&"[1/5]")
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
        let (name, version) = hwnd.await.expect("Error while installing");

        let overrid = format!("      ⚠️  Overriding existing dependency {name}");

        if let Some(_) = metadata.dependencies.insert(name, version) {
          bars.suspend(|| {
            println!("{overrid}");
          });
        }
      }

      // It'll be already done by now...
      println!(
        "{} 🗃️  Updating metadata...",
        chalk.bold().dim().string(&"[2/5]")
      );
      
      write_meta(&metadata).await;

      sleep(Duration::from_secs(1)).await;
    }
    PackageAction::Remove => {
      let metadata_arc = Arc::new(MetaPtr(addr_of!(metadata)));
      let args: Vec<Arc<String>> = args.into_iter().map(|x| Arc::new(x)).collect();
      println!(
        "{} 📦 Resolving packages...",
        chalk.bold().dim().string(&"[1/5]")
      );

      let mut handles = vec![];

      for pkg in &args {
        let pkg = pkg.clone();

        let bar = ProgressBar::no_length();
        let bar = bars.add(bar);

        let meta = Arc::new(MetaPtr(addr_of!(metadata)));
        handles.push(tokio::spawn(async move {
          remove::remove(meta, pkg, bar).await
        }));
      }

      let mut store = vec![];
      for hwnd in handles {
        let name = hwnd.await.expect("Error while installing");

        store.push(name);
      }

      for hwnd in store {
        let name: &String = &hwnd;
        let overrid = format!("      ❌ No dependency named {name} was found");

        if let None = metadata.dependencies.remove(name) {
          bars.suspend(|| {
            println!("{overrid}");
          });
        }
      }

      // It'll be already done by now...
      println!(
        "{} 🗃️  Updating metadata...",
        chalk.bold().dim().string(&"[2/5]")
      );
      
      write_meta(&metadata).await;

      sleep(Duration::from_secs(1)).await;
    },
  }

  let bar = ProgressBar::new_spinner().with_style(spinner_style());
  let bar = bar.with_message("Please wait...");

  let bar = bars.add(bar);

  bars.suspend(|| {
    println!(
      "{} 📜 Running preinstall scripts...",
      chalk.bold().dim().string(&"[3/5]")
    );
  });

  let metadata_arc = Arc::new(MetaPtr(addr_of!(metadata)));

  let task = tokio::task::spawn(linker::run_script(metadata_arc.clone(), ScriptClass::Pre, bars.clone()));

  loop {
    bar.tick();

    if task.is_finished() {
      task.await;
      break
    }

    sleep(Duration::from_millis(20)).await;
  }

  bars.suspend(|| {
    println!(
      "{} 🛠️  Linking dependencies...",
      chalk.bold().dim().string(&"[4/5]")
    );
  });

  let task = tokio::task::spawn(linker::link(metadata_arc.clone(), bars.clone()));

  loop {
    bar.tick();
    
    if task.is_finished() {
      task.await;
      break
    }

    sleep(Duration::from_millis(20)).await;
  }

  bars.suspend(|| {
    println!(
      "{} 📜 Running postinstall scripts...",
      chalk.bold().dim().string(&"[5/5]")
    );
  });

  let task = tokio::task::spawn(linker::run_script(metadata_arc.clone(), ScriptClass::Post, bars.clone()));

  loop {
    bar.tick();

    if task.is_finished() {
      task.await;
      break
    }

    sleep(Duration::from_millis(20)).await;
  }

  bar.finish_and_clear();
  
  let dur = SystemTime::now().duration_since(start).expect("Time is running backwards");
  let dur = HumanDuration(dur);
  
  println!("      ✅ Installation done in {dur}");
}
