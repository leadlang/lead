use std::time::Duration;

use inquire::Text;
use dirs::home_dir;

use indicatif::ProgressBar;

fn main() {
    let mut home = home_dir().unwrap();
    home.push("leadLang");

    let folder = Text::new(&"Where shall we install lead?")
        .with_default(&home.to_str().unwrap())
        .prompt()
        .unwrap();

    let bar = ProgressBar::new_spinner()
        .with_message("Fetching packages...");
    bar.enable_steady_tick(Duration::from_millis(50));

    loop {}
}
