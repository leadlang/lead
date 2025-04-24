use crate::metadata::Metadata;
use interpreter::{LeadCode, Structure};
use std::{collections::HashMap, fs, sync::Arc};

pub fn parse(metadata: &Metadata) -> Structure {
  let mut res = HashMap::new();

  let data = get_files(&metadata.src_dir, "");

  for (file, code) in data {
    // Lead Module Parsing
    if file.ends_with(".mod.pb") {
      let mut map = HashMap::new();

      let lines = code
        .lines()
        .filter(|x| !x.trim().is_empty())
        .map(|x| x.split(" ").collect::<Vec<_>>().leak() as &'static [_])
        .collect::<Vec<_>>()
        .leak() as &'static [_];

      let out = interpreter::parser::parse_lead_module(lines);

      _ = res.insert(file, LeadCode::LeadModule(Arc::new(map)));
    }
    // Lead Code Parsing
    else {
      _ = res.insert(
        file,
        LeadCode::Code(
          code
            .lines()
            .filter_map(|x| {
              let x = x.trim();
              if !x.starts_with("#") && !x.is_empty() {
                return Some(
                  x.split(" ").collect::<Vec<&'static str>>().leak() as &'static [&'static str]
                );
              }

              None
            })
            .collect::<Vec<&'static [&'static str]>>()
            .leak(),
        ),
      );
    }
  }

  println!("{res:#?}");

  res
}

pub fn get_files(src: &str, root: &str) -> Vec<(&'static str, &'static str)> {
  fs::read_dir(format!("{src}/{root}"))
    .expect("Unable to read dir entries")
    .filter_map(|x| {
      let x = x.ok()?;

      let file = x.file_name().into_string().ok()?;

      let metadata = x.metadata().ok()?;

      if metadata.is_dir() {
        let dir = format!("{root}/{file}");
        Some(get_files(src, &dir))
      } else {
        if !file.ends_with(".pb") {
          return None;
        }

        let path = format!("{root}/{file}").leak();

        return Some(vec![(
          &path[1..] as &'static str,
          fs::read_to_string(x.path()).ok()?.leak() as &'static str,
        )]);
      }
    })
    .flatten()
    .collect()
}
