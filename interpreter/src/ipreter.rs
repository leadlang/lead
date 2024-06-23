use crate::{
  error, runtime::_root_syntax::insert_into_application, types::{call_runtime_val, mkbuf, set_runtime_val, MethodData, Options}, Application
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

  if caller.starts_with("*") {
    insert_into_application(app as *mut _ as _, &tokens);
  } else if caller.starts_with("@") {
    if val_type == "$" {
      let _ = app.heap.set(to_set, mkbuf(&caller, &file));
    }
  } else if caller.starts_with("$") {
    match call_runtime_val(caller, &tokens, &mut app.heap, &file, &mut opt) {
      None => if &caller != &"" {
        error(&format!("Unexpected `{}`", &caller), &file);
      },
      Some(v) => {
        opt.pre = v.to_string();
        if opt.marker {
          app.next_marker = true;
        }

        let runt = opt.rem_r_runtime();

        if val_type == "*" {
          let _ = app.heap.set_ptr(to_set, opt.r_ptr_target, opt.r_ptr);
        } else if val_type == "$" && opt.r_val.is_some() {
          let _ = app.heap.set(to_set, opt.r_val.unwrap());
        } else if val_type == "$" && runt.is_some() {
          let _ = set_runtime_val(to_set, v, runt.unwrap());
        }
      }
    }
  } else {
    match app.pkg.inner.get_mut(caller) {
      Some(MethodData::Static(p, v)) => {
        v(&tokens, &mut app.heap, &file, &mut opt);

        let pkg: *const str = *p as *const _;
        let pkg = unsafe { &*pkg };

        opt.pre = pkg.to_string();

        if opt.marker {
          app.next_marker = true;
        }

        let runt = opt.rem_r_runtime();

        if val_type == "*" {
          let _ = app.heap.set_ptr(to_set, opt.r_ptr_target, opt.r_ptr);
        } else if val_type == "$" && opt.r_val.is_some() {
          let _ = app.heap.set(to_set, opt.r_val.unwrap());
        } else if val_type == "$" && runt.is_some() {
          let _ = set_runtime_val(to_set, pkg, runt.unwrap());
        }
      }
      _ => {
        match app.modules.get_mut(caller) {
          _ => if &caller != &"" {
            error(&format!("Unexpected `{}`", &caller), &file);
          }
        }
      }
    }
  }
}
