use crate::{
  error,
  runtime::_root_syntax::insert_into_application,
  types::{
    call_runtime_val, mkbuf, set_runtime_val, BufValue, Heap, HeapWrapper, Options, RawRTValue,
  },
  Application,
};

pub fn interpret(file: &str, mut app: &mut Application) {
  let file_name = if file == ":entry" { app.entry } else { file };

  let app_ptr = app as *mut Application;

  let file = app.code.get(file).unwrap_or_else(move || {
    let app = unsafe { &mut *app_ptr };
    let data = app.module_resolver.call_mut((&format!("./{file}.pb"),));

    app
      .code
      .insert(file.into(), String::from_utf8(data).unwrap());

    app.code.get(file).expect("Impossible")
  });

  let file = file.replace("\r", "");
  let file = file.split("\n").collect::<Vec<_>>();

  let mut line = 0usize;

  let app2 = app as *mut Application;

  let app2 = unsafe { &mut *app2 };

  while line < file.len() {
    let content = &file[line];

    if !content.starts_with("#") {
      unsafe {tok_parse(
        format!("{}:{}", &file_name, line),
        content,
        &mut app,
        &mut app2.heap,
        &mut line,
      );}
    }

    line += 1;
  }
}

pub(crate) unsafe fn tok_parse(
  file: String,
  piece: &str,
  app: &mut Application,
  heap: &mut Heap,
  line: &mut usize,
) {
  let mut tokens: Vec<*const str> = piece.split(" ").map(|x| x as *const str).collect();

  let mut caller = unsafe { &*tokens[0] };
  let mut val_type = "<-none->";

  let mut to_set = String::new();

  if caller.ends_with(":") && (caller.starts_with("*") || caller.starts_with("$")) {
    if caller.starts_with("*") {
      val_type = "*";
    } else {
      val_type = "$";
    }

    let l = unsafe { &*tokens.remove(0) };
    to_set = l.split_at(l.len() - 1).0.into();

    caller = unsafe { &*tokens[0] };
  }

  let mut opt = Options::new();

  if caller.starts_with("*if$") {
    let caller = unsafe { &*tokens.remove(0) };

    let caller = caller.replacen("*if", "", 1);

    let BufValue::Bool(x) = heap.get(&caller).expect("Unable to get the value") else {
      panic!("Invalid type, expected boolean in *if");
    };

    let piece = tokens
      .into_iter()
      .map(|x| unsafe { &*x })
      .collect::<Vec<_>>()
      .join(" ");

    if *x {
      tok_parse(file, &piece, app, heap, line);
    }
  } else if caller.starts_with("*else$") {
    let caller = unsafe { &*tokens.remove(0) };

    let caller = caller.replacen("*else", "", 1);

    let BufValue::Bool(x) = heap.get(&caller).expect("Unable to get the value") else {
      panic!("Invalid type, expected boolean in *if");
    };

    let piece = tokens
      .into_iter()
      .map(|x| unsafe { &*x })
      .collect::<Vec<_>>()
      .join(" ");

    if !*x {
      tok_parse(file, &piece, app, heap, line);
    }
  } else if caller.starts_with("*") {
    insert_into_application(app as *mut _ as _, &tokens, line, to_set);
  } else if caller.starts_with("@") {
    if val_type == "$" {
      let _ = heap.set(to_set, mkbuf(&caller, &file));
    }
  } else if caller.starts_with("$") {
    let app_ptr = app as *mut _;
    let app_heap_ptr = heap as *mut _;
    let tokens_ptr = &tokens as *const _;
    let caller_ptr = caller as *const _;

    let wrap = HeapWrapper {
      heap: unsafe { &mut *app_heap_ptr },
      args: unsafe { &*tokens_ptr },
      pkg_name: unsafe { &*caller_ptr },
      app: app_ptr,
    };

    match call_runtime_val(heap, caller, &tokens, wrap, &file, &mut opt, &file) {
      None => {
        if &caller != &"" {
          error(&format!("Unexpected `{}`", &caller), &file);
        }
      }
      Some(v) => {
        opt.pre = v.to_string();

        let runt = opt.rem_r_runtime();

        if val_type == "*" {
          let _ = heap.set_ptr(to_set, opt.r_ptr_target, opt.r_ptr);
        } else if val_type == "$" && opt.r_val.is_some() {
          let _ = heap.set(to_set, opt.r_val.unwrap());
        } else if val_type == "$" && runt.is_some() {
          let _ = set_runtime_val(heap, to_set, v, RawRTValue::RT(runt.unwrap()));
        }
      }
    }
  } else {
    let app_ptr = app as *mut _;
    let app_heap_ptr = heap as *mut _;
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
          let _ = heap.set_ptr(to_set, opt.r_ptr_target, opt.r_ptr);
        } else if val_type == "$" && opt.r_val.is_some() {
          let _ = heap.set(to_set, opt.r_val.unwrap());
        } else if val_type == "$" && runt.is_some() {
          let _ = set_runtime_val(heap, to_set, pkg, RawRTValue::RT(runt.unwrap()));
        }
      }
      _ => {
        let app2 = app as *mut Application;

        match app.modules.get_mut(caller) {
          Some(v) => {
            let tkns = tokens.drain(2..).collect::<Vec<_>>();
            let token0 = &*tokens.remove(1);

            v.run_method(app2, &token0, &file, move |fn_heap, app_heap, args| {
              if tkns.len() != args.len() {
                error(
                  "Not all arguments provided",
                  ":interpreter:loadmodule:heap:check",
                );
              }

              tkns.into_iter().zip(args.iter()).for_each(|(token, arg)| {
                let token = unsafe { &*token };
                let from = app_heap
                  .remove(token)
                  .unwrap_or_else(|| {
                    error(
                      format!("Unable to get {token} from Heap"),
                      ":interpreter:loadmodule",
                    )
                  })
                  .unwrap_or_else(|| {
                    error(
                      format!("Unable to get {token} from Heap"),
                      ":interpreter:loadmodule",
                    )
                  });

                fn_heap
                  .set((*arg as &str).replacen("->$", "$", 1), from)
                  .unwrap();
              });
            });
          }
          _ => {
            if &caller != &"" {
              error(&format!("Unexpected `{}`", &caller), &file);
            }
          }
        }
      }
    }
  }
}
