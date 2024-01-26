use crate::{types::MethodData, Application};

pub fn interpret(file: &str, mut app: &mut Application) {
  let file = app.code.get(file).unwrap();

  let file = file.replace("\r", "");
  file.split("\n").for_each(|piece| {
    if piece.starts_with("!") && app.next_marker {
      if piece == "!end" {
        app.next_marker = false;
      } else {
        let piece = piece.replacen("!", "", 1);
        tok_parse(&piece, &mut app);
      }
    } else if piece.starts_with(":") {
      tok_parse(piece, &mut app);
    }
  });
}

fn tok_parse(piece: &str, app: &mut Application) {
  let tokens: Vec<String> = piece.split(" ").map(|x| x.to_string()).collect();

  match app.pkg.inner.get_mut(tokens[0].as_str()) {
    Some(MethodData::Static(_, v)) => v(&tokens, &mut app.heap, &mut app.next_marker),
    _ => {}
  }
}
