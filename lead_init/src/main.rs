use std::{fs::remove_dir_all, time::Duration};

use inquire::Text;
use dirs::home_dir;

use indicatif::ProgressBar;
mod utils;

#[tokio::main]
async fn main() {
    let mut home = home_dir().unwrap();
    home.push("leadLang");

    let dir = Text::new(&"Where shall we install lead?")
        .with_default(&home.to_str().unwrap())
        .prompt()
        .unwrap();

    let _ = remove_dir_all(&dir);

    let bar = ProgressBar::new_spinner()
        .with_message("Fetching packages...");
    bar.enable_steady_tick(Duration::from_millis(1));

    let zip = utils::get_bin_zip().await;

    tokio::time::sleep(Duration::from_secs(1)).await;

    let asset = zip.assets.into_iter()
        .find(|x| {
            use std::env::consts::{OS, ARCH};
            x.name.starts_with("binaries_") && x.name.contains(&OS.replace("macos", "darwin")) && x.name.contains(ARCH)
        })
        .unwrap();

    drop(bar);

    let p_bar = ProgressBar::new_spinner()
        .with_message("Installing...");
    
    p_bar.enable_steady_tick(Duration::from_millis(3));

    utils::download_install_lead(&asset.browser_download_url, &dir).await;
    utils::postinstall(&dir).await;
}
