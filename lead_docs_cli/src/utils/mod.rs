use std::collections::HashMap;

use inquire::Select;

mod docs;
mod package;

pub fn make_sel() {
  let select = vec!["📚 Lead Default", "⚒️  Workspace"];

  let category = Select::new("Select", select)
    .prompt()
    .expect("You must select one...");

  let data = match category {
    "📚 Lead Default" => docs::lead_lib(),
    "⚒️  Workspace" => docs::lead_ws(),
    _ => panic!("Invalid category"),
  };

  let package = docs::prompt(data);
  let doc: HashMap<String, HashMap<&str, &str>> = package.doc;

  println!("{doc:?}");
  navigate(&package.display, &doc);
}

fn navigate(display: &str, doc: &HashMap<String, HashMap<&str, &str>>) {
  let mut root: Option<&str> = None;

  #[allow(unused_assignments)]
  let mut last = None;

  let mut current_level: u8 = 0;

  loop {
    let mut choices = if current_level == 0 { vec![] } else { vec![".."] };
    let mut c = match current_level {
      1 => doc.get(*root.as_ref().unwrap()).unwrap().iter().map(|(a,_)| a as &str).collect::<Vec<_>>(),
      0 => doc.iter().map(|(a, _)| a as &str).collect::<Vec<_>>(),
      _ => panic!("Unknown Level")
    };
    
    choices.append(&mut c);
    choices.push("❌ Quit");

    let sel = Select::new(&format!("Inside of {display}"), choices).prompt().expect("You must select one...");

    match sel {
      ".." => {
        current_level -= 1;
      },
      "❌ Quit" => return (),
      e => {
        current_level += 1;

        match current_level {
          1 => root = Some(e),
          2 => {
            last = Some(e);
            break;
          },
          _ => panic!("Unknown Level")
        }
      }
    }
  }

  println!("? {last:?}");
}
