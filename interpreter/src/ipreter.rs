use crate::{
  error,
  types::{mkbuf, MethodData, Options},
  Application,
};

pub fn interpret(file: &str, mut app: &mut Application) {
  let file_name = if file == ":entry" { app.entry } else { file };

  let file = app.code.get(file).unwrap();

  let file = file.replace("\r", "");
  file.split("\n").enumerate().for_each(|(line, piece)| {
    if piece.starts_with("!") && app.next_marker {
      if piece == "!end" {
        app.next_marker = false;
      } else {
        let piece = piece.replacen("!", "", 1);
        tok_parse(format!("{}:{}", &file_name, line + 1), &piece, &mut app);
      }
    } else if !piece.starts_with("#") {
      tok_parse(format!("{}:{}", &file_name, line + 1), piece, &mut app);
    }
  });
}

fn tok_parse(file: String, piece: &str, app: &mut Application) {
  let mut tokens: Vec<String> = piece.split(" ").map(|x| x.to_string()).collect();

  let mut caller = tokens[0].as_str();
  let mut val_type = "<-none->";

  let mut to_set = String::new();

  if tokens[0].ends_with(":") && (tokens[0].starts_with("*") || tokens[0].starts_with("$")) {
    if tokens[0].starts_with("*") {
      val_type = "*";
    } else {
      val_type = "$";
    }

    to_set = tokens.remove(0);
    to_set = to_set.split_at(to_set.len() - 1).0.into();

    caller = tokens[0].as_str();
  }

  let mut opt = Options::new();

  if caller.starts_with("@") {
    if val_type == "$" {
      let _ = app.heap.set(to_set, mkbuf(&caller, &file));
    }
  } else {
    match app.pkg.inner.get_mut(caller) {
      Some(MethodData::Static(_, v)) => {
        v(&tokens, &mut app.heap, &file, &mut opt);

        if opt.marker {
          app.next_marker = true;
        }

        if val_type == "*" {
          let _ = app.heap.set_ptr(to_set, opt.r_ptr_target, opt.r_ptr);
        } else if val_type == "$" && opt.r_val.is_some() {
          let _ = app.heap.set(to_set, opt.r_val.unwrap());
        }
      }
      _ => {
        if &caller != &"" {
          error(&format!("Unexpected `{}`", &caller), &file);
        }
      }
    }
  }
}
