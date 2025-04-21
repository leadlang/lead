use std::{
  borrow::Cow,
  collections::HashMap,
  mem::transmute,
  ops::{Deref, DerefMut},
  time::Duration,
};

use tokio::{
  runtime::Builder,
  task::{spawn_blocking, yield_now},
  time::Instant,
};

use crate::{
  error,
  runtime::_root_syntax::insert_into_application,
  types::{
    call_runtime_val, get_runtime_ptr, mkbuf, set_runtime_val, BufValue, Heap, HeapWrapper,
    Options, RawRTValue, SafePtr, SafePtrMut,
  },
  Application, LeadCode, Scheduler,
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

pub fn schedule<'a>(app: &mut Application<'a>) {
  let runtime = Builder::new_current_thread()
    .enable_all()
    .build()
    .expect("Unable to build async runtime");

  let app: Wrapped<Application<'static>> = unsafe { Wrapped::new(app) };
  let mut scheduler: Wrapped<Scheduler> = unsafe { Wrapped::new(&mut Scheduler::new(app)) };

  runtime.block_on(async move {
    loop {
      scheduler.manage().await;
      // We instantly go to sleep to allow LeadLang to evaluate function and code
      // This also means that there's at least a 10 millis time frame when you can expect scheduler to give you a Mutex<T>
      tokio::time::sleep(Duration::from_millis(10)).await;
    }
  });
}

pub struct AsyncHeapHelper {
  inner: Heap,
  blockmap: HashMap<*const (), bool>,
}

impl From<Heap> for AsyncHeapHelper {
  fn from(value: Heap) -> Self {
    Self {
      inner: value,
      blockmap: HashMap::new(),
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
        app.0,
        unsafe { transmute(&mut heap) },
        unsafe { transmute(&mut line) },
        unsafe { transmute(&mut markers) },
        None,
      )
      .await;
    }

    line += 1;

    if total <= 0 {
      // Did a heavy task. Yielding...
      yield_now().await;
    }
  }
}

enum Args<'a> {
  Ignore,
  Args(&'a [&'static str], &'static str, bool),
}

/// This (sync) function quickly runs a recursive scan to check for state
pub fn check_state<'a>(
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

    let caller = &caller[2..];

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

  Args::Args(args, to_set, val_type)
}

pub async fn tok_run(
  total: &mut i8,
  args: &'static [&'static str],
  to_set: &'static str,
  val_type: bool,
  file: &'static str,
  app: *mut Application<'static>,
  heap: &'static mut AsyncHeapHelper,
  line: &'static mut usize,
  markers: &'static mut HashMap<Cow<'static, str>, usize>,
  orig_opt: Option<&'static mut Options>,
) {
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

    let blocking = heap.blockmap.get(&ptr).map_or_else(|| None, |x| Some(*x));

    let app = SafePtrMut(app);
    let hp = SafePtrMut(&mut heap.inner);
    let caller = SafePtr(caller);
    let args = SafePtr(args);
    let file = SafePtr(file as *const str);
    let line = SafePtr(line);

    let run = async move || {
      let _opt = SafePtrMut(&mut opt);
      
      match call_runtime_val(
        app,
        hp,
        caller,
        args,
        wrap,
        _opt,
        file,
        line,
      ) {
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
        Some(fut) => Some(async move {
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
        }),
      };
    };

    let Some(x) = blocking else {
      let t0 = Instant::now();

      spawn_blocking(run).await;

      // 10ms at max should be the blocking time
      heap.blockmap.insert(ptr, t0.elapsed().as_millis() > 10);
      return ();
    };

    if x {
      spawn_blocking(run).await;
      return ();
    }

    run();

    return ();
  }
}
