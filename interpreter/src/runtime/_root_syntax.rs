use std::collections::HashMap;
use crate::{error, ipreter::{interpret, tok_parse}, types::{set_runtime_val, Heap, RawRTValue}, Application, RespPackage};

#[derive(Debug)]
pub struct RTCreatedModule<'a> {
  pub(crate) name: &'a str,
  pub(crate) heap: Heap,
  pub(crate) methods: HashMap<&'a str, (Vec<&'a str>, String)>,
  pub(crate) drop_fn: String
}

impl<'a> RTCreatedModule<'a> {
  pub(crate) fn run_method<T: FnOnce(&mut Heap, &mut Heap, &Vec<&str>) -> ()>(&mut self, app: *mut Application, method: &str, file: &str, into_heap: T) {
    let app = unsafe { &mut *app };

    let (args, method_code) = self.methods.get(&method).unwrap_or_else(|| error("Unable to find :method", file));
    into_heap(&mut self.heap, &mut app.heap, args);

    // run
    let file_name = ":fn";
    
    let file = method_code.replace("\r", "");
    let file = file.split("\n").collect::<Vec<_>>();

    let mut line = 0usize;
  
    while line < file.len() {
      let content = &file[line];
  
      if !content.starts_with("#") {
        tok_parse(format!("{}:{}", &file_name, line), content, app, &mut self.heap, &mut line);
      }
  
      line += 1;
    }

    self.heap.clear();
  }
}

pub fn insert_into_application(app: *mut Application, args: &Vec<String>, line: &mut usize, to_set: String) {
  let app = unsafe { &mut *app };

  let [a, v] = &args[..] else {
    panic!("Invalid syntax");
  };

  match a.as_str() {
    "*run" => {
      interpret(&v, app);
    }
    "*mark" => {
      app.markers.insert(v.into(), *line);
    }
    "*goto" => {
      *line = *app.markers.get(v).expect("No marker was found!");
    }
    "*import" => {
      let RespPackage { name, methods, dyn_methods } = app.pkg_resolver.call_mut((v.as_str(),));

      let mut pkg = HashMap::new();

      for (sig, call) in methods {
        pkg.insert(sig.to_string(), *call);
      }
      for (sig, call) in dyn_methods {
        pkg.insert(sig.to_string(), call);
      }

      let val = RawRTValue::PKG(pkg);

      set_runtime_val(&mut app.heap, to_set, {
        let name = String::from_utf8_lossy(name);
        let name: &'static mut str = name.to_string().leak::<'static>();

        name
      }, val);
    }
    "*mod" => {
      let code = String::from_utf8(app.module_resolver.call_mut((format!("./{v}.mod.pb").as_str(),))).unwrap_or_else(|_| {
        panic!("Unable to read {v}.mod.pb");
      });

      for m in parse_into_modules(code) {
        let None = app.modules.insert(m.name.into(), m) else {
          panic!("Duplicate Module");
        };
      }
    }
    a => panic!("Unknown {}", a)
  };
}

pub fn parse_into_modules<'a>(code: String) -> Vec<RTCreatedModule<'a>> {
  let mut data = vec![];

  let code = code.leak();
  let split = code.split("\n");
  let split = split.map(|x| x.trim()).filter(|x| x != &"" && !x.starts_with("#")).collect::<Vec<_>>();

  let mut mod_id = 0;

  let mut ctx = "";

  let mut tok_arg: Vec<&str> = vec![];
  let mut tok_ens: String = String::new();

  let mut in_ctx = false;

  for tokens in split {
    let mut tok = tokens.split(" ").collect::<Vec<_>>();

    if !in_ctx {
      let caller = tok.remove(0);

      match caller {
        "__declare_global" => {
          mod_id = data.len();
          data.push(RTCreatedModule {
            drop_fn: "".into(),
            heap: Heap::new(),
            methods: HashMap::new(),
            name: tok.remove(0)
          });
        }
        "_fn" => {
          ctx = tok.remove(0);
          in_ctx = true;
          
          for t in &tok {
            if (!t.starts_with("->")) || (t.starts_with("->&")) {
              error(format!("Arguments of module parameters can ONLY be move! {t} is not move!"), ":core:parser");
            }
          }
          tok_arg = tok.clone();
        }
        "_drop" => {
          ctx = "drop";
          in_ctx = true;
        }
        "__end" => {

        }
        a => panic!("Unknown NON-CONTEXT {a}")
      };
    } else {
      if tok[0] == "_end" {
        in_ctx = false;
        
        let module: &mut RTCreatedModule<'a> = data.get_mut(mod_id).unwrap();

        if ctx == "drop" {
          module.drop_fn = tok_ens.clone();
        } else {
          let None = module.methods.insert(ctx.into(),(tok_arg.clone(), tok_ens.clone())) else {
            panic!("Method overlap");
          };
        }

        tok_arg = vec![];
        tok_ens = String::new();
      } else {
        tok_ens.push_str(tokens);
        tok_ens.push_str("\n");
      }
    }
  }

  data
}