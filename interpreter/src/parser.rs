use std::collections::HashMap;

use crate::error;

#[derive(Debug)]
pub struct ParsedTokenTree<'a> {
  pub name: &'a str,
  pub data: HashMap<&'a str, (&'a [&'a str], &'a [&'a [&'a str]])>,
}

pub fn parse_lead_module<'a>(lines: &'a [&'a [&'a str]]) -> Option<ParsedTokenTree<'a>> {
  let mut name = "";
  let mut data = HashMap::new();

  let mut ctx = "";
  let mut args: &'a [&'a str] = &[];

  let mut start = usize::MAX;

  for (index, tokens) in lines.iter().enumerate() {
    let caller = tokens[0];

    // Check for declarators if there's no `ctx`
    if ctx == "" {
      match caller {
        "declare" => {
          if name != "" {
            error(
              "Lead Language Module cannot have more than 1 module declaration",
              ":fn",
            );
          }

          name = tokens[1];
        }
        "fn" => {
          ctx = tokens[1];
          start = index + 1;

          for t in &tokens[2..] {
            if (!t.starts_with("->")) || (t.starts_with("->&")) {
              error(
                format!(
                  "Arguments of module parameters can ONLY be of `move` type! {t} is not move!"
                ),
                ":fn",
              );
            }
          }
          args = &tokens[2..];
        }
        a => error(format!("Unknown `{a}`. No context was detected"), ":fn"),
      }
    } else {
      if caller == "*end" {
        if start == usize::MAX {
          error("Something's not correct!!", ":fn");
        }

        // Upto the `*end` but not including `*end`
        let code = &lines[start..index];

        start = usize::MAX;

        data.insert(ctx, (args, code));
        ctx = "";
      }
    }
  }

  Some(ParsedTokenTree { name, data })
}
