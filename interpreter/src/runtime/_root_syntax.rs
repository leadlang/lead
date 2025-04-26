use tokio::task::yield_now;

use crate::{
  error,
  parallel_ipreter::{check_state, interpret, tok_run, Args, AsyncHeapHelper, Wrapped},
  types::{
    set_into_extends, set_runtime_val, BufValue, Heap, Options, RawRTValue, SafePtr, SafePtrMut,
  },
  Application, ExtendsInternal, LeadCode, RespPackage, StaticLeadModule,
};
use std::{
  borrow::Cow,
  collections::HashMap,
  future::Future,
  mem::transmute,
  pin::Pin,
  sync::Arc,
  task::{Context, Poll},
};

#[derive(Debug)]
pub(crate) struct RTCreatedModule {
  pub(crate) heap: Heap,
  pub(crate) methods: StaticLeadModule,
}

impl RTCreatedModule {
  pub(crate) async fn run_method<T: FnOnce(&mut Heap, &mut Heap, &[&str]) -> ()>(
    &mut self,
    app: *mut Application<'static>,
    method: &str,
    file: &str,
    into_heap: T,
    heap: &mut Heap,
    opt: &mut Options,
  ) {
    let opt = SafePtrMut(opt);
    let app = Wrapped(app);
    let mut temp_heap = Heap::new_with_this(&mut self.heap, app.pkg.extends.clone());

    let (args, method_code) = self
      .methods
      .get(&method)
      .unwrap_or_else(|| error("Unable to find :method", file));
    into_heap(&mut temp_heap, heap, args);

    // run
    let file_name = ":fn";

    let file = method_code;

    let mut line = 0usize;

    let mut markers: HashMap<Cow<'static, str>, usize> = HashMap::new();

    let len = file.len();

    let mut total: i8 = 8;

    let mut heap = AsyncHeapHelper::from(temp_heap);

    while line < len {
      let args = file[line];

      let t = SafePtrMut(&mut total);

      if let Args::Args(x, to_set, val_type) =
        check_state(unsafe { &mut *t.0 }, args, file_name, &mut heap, &mut line)
      {
        tokio::spawn(Sendify(tok_run(
          unsafe { &mut *t.0 },
          unsafe { transmute(x) },
          to_set,
          val_type,
          ":fn",
          SafePtrMut(app.0),
          unsafe { transmute(&mut heap) },
          unsafe { transmute(&mut line) },
          unsafe { transmute(&mut markers) },
          Some(unsafe { &mut *opt.0 }),
        )))
        .await
        .expect("Expected no errors");
      }

      line += 1;

      if total <= 0 {
        total = 8;
        // Did a heavy task. Yielding...
        yield_now().await;
      }
    }
  }
}

pub struct Sendify<Fut>(pub(crate) Fut);

unsafe impl<Fut> Send for Sendify<Fut> {}
unsafe impl<Fut> Sync for Sendify<Fut> {}

impl<Fut: Future> Future for Sendify<Fut> {
  type Output = Fut::Output;

  fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    unsafe { self.map_unchecked_mut(|s| &mut s.0) }.poll(cx)
  }
}

#[allow(unused)]
pub(crate) async fn insert_into_application(
  app: SafePtrMut<Application<'static>>,
  args: SafePtr<[&'static str]>,
  line: SafePtrMut<usize>,
  to_set: Cow<'static, str>,
  heap: SafePtrMut<Heap>,
  markers: SafePtrMut<HashMap<Cow<'static, str>, usize>>,
) {
  let app = unsafe { &mut *app.0 };
  let heap = unsafe { &mut *heap.0 };
  let markers = unsafe { &mut *markers.0 };
  let args = unsafe { &*args.0 };
  let line = unsafe { &mut *line.0 };

  if args.len() == 3 {
    let [a, v, v2] = &args[..] else {
      panic!("Invalid syntax");
    };

    unsafe {
      let a = &**a;

      match a {
        "*listen" => {
          let function = &**v2;
          let module = &**v;

          let module = heap
            .remove(module)
            .expect("Invalid Format")
            .expect("Unable to capture Runtime");

          let BufValue::RuntimeRaw(module) = module else {
            panic!("Expected, Lead Module");
          };

          let RawRTValue::RTCM(mut module) = module.0 else {
            panic!("Expected, Lead Module only");
          };

          let BufValue::Listener(mut listen) = heap
            .remove(function)
            .expect("Unable to capture heaplistener")
            .expect("Unable to capture heaplistener")
          else {
            panic!("Invalid! Not HeapListener")
          };

          let app_ptr: &'static mut Application = unsafe { transmute(&mut *app) };

          tokio::spawn(Sendify(async move {
            let mut dummy_heap = Heap::new(app_ptr.pkg.extends.clone());
            let app = app_ptr as *mut _;

            let mut opt = Options::new();

            while let Some(event) = listen.recv().await {
              let app = unsafe {
                transmute::<&mut Application, &'static mut Application<'static>>(&mut *app)
              };
              let opt: &'static mut Options = unsafe { transmute(&mut opt) };
              let dummy_heap: &'static mut Heap = unsafe { transmute(&mut dummy_heap) };
              let module: &'static mut RTCreatedModule = unsafe { transmute(&mut module) };

              module.run_method(
                app as _,
                "on",
                "",
                move |fn_heap, _, c| {
                  if c.len() == 1 {
                    let arg0: &'static str = unsafe { transmute(&*c[0]) };

                    fn_heap.set(Cow::Borrowed(&arg0[2..]), event);
                  } else {
                    panic!("Expected, exactly 1 argument");
                  }
                },
                dummy_heap,
                opt,
              );
            }
          }));
        }
        _ => panic!("Invalid syntax"),
      }
    }

    return;
  }

  let [a, v] = &args[..] else {
    panic!("Invalid syntax");
  };

  unsafe {
    let v = &&**v;
    match &**a {
      "*run" => {
        let v = SafePtr(*v);
        let app = SafePtrMut(app);
        tokio::spawn(Sendify(async move {
          let app = app;
          let app = Wrapped(app.0);
          interpret(&v, app).await;
        }))
        .await;
      }
      "*mark" => {
        markers.insert(Cow::Borrowed(*v as &str), *line);
      }
      "*goto" => {
        *line = *markers.get(*v as &str).expect("No marker was found!");
      }
      "*prototype" => {
        let packages = app.pkg_resolver.call_mut((v, true));

        for pkg in packages {
          // SAFETY: Infaillable
          set_into_extends(pkg.extends.unwrap(), &mut heap.extends);
        }
      }
      "*import" => {
        let packages = app.pkg_resolver.call_mut((v, false));

        let mut pkg = HashMap::new();

        for package in packages {
          let RespPackage { methods, .. } = package;

          for (sig, call) in methods {
            pkg.insert(sig.to_string(), *call);
          }
        }

        let val = RawRTValue::PKG(pkg);

        set_runtime_val(heap, to_set, val);
      }
      "*mod" => {
        let LeadCode::LeadModule(code) = app.code.get(v).unwrap_or_else(|| {
          panic!("Unable to read {v}, did you mean {v}.mod.pb?");
        }) else {
          panic!("Expected LeadModule, found Lead Code");
        };

        let m = parse_into_modules(app.pkg.extends.clone(), code);

        set_runtime_val(heap, to_set, RawRTValue::RTCM(m));
      }
      a => panic!("Unknown {}", a),
    };
  }
}

pub(crate) fn parse_into_modules(
  entry: Arc<ExtendsInternal>,
  methods: &StaticLeadModule,
) -> RTCreatedModule {
  return RTCreatedModule {
    heap: Heap::new(entry),
    methods: methods.clone(),
  };
}
