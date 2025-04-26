use std::{
  borrow::Cow,
  collections::HashMap,
  mem::transmute,
  ops::{Deref, DerefMut},
};

use tokio::{
  runtime::Runtime,
  task::{spawn_blocking, yield_now},
  time::Instant,
};

use crate::{
  error,
  runtime::_root_syntax::{insert_into_application, Sendify},
  types::{
    call_runtime_val, get_runtime_ptr, mkbuf, set_runtime_val, BufValue, Heap, HeapWrapper,
    Options, RawRTValue, SafePtr, SafePtrMut,
  },
  Application, LeadCode,
};

pub(crate) struct Wrapped<T>(pub(crate) *mut T);

unsafe impl<E> Send for Wrapped<E> {}
unsafe impl<E> Sync for Wrapped<E> {}

impl<E> Clone for Wrapped<E> {
  fn clone(&self) -> Self {
    Self(self.0)
  }
}

impl<E> Copy for Wrapped<E> {}

impl<E> Wrapped<E> {
  unsafe fn new<T>(app: &mut T) -> Self {
    unsafe {
      let app: &mut E = std::mem::transmute(app);
      let app = app as *mut E;

      Self(app)
    }
  }
}

impl<T> Deref for Wrapped<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    let i = unsafe { &*self.0 };

    i
  }
}

impl<E> DerefMut for Wrapped<E> {
  fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe { &mut *self.0 }
  }
}

pub fn schedule<'a>(runtime: Runtime, app: &mut Application<'a>) {
  let app: Wrapped<Application<'static>> = unsafe { Wrapped::new(app) };

  runtime.block_on(async move {
    interpret(":entry", app).await;
  });
}

#[derive(Debug)]
pub(crate) struct Timing {
  curr: u8,
  // We store `10` readings to compare
  times: [u128; 10],
}

impl Default for Timing {
  fn default() -> Self {
    Self {
      curr: 0,
      times: [0; 10],
    }
  }
}

#[derive(Debug)]
pub(crate) struct AsyncHeapHelper {
  inner: Heap,
  blockmap: HashMap<*const (), bool>,
  timings: HashMap<*const (), Timing>,
}

impl AsyncHeapHelper {
  pub(crate) fn is_blocking(&self, f: *const ()) -> Option<&bool> {
    self.blockmap.get(&f)
  }

  pub(crate) fn report(&mut self, f: *const (), micros: u128) {
    let timing = self.timings.entry(f).or_insert(Timing::default());

    timing.times[timing.curr as usize] = micros;
    timing.curr += 1;

    // Compare
    if timing.curr == 10 {
      let data = self.timings.remove(&f).expect("Timing MUST exist");

      let avg = data.times.into_iter().sum::<u128>() / 10u128;

      // If less than `200 micro sec`, it is non blocking
      // 40 micro sec penatly for spawn_blocking
      self.blockmap.insert(f, avg >= 240);
    }
  }
}

impl From<Heap> for AsyncHeapHelper {
  fn from(value: Heap) -> Self {
    Self {
      inner: value,
      blockmap: HashMap::new(),
      timings: HashMap::new(),
    }
  }
}

impl Deref for AsyncHeapHelper {
  type Target = Heap;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

impl DerefMut for AsyncHeapHelper {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.inner
  }
}

unsafe impl Send for AsyncHeapHelper {}
unsafe impl Sync for AsyncHeapHelper {}

pub async fn interpret(file: &str, app: Wrapped<Application<'static>>) {
  let app2 = app.clone();
  let Some(x) = app2.code.get(file) else {
    error("Unable to find file", file);
  };

  let LeadCode::Code(x) = x else {
    error("Expected Lead Code, found Lead Module Code", file);
  };

  let len = x.len();
  let mut line = 0;

  let heap = Heap::new(app.pkg.extends.clone());
  let mut heap = AsyncHeapHelper::from(heap);

  let mut markers: HashMap<Cow<'static, str>, usize> = HashMap::new();

  let mut total = 8;

  while line < len {
    let args = &x[line];

    if let Args::Args(x, to_set, val_type) =
      check_state(&mut total, args, file, &mut heap, &mut line)
    {
      tok_run(
        &mut total,
        unsafe { transmute(x) },
        to_set,
        val_type,
        unsafe { transmute(file) },
        SafePtrMut(app.0),
        unsafe { transmute(&mut heap) },
        unsafe { transmute(&mut line) },
        unsafe { transmute(&mut markers) },
        None,
      )
      .await;
    }

    line += 1;

    if total <= 0 {
      total = 8;
      // Did a heavy task. Yielding...
      yield_now().await;
    }
  }
}

pub(crate) enum Args<'a> {
  Ignore,
  Args(&'a [&'static str], &'static str, bool),
}

/// This (sync) function quickly runs a recursive scan to check for state
pub(crate) fn check_state<'a>(
  complexity: &mut i8,
  args: &'a [&'static str],
  file: &str,
  heap: &mut AsyncHeapHelper,
  line: &mut usize,
) -> Args<'a> {
  let mut caller = &*args[0];

  let mut start = 0;

  let mut val_type = false;
  let mut to_set = "";

  if caller.ends_with(":") && caller.starts_with("$") {
    val_type = true;
    let l = args[0];
    let set = l.split_at(l.len() - 1).0;

    to_set = set;

    start = 1;
    caller = args[1];
  }

  if &caller[0..1] == "#" {
    return Args::Ignore;
  }

  let r#if = caller.starts_with("*if$");
  let r#else = caller.starts_with("*else$");

  if r#if || r#else {
    let caller = &*args[start];

    let caller = if r#if { &caller[3..] } else { &caller[5..] };

    let BufValue::Bool(x) = heap.get(&caller).expect("Unable to get value") else {
      error(
        "Invalid type, expected boolean in *if",
        format!("{file}:{line}"),
      );
    };

    *complexity -= 1;

    if r#if && *x {
      return check_state(complexity, &args[start + 1..], file, heap, line);
    } else if r#else && !*x {
      return check_state(complexity, &args[start + 1..], file, heap, line);
    }

    return Args::Ignore;
  }

  Args::Args(&args[start..], to_set, val_type)
}

pub async fn tok_run(
  total: &mut i8,
  args: &'static [&'static str],
  to_set: &'static str,
  val_type: bool,
  file: &'static str,
  app: SafePtrMut<Application<'static>>,
  heap: &'static mut AsyncHeapHelper,
  line: &'static mut usize,
  markers: &'static mut HashMap<Cow<'static, str>, usize>,
  orig_opt: Option<&'static mut Options>,
) {
  let app = app.0;
  let args: &'static [&'static str] = unsafe { transmute(args) };

  let app = unsafe { &mut *app };

  let caller = args[0];

  let mut opt = Options::new();

  if caller == "*return" {
    *total -= 1;

    let Some(opt) = orig_opt else {
      error("*return can only be called from a lead module", file);
    };

    let var = args[1];

    opt.r_val = Some(
      heap
        .remove(var)
        .expect("Cannot find variable")
        .expect("Cannot find variable")
        .into(),
    );

    return ();
  }

  if &caller[0..1] == "@" {
    *total -= 1;

    if val_type {
      let _ = heap.set(Cow::Borrowed(to_set), mkbuf(&caller, &file));
    }

    return ();
  }

  if &caller[0..1] == "*" {
    *total -= 1;

    insert_into_application(
      SafePtrMut(app),
      SafePtr(args),
      SafePtrMut(line),
      Cow::Borrowed(to_set),
      SafePtrMut(heap.deref_mut()),
      SafePtrMut(markers),
    )
    .await;

    return ();
  }

  if &caller[0..1] == "$" {
    *total -= 1;

    let app = app as *mut _;

    let mut heap: Wrapped<AsyncHeapHelper> = unsafe { Wrapped::new(heap) };

    let heap2 = &mut heap.inner as *mut Heap;

    let wrap: HeapWrapper<'static> = HeapWrapper {
      heap: unsafe { &mut *heap2 },
      args,
      allow_full: false,
      app,
      pkg_name: caller,
    };

    let app: &'static mut Application<'static> = unsafe { transmute(&mut *app) };

    let Some(ptr) = get_runtime_ptr(&mut *heap, caller, file, line) else {
      error(
        &format!("Unexpected `{}`", &caller),
        format!("{file}:{line}"),
      );
    };

    let blocking = heap.is_blocking(ptr).map(|x| *x);

    let app = SafePtrMut(app);
    let hp = SafePtrMut(&mut heap.inner);
    let caller = SafePtr(caller);
    let args = SafePtr(args);
    let file = SafePtr(file as *const str);
    let line = SafePtr(line);

    let run = move || {
      let _opt = SafePtrMut(&mut opt);

      return match call_runtime_val(app, hp, caller, args, wrap, _opt, file, line) {
        None => {
          let runt = opt.rem_r_runtime();

          if val_type && opt.r_val.is_some() {
            let _ = heap.set(Cow::Borrowed(to_set), opt.r_val());
          } else if val_type && runt.is_some() {
            let _ = set_runtime_val(
              &mut heap,
              Cow::Borrowed(to_set),
              RawRTValue::RT(runt.unwrap()),
            );
          }
          None
        }
        Some(fut) => Some(Box::pin(Sendify(async move {
          fut.await;

          let runt = opt.rem_r_runtime();

          if val_type && opt.r_val.is_some() {
            let _ = heap.set(Cow::Borrowed(to_set), opt.r_val());
          } else if val_type && runt.is_some() {
            let _ = set_runtime_val(
              &mut heap,
              Cow::Borrowed(to_set),
              RawRTValue::RT(runt.unwrap()),
            );
          }
        }))),
      };
    };

    let Some(x) = blocking else {
      let t0 = Instant::now();

      let run = Sendify(run);

      let out = spawn_blocking(move || (run.0)())
        .await
        .expect("Didn't expect an error");

      // Report before await
      heap.report(ptr, t0.elapsed().as_micros());
      if let Some(out) = out {
        out.await;
      }

      return ();
    };

    if x {
      let run = Sendify(run);
      if let Some(x) = spawn_blocking(move || (run.0)())
        .await
        .expect("Didn't expect an error")
      {
        x.await;
      }
      return ();
    }

    if let Some(x) = run() {
      x.await;
    }

    return ();
  }

  let app = SafePtrMut(app);
  let hp = SafePtrMut(&mut heap.inner);
  let file = SafePtr(file as *const str);

  match unsafe { &mut *app.0 }.pkg.inner.get_mut(caller) {
    Some((p, v)) => {
      let pkg: *const str = *p as *const _;
      let pkg = unsafe { &*pkg };

      let wrap = HeapWrapper {
        heap: unsafe { &mut *hp.0 },
        args: &args[1..],
        pkg_name: pkg,
        app: app.0,
        allow_full: true,
      };

      let v_ptr = v as *const _ as *const ();

      let blocking = heap.is_blocking(v_ptr).map(|x| *x);

      let opt_ptr = SafePtrMut(&mut opt);

      let f = move || {
        let opt_ptr = opt_ptr;

        v(args as *const _, wrap, &file.deref(), unsafe {
          &mut *opt_ptr.0
        })
      };

      match blocking {
        Some(true) => {
          spawn_blocking(f).await.expect("Didn't expect an error");
        }
        Some(false) => {
          f();
        }
        None => {
          let t0 = Instant::now();
          spawn_blocking(f).await.expect("Didn't expect an error");
          heap.report(v_ptr, t0.elapsed().as_micros());
        }
      };

      let runt = opt.rem_r_runtime();

      if val_type && opt.r_val.is_some() {
        let _ = heap.set(Cow::Borrowed(to_set), opt.r_val());
      } else if val_type && runt.is_some() {
        let _ = set_runtime_val(heap, Cow::Borrowed(to_set), RawRTValue::RT(runt.unwrap()));
      }

      return ();
    }
    _ => {
      if &caller != &"" {
        error(&format!("Unexpected `{}`", &caller), &file.deref());
      }

      return ();
    }
  }
}
