use tokio::task::yield_now;

use crate::{
  error,
  ipreter::{interpret, tok_parse},
  types::{set_runtime_val, BufValue, Heap, Options, RawRTValue, make_unsafe_send_future},
  Application, RespPackage,
};
use std::{
  borrow::Cow,
  collections::HashMap,
  mem::{take, transmute},
};

#[derive(Debug)]
pub struct RTCreatedModule {
  pub(crate) code: String,
  pub(crate) lines: Vec<&'static str>,
  pub(crate) name: &'static str,
  pub(crate) heap: Heap,
  pub(crate) methods: HashMap<&'static str, (Vec<&'static str>, &'static [&'static str])>,
}

impl RTCreatedModule {
  pub(crate) fn run_method<T: FnOnce(&mut Heap, &mut Heap, &Vec<&str>) -> ()>(
    &mut self,
    app: *mut Application,
    method: &str,
    file: &str,
    into_heap: T,
    heap: &mut Heap,
    opt: &mut Options,
  ) {
    let mut temp_heap = Heap::new_with_this(&mut self.heap);
    let app = unsafe { &mut *app };

    let (args, method_code) = self
      .methods
      .get(&method)
      .unwrap_or_else(|| error("Unable to find :method", file));
    into_heap(&mut temp_heap, heap, args);

    // run
    let file_name = ":fn";

    let file = method_code;

    let mut line = 0usize;

    let mut markers = HashMap::new();

    while line < file.len() {
      let content = file[line];

      if !content.starts_with("#") {
        unsafe {
          tok_parse(
            format!("{}:{}", &file_name, line),
            content,
            app,
            &mut temp_heap,
            &mut line,
            &mut markers,
            false,
            Some(opt),
          );
        }
      }

      line += 1;
    }

    drop(temp_heap);
  }

  pub(crate) async fn run_method_async<'a, T: FnOnce(&mut Heap, &mut Heap, &Vec<&str>) -> ()>(
    &mut self,
    app: *mut Application<'a>,
    method: &str,
    file: &str,
    into_heap: T,
    heap: &mut Heap,
    opt: &mut Options,
  ) {
    let mut temp_heap = Heap::new_with_this(&mut self.heap);
    let app = unsafe { &mut *app };

    let (args, method_code) = self
      .methods
      .get(&method)
      .unwrap_or_else(|| error("Unable to find :method", file));
    into_heap(&mut temp_heap, heap, args);

    // run
    let file_name = ":fn";

    let file = method_code;

    let mut line = 0usize;

    let mut markers = HashMap::new();

    while line < file.len() {
      let content = file[line];

      if !content.starts_with("#") {
        unsafe {
          if let Some(x) = tok_parse(
            format!("{}:{}", &file_name, line),
            content,
            app,
            &mut temp_heap,
            &mut line,
            &mut markers,
            true,
            Some(opt),
          ) {
            x.await;
          }
        }

        yield_now().await;
      }

      line += 1;
    }

    drop(temp_heap);
  }
}

#[allow(unused)]
pub fn insert_into_application(
  app: *mut Application,
  args: &[*const str],
  line: &mut usize,
  to_set: Cow<'static, str>,
  heap: &mut Heap,
  markers: &mut HashMap<Cow<'static, str>, usize>,
) {
  let app = unsafe { &mut *app };

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

          let BufValue::RuntimeRaw(name, module) = module else {
            panic!("Expected, Lead Module");
          };

          let RawRTValue::RTCM(mut module) = module.0 else {
            panic!("Expected, Lead Module, not {name}");
          };

          let BufValue::Listener(mut listen) = heap
            .remove(function)
            .expect("Unable to capture heaplistener")
            .expect("Unable to capture heaplistener")
          else {
            panic!("Invalid! Not HeapListener")
          };

          let app_ptr: &'static mut Application = unsafe { transmute(&mut *app) };

          let future = async move {
            let mut dummy_heap = Heap::new();
            let app = app_ptr as *mut _;

            while let Some(event) = listen.recv().await {
              module
                .run_method_async(
                  app,
                  "method",
                  "",
                  move |fn_heap, _, c| {
                    if c.len() == 1 {
                      let arg0: &'static str = unsafe { transmute(&*c[0]) };

                      fn_heap.set(Cow::Borrowed(arg0), event);
                    } else {
                      panic!("Expected, exactly 1 argument");
                    }
                  },
                  &mut dummy_heap,
                  &mut Options::new(&unsafe { &*app }.runtime as *const _),
                )
                .await;
            }
          };

          app.runtime.spawn(make_unsafe_send_future(future));
        }
        _ => panic!("Invalid syntax"),
      }
    }
  }

  let [a, v] = &args[..] else {
    panic!("Invalid syntax");
  };

  unsafe {
    let v = &&**v;
    match &**a {
      "*run" => {
        interpret(&v, app);
      }
      "*mark" => {
        markers.insert(Cow::Borrowed(*v as &str), *line);
      }
      "*goto" => {
        *line = *markers.get(*v as &str).expect("No marker was found!");
      }
      "*import" => {
        let RespPackage {
          name,
          methods,
          dyn_methods,
        } = app.pkg_resolver.call_mut((v,));

        let mut pkg = HashMap::new();

        for (sig, call) in methods {
          pkg.insert(sig.to_string(), *call);
        }
        for (sig, call) in dyn_methods {
          pkg.insert(sig.to_string(), call);
        }

        let val = RawRTValue::PKG(pkg);

        set_runtime_val(
          heap,
          to_set,
          {
            let name = String::from_utf8_lossy(name);
            let name: &'static mut str = name.to_string().leak::<'static>();

            name
          },
          val,
        );
      }
      "*mod" => {
        let code = String::from_utf8(
          app
            .module_resolver
            .call_mut((format!("./{v}.mod.pb").as_str(),)),
        )
        .unwrap_or_else(|_| {
          panic!("Unable to read {v}.mod.pb");
        });

        let Some(m) = parse_into_modules(code) else {
          panic!("No RTC Module found in the module file");
        };

        set_runtime_val(heap, to_set, m.name, RawRTValue::RTCM(m));
      }
      a => panic!("Unknown {}", a),
    };
  }
}

pub fn parse_into_modules(code: String) -> Option<RTCreatedModule> {
  let mut data = RTCreatedModule {
    code,
    lines: vec![],
    heap: Heap::new(),
    methods: HashMap::new(),
    name: "%none",
  };

  let split = data.code.split("\n");
  let split = split
    .map(|x| unsafe { transmute::<&str, &'static str>(x.trim()) })
    .filter(|x| x != &"" && !x.starts_with("#"))
    .collect::<Vec<_>>();

  data.lines = split;

  let mut mod_id: u8 = 0;

  let mut ctx = "";

  let mut tok_arg: Vec<&str> = vec![];

  let mut start: i64 = -1;

  let mut in_ctx = false;

  for (id, tokens) in data.lines.iter().enumerate() {
    let mut tok = tokens.split(" ").collect::<Vec<_>>();

    if !in_ctx {
      let caller = tok.remove(0);

      match caller {
        "declare" => {
          if mod_id != 0 {
            panic!("More than 1 module found in a single lead module file");
          }

          mod_id += 1;
          data.name = tok.remove(0);
        }
        "fn" => {
          ctx = tok.remove(0);
          in_ctx = true;

          for t in &tok {
            if (!t.starts_with("->")) || (t.starts_with("->&")) {
              error(
                format!("Arguments of module parameters can ONLY be move! {t} is not move!"),
                ":core:parser",
              );
            }
          }
          tok_arg = take(&mut tok);
        }
        a => panic!("Unknown NON-CONTEXT {a}"),
      };
    } else {
      if tok[0] == "*end" {
        in_ctx = false;

        let lines: &'static [&'static str] =
          unsafe { transmute(&data.lines[..] as &[&'static str]) };

        let begin = start as usize;

        let None = data
          .methods
          .insert(ctx, (std::mem::take(&mut tok_arg), &lines[begin..id]))
        else {
          panic!("Method overlap");
        };

        start = -1;
      } else {
        if start == -1 {
          start = id as i64;
        }
      }
    }
  }

  if mod_id == 0 {
    None
  } else {
    Some(data)
  }
}
