use std::env::args;

mod docs;
mod app;

/// The main entry point of the program.
///
/// This function will be called when the program starts, and is where program execution begins.
///
fn main() {
    println!("⚠️ Under Construction ⚠️");
    let args: Vec<String> = args().collect();

    let cmd0: &str = &args[1];

    match cmd0 {
        "" | "run" => {

        }
        "docs" => {
            docs::run_docs();
        }
        e => {
            if e != "help" {
                println!("Unknown command: {}", e);
            }
        }
    }
}
