use crate::{
  error, runtime::_root_syntax::insert_into_application, types::{call_runtime_val, mkbuf, set_runtime_val, BufValue, HeapWrapper, Options, RawRTValue}, Application
};

pub fn interpret(file: &str, mut app: &mut Application) {
  let file_name = if file == ":entry" { app.entry } else { file };

  let file = app.code.get(file).unwrap();

  let file = file.replace("\r", "");
  let file = file.split("\n").collect::<Vec<_>>();

  let mut line = 0usize;

  while line < file.len() {
    let content = &file[line];

    if !content.starts_with("#") {
      tok_parse(format!("{}:{}", &file_name, line), content, &mut app, &mut line);
    }

    line += 1;
  }
}

fn tok_parse(file: String, piece: &str, app: &mut Application, line: &mut usize) {
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

  if caller.starts_with("*if$") {
    let caller = tokens.remove(0);

    let caller = caller.replacen("*if", "", 1);

    let BufValue::Bool(x) = app.heap.get(&caller).expect("Unable to get the value") else {
      panic!("Invalid type, expected boolean in *if");
    };

    let piece = tokens.join(" ");

    if *x {
      tok_parse(file, &piece, app, line);
    }
  } else if caller.starts_with("*else$") {
    let caller = tokens.remove(0);

    let caller = caller.replacen("*else", "", 1);

    let BufValue::Bool(x) = app.heap.get(&caller).expect("Unable to get the value") else {
      panic!("Invalid type, expected boolean in *if");
    };

    let piece = tokens.join(" ");

    if !*x {
      tok_parse(file, &piece, app, line);
    }
  } else if caller.starts_with("*") {
    insert_into_application(app as *mut _ as _, &tokens, line, to_set);
  } else if caller.starts_with("@") {
    if val_type == "$" {
      let _ = app.heap.set(to_set, mkbuf(&caller, &file));
    }
  } else if caller.starts_with("$") {
    let app_ptr = app as *mut _;
    let app_heap_ptr = &mut app.heap as *mut _;
    let tokens_ptr = &tokens as *const _;
    let caller_ptr = caller as *const _;

    let wrap = HeapWrapper {
      heap: unsafe { &mut *app_heap_ptr },
      args: unsafe { &*tokens_ptr },
      pkg_name: unsafe { &*caller_ptr },
      app: app_ptr,
    };

    match call_runtime_val(caller, &tokens, wrap, &file, &mut opt, &file) {
      None => if &caller != &"" {
        error(&format!("Unexpected `{}`", &caller), &file);
      },
      Some(v) => {
        opt.pre = v.to_string();

        let runt = opt.rem_r_runtime();

        if val_type == "*" {
          let _ = app.heap.set_ptr(to_set, opt.r_ptr_target, opt.r_ptr);
        } else if val_type == "$" && opt.r_val.is_some() {
          let _ = app.heap.set(to_set, opt.r_val.unwrap());
        } else if val_type == "$" && runt.is_some() {
          let _ = set_runtime_val(to_set, v, RawRTValue::RT(runt.unwrap()));
        }
      }
    }
  } else {
    let app_ptr = app as *mut _;
    let app_heap_ptr = &mut app.heap as *mut _;
    let tokens_ptr = &tokens as *const _;

    match app.pkg.inner.get_mut(caller) {
      Some((p, v)) => {
        let pkg: *const str = *p as *const _;
        let pkg = unsafe { &*pkg };

        let wrap = HeapWrapper {
          heap: unsafe { &mut *app_heap_ptr },
          args: unsafe { &*tokens_ptr },
          pkg_name: pkg,
          app: app_ptr,
        };

        v(&tokens, wrap, &file, &mut opt);

        opt.pre = pkg.to_string();

        let runt = opt.rem_r_runtime();

        if val_type == "*" {
          let _ = app.heap.set_ptr(to_set, opt.r_ptr_target, opt.r_ptr);
        } else if val_type == "$" && opt.r_val.is_some() {
          let _ = app.heap.set(to_set, opt.r_val.unwrap());
        } else if val_type == "$" && runt.is_some() {
          let _ = set_runtime_val(to_set, pkg, RawRTValue::RT(runt.unwrap()));
        }
      }
      _ => {
        match app.modules.get_mut(caller) {
          Some(v) => {
            
          }
          _ => if &caller != &"" {
            error(&format!("Unexpected `{}`", &caller), &file);
          }
        }
      }
    }
  }
}
