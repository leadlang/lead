use std::{collections::HashMap, fs};
use crate::{types::Heap, Application};

#[derive(Debug)]
pub struct RTCreatedModule<'a> {
  pub name: &'a str,
  pub heap: Heap,
  pub methods: HashMap<&'a str, (Vec<&'a str>, String)>,
  pub drop_fn: String
}

pub fn insert_into_application(app: *mut Application, args: &Vec<String>) {
  let app = unsafe { &mut *app };

  let [a, v] = &args[..] else {
    panic!("");
  };

  match a.as_str() {
    "*mod" => {
      let code = fs::read_to_string(format!("./{v}.mod.pb")).map_or_else(|_| {
        panic!("Unable to read {}", v)
      }, |f| f);

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