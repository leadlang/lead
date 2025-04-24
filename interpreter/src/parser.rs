use std::collections::HashMap;

use crate::error;

#[derive(Debug)]
pub struct ParsedTokenTree<'a> {
  pub name: &'a str,
  pub data: HashMap<&'a str, (&'a [&'a str], &'a [&'a [&'a str]])>
}

pub fn parse_lead_module<'a>(lines: &'a [&'a [&'a str]]) -> Option<ParsedTokenTree<'a>> {
  let mut name = "";
  let mut data = HashMap::new();

  let mut ctx = "";

  let mut start = 0;

  for (index, tokens) in lines.iter().enumerate() {
    let caller = tokens[0];

    if ctx != "" {
      match caller {
        a => error(format!("{a}"), "")
      }
    }
  }
  
  Some(
    ParsedTokenTree {
      name,
      data
    }
  )
}