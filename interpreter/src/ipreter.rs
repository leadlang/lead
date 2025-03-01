use std::{borrow::Cow, collections::HashMap, future::Future};

use crate::{
  error,
  runtime::_root_syntax::insert_into_application,
  types::{
    call_runtime_val, mkbuf, set_runtime_val, BufValue, Heap, HeapWrapper, Options, Output,
    RawRTValue,
  },
  Application,
};

pub fn interpret<'a>(file: &str, mut app: &mut Application<'a>) {
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

  let app2: *mut Application<'static> = unsafe { std::mem::transmute(app as *mut Application) };

  let app2: &'static mut Application<'static> = unsafe { &mut *app2 };

  let mut markers = HashMap::new();

  while line < file.len() {
    let content = &file[line];

    if !content.starts_with("#") {
      unsafe {
        let f = tok_parse(
          format!("{}:{}", &file_name, line+1),
          content,
          &mut app,
          &mut app2.heap,
          &mut line,
          &mut markers,
          false,
        );

        drop(f);
      }
    }

    line += 1;
  }
}

pub(crate) unsafe fn tok_parse<'a>(
  file: String,
  piece: &str,
  app: &mut Application<'a>,
  heap: *mut Heap,
  line: &mut usize,
  markers: &mut HashMap<Cow<'static, str>, usize>,
  r#async: bool,
) -> Option<Box<dyn Future<Output = ()> + 'a>> {
  let heap: &'static mut Heap = unsafe { &mut *heap };

  let mut tokens: Vec<*const str> = piece.split(" ").map(|x| x as *const str).collect();

  let mut caller = unsafe { &*tokens[0] };
  let mut val_type = false;

  let mut to_set = "";

  if caller.ends_with(":") && caller.starts_with("$") {
    val_type = true;
    let l = unsafe { &*tokens.remove(0) };
    let set = l.split_at(l.len() - 1).0;

    to_set = set;

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
      return tok_parse(file, &piece, app, heap, line, markers, r#async);
    }

    None
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
      return tok_parse(file, &piece, app, heap, line, markers, r#async);
    }

    None
  } else if caller.starts_with("*") {
    insert_into_application(
      app as *mut _ as _,
      &tokens,
      line,
      Cow::Borrowed(to_set),
      heap,
      markers,
    );

    None
  } else if caller.starts_with("@") {
    if val_type {
      let _ = heap.set(Cow::Borrowed(to_set), mkbuf(&caller, &file));
    }

    None
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

    match call_runtime_val(
      app as _,
      heap,
      caller,
      &mut tokens,
      wrap,
      &file,
      &mut opt,
      &file,
      r#async,
    ) {
      None => {
        if &caller != &"" {
          error(&format!("Unexpected `{}`", &caller), &file);
        }

        None
      }
      Some(Output::String(v)) => {
        let runt = opt.rem_r_runtime();

        if val_type && opt.r_val.is_some() {
          let _ = heap.set(Cow::Borrowed(to_set), opt.r_val.unwrap());
        } else if val_type && runt.is_some() {
          let _ = set_runtime_val(
            heap,
            Cow::Borrowed(to_set),
            v,
            RawRTValue::RT(runt.unwrap()),
          );
        }

        None
      }
      Some(Output::Future(v)) => {
        return Some(Box::new(async move {
          let v = v.await;

          let runt = opt.rem_r_runtime();

          if val_type && opt.r_val.is_some() {
            let _ = heap.set(Cow::Borrowed(to_set), opt.r_val.unwrap());
          } else if val_type && runt.is_some() {
            let _ = set_runtime_val(
              heap,
              Cow::Borrowed(to_set),
              v,
              RawRTValue::RT(runt.unwrap()),
            );
          }
        }));
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

        let runt = opt.rem_r_runtime();

        if val_type && opt.r_val.is_some() {
          let _ = heap.set(Cow::Borrowed(to_set), opt.r_val.unwrap());
        } else if val_type && runt.is_some() {
          let _ = set_runtime_val(
            heap,
            Cow::Borrowed(to_set),
            pkg,
            RawRTValue::RT(runt.unwrap()),
          );
        }

        None
      }
      _ => {
        if &caller != &"" {
          error(&format!("Unexpected `{}`", &caller), &file);
        }

        None
      }
    }
  }
}
