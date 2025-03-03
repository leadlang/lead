use std::{borrow::Cow, collections::HashMap, future::Future, pin::Pin};

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
          format!("{}:{}", &file_name, line + 1),
          content,
          &mut app,
          &mut app2.heap,
          &mut line,
          &mut markers,
          false,
          None,
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
  orig_opt: Option<&mut Options>,
) -> Option<Pin<Box<dyn Future<Output = ()> + 'a>>> {
  let heap: &'static mut Heap = unsafe { &mut *heap };

  let tokens: Vec<*const str> = piece.split(" ").map(|x| x as *const str).collect();

  let mut caller = unsafe { &*tokens[0] };
  let mut val_type = false;

  let mut to_set = "";
  let mut start = 0;

  if caller.ends_with(":") && caller.starts_with("$") {
    val_type = true;
    let l = unsafe { &*tokens[0] };
    let set = l.split_at(l.len() - 1).0;

    to_set = set;

    start = 1;
    caller = unsafe { &*tokens[1] };
  }

  let mut opt = Options::new();

  if caller.starts_with("*if$") {
    let caller = unsafe { &*tokens[start] };

    let caller = caller.replacen("*if", "", 1);

    let BufValue::Bool(x) = heap.get(&caller).expect("Unable to get the value") else {
      panic!("Invalid type, expected boolean in *if");
    };

    if *x {
      return tok_parse(
        file,
        &piece[unsafe { &*tokens[start] }.len() + 1..],
        app,
        heap,
        line,
        markers,
        r#async,
        orig_opt,
      );
    }

    None
  } else if caller.starts_with("*else$") {
    let caller = unsafe { &*tokens[start] };

    let caller = caller.replacen("*else", "", 1);

    let BufValue::Bool(x) = heap.get(&caller).expect("Unable to get the value") else {
      panic!("Invalid type, expected boolean in *if");
    };

    if !*x {
      return tok_parse(
        file,
        &piece[unsafe { &*tokens[start] }.len() + 1..],
        app,
        heap,
        line,
        markers,
        r#async,
        orig_opt,
      );
    }

    None
  } else if caller == "*return" {
    let Some(opt) = orig_opt else {
      error("*return can only be called from a lead module", file);
    };

    let var = unsafe { &*tokens[start + 1] };

    opt.r_val = Some(
      heap
        .remove(var)
        .expect("Cannot find variable")
        .expect("Cannot find variable")
        .into(),
    );

    None
  } else if caller.starts_with("*") {
    insert_into_application(
      app as *mut _ as _,
      &tokens[start..],
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
    let tokens_ptr = &tokens as &[*const str];
    let caller_ptr = caller as *const _;

    let wrap = HeapWrapper {
      heap: unsafe { &mut *app_heap_ptr },
      args: unsafe { &*(tokens_ptr as *const _) },
      pkg_name: unsafe { &*caller_ptr },
      app: app_ptr,
      allow_full: false
    };

    match call_runtime_val(
      app as _,
      heap,
      caller,
      &tokens[start..],
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
          let _ = heap.set(Cow::Borrowed(to_set), opt.r_val());
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
        return Some(Box::pin(async move {
          let v = v.await;

          let runt = opt.rem_r_runtime();

          if val_type && opt.r_val.is_some() {
            let _ = heap.set(Cow::Borrowed(to_set), opt.r_val());
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
    let tokens_ptr = &tokens as &[_];

    match app.pkg.inner.get_mut(caller) {
      Some((p, v)) => {
        let pkg: *const str = *p as *const _;
        let pkg = unsafe { &*pkg };

        let wrap = HeapWrapper {
          heap: unsafe { &mut *app_heap_ptr },
          args: unsafe { &*(tokens_ptr as *const _) },
          pkg_name: pkg,
          app: app_ptr,
          allow_full: true
        };

        v(&tokens[start..] as *const _, wrap, &file, &mut opt);

        let runt = opt.rem_r_runtime();

        if val_type && opt.r_val.is_some() {
          let _ = heap.set(Cow::Borrowed(to_set), opt.r_val());
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
