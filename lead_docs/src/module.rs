use std::{collections::HashMap, env, fs, path::{Path, PathBuf}};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LeadModule {
  pub name: String,
  pub own: String,
  pub methods: Vec<Method>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Method {
  pub name: String,
  pub desc: String,
}

fn get_file(own: &str, file: &str, is_base: bool) -> PathBuf {
  let mut lead_home = PathBuf::from(env::var("LEAD_HOME").unwrap());

  let mut file: String = file.into();

  if is_base {
    lead_home.push("docs");
    lead_home.push(own);

    file = file.replace("./docs", "./");
  }

  if file.ends_with(".md") {
    lead_home.push(file);
  } else {
    lead_home.push(format!("{}.md", &file));
  }

  lead_home
}

impl LeadModule {
  pub fn new(own: &str, refs: Vec<&str>, pkgs: Vec<&str>, base: bool) -> Vec<Self> {
    let mut lib_map: HashMap<&str, LeadModule> = HashMap::new();

    for pkg in pkgs {
      let [m_id, file] = pkg.split("->").collect::<Vec<_>>()[..] else {
        panic!("");
      };

      let file = get_file(own, file, base);
      println!("{}", file.display());
      let doc = fs::read_to_string(file);
      println!("{:?}", doc);
    }

    for method in refs {
      let [key, module, doc_file] = method.split("->").collect::<Vec<_>>()[..] else {
        panic!("");
      };
    }

    vec![]
  }
}