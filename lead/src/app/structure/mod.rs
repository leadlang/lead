use crate::metadata::Metadata;
use interpreter::{LeadCode, Structure};
use std::{collections::HashMap, fs, sync::Arc};

pub fn parse(metadata: &Metadata) -> Structure {
  let mut res = HashMap::new();

  let entry: &str = &metadata.entry_file;
  let data = get_files(&metadata.src_dir, "");

  for (file, code) in data {
    // Lead Module Parsing
    if file.ends_with(".mod.pb") {
      let lines = code
        .lines()
        .filter_map(|x| {
          let trimmed = x.trim();
          
          if trimmed.is_empty() {
            None
          } else {
            Some(trimmed)
          }
        })
        .map(|x| x.split(" ").collect::<Vec<_>>().leak() as &'static [_])
        .collect::<Vec<_>>()
        .leak() as &'static [_];

      let out = interpreter::parser::parse_lead_module(lines).expect("Unable to parse Lead Module");

      _ = res.insert(file, LeadCode::LeadModule(Arc::new(out.data)));
    }
    // Lead Code Parsing
    else {
      _ = res.insert(
        if file == entry {
          ":entry"
        } else {
          file
        },
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
