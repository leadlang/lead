use crate::{
  error,
  ipreter::{interpret, tok_parse},
  types::{
    set_into_extends, set_runtime_val, BufValue, Heap, Options, RawRTValue,
  },
  Application, ExtendsInternal, RespPackage,
};
use std::{
  borrow::Cow,
  collections::HashMap,
  mem::{take, transmute},
  sync::Arc,
  thread,
};

#[derive(Debug)]
pub(crate) struct RTCreatedModule {
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
    let app = unsafe { &mut *app };
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

          thread::spawn(move || {
            let mut dummy_heap = Heap::new(app_ptr.pkg.extends.clone());
            let app = app_ptr as *mut _;

            let mut opt = Options::new();

            while let Ok(event) = listen.recv() {
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
          });
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
        interpret(&v, app);
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

        set_runtime_val(heap, to_set, "loaded", val);
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

        let Some(m) = parse_into_modules(app.pkg.extends.clone(), code) else {
          panic!("No RTC Module found in the module file");
        };

        set_runtime_val(heap, to_set, m.name, RawRTValue::RTCM(m));
      }
      a => panic!("Unknown {}", a),
    };
  }
}

pub(crate) fn parse_into_modules(
  entry: Arc<ExtendsInternal>,
  code: String,
) -> Option<RTCreatedModule> {
  let mut data = RTCreatedModule {
    code,
    lines: vec![],
    heap: Heap::new(entry),
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

  let mut start: usize = 0;

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
          start = id + 1;

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

        if start == usize::MAX {
          panic!("Something is wrong!");
        }

        let lines: &'static [&'static str] =
          unsafe { transmute(&data.lines[..] as &[&'static str]) };

        let begin = start as usize;

        let None = data
          .methods
          .insert(ctx, (std::mem::take(&mut tok_arg), &lines[begin..id]))
        else {
          panic!("Method overlap");
        };

        start = usize::MAX;
      }
    }
  }

  if mod_id == 0 {
    None
  } else {
    Some(data)
  }
}
