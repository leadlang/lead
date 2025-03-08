#![feature(proc_macro_diagnostic)]
#![allow(dead_code)]

extern crate proc_macro;

use std::str::FromStr;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use ron::error::SpannedResult;
use syn::{parse_macro_input, spanned::Spanned, FnArg, Ident, ItemFn};

use serde::Deserialize;

#[derive(Deserialize)]
struct Documentation {
  desc: String,
  #[serde(default)]
  usage: Vec<Usage>,
  #[serde(default)]
  notes: Option<String>,
  #[serde(default)]
  params: Vec<String>,
  #[serde(default)]
  returns: Option<String>
}

#[derive(Deserialize)]
struct Usage {
  desc: String,
  code: String
}

impl Usage {
  fn r#str(self) -> String {
    format!("### {}\n\n```\n{}\n```", self.desc, self.code)
  }
}

impl Documentation {
  pub fn to_fn(self) -> (String, Vec<String>, Option<String>) {
    let fmtheader = if self.usage.is_empty() {
      ""
    } else {
      "## Format:\n\n"
    };

    let format = self.usage.into_iter().map(|x| x.r#str()).collect::<Vec<_>>().join("\n\n");
    let notes = match self.notes {
      Some(x) => format!("## Notes:\n{x}"),
      _ => format!("")
    };

    let a = format!("{}
%sig%

{fmtheader}
{format}

{notes}", self.desc);

    (a, self.params, self.returns)
  }
}

macro_rules! gen {
  ($ident:ident, $parse_mut:ident, $sig_d:ident, $sig_std:ident, $x:expr, $y:expr, $z:expr) => {
    {
      $parse_mut.push_str($x);
      $sig_d.push_str($y);
      $sig_d.push_str(&$ident);
      $sig_d.push_str(" ");

      $sig_std.push($z);
    }
  };
}

#[proc_macro_attribute]
pub fn define(args: TokenStream, input: TokenStream) -> TokenStream {
  let args = args.to_string();

  let Ok(doc): SpannedResult<Documentation> = ron::from_str(&args) else {
    TokenStream2::from_str(&args).unwrap().span()
     .unwrap()
     .error("Unable to parse documentation")
     .emit();

    return TokenStream::new();
  };

  let (mut doc, regex_params,  ret) = doc.to_fn();

  let input = parse_macro_input!(input as ItemFn);

  let ItemFn {
    // The function signature
    sig,
    // The visibility specifier of this function
    vis,
    // The function block or body
    block,
    // Other attributes applied to this function
    attrs: _,
  } = input;

  let mut parse_mut = String::from("interpreter::parse!(file + heap + args:");
  let mut params_to_pass = vec![];

  let inputs = sig.inputs;

  let mut sig_std = vec![];
  let mut sig_d = String::from("# Function Params\n\n```\n");

  inputs.iter().enumerate().for_each(|(i, s)| {
    match s {
      FnArg::Typed(s) => {
        let typ = s.ty.to_token_stream().to_string();
        let ident = s.pat.to_token_stream().to_string();

        if ["args", "heap", "file", "opt", "ret"].contains(&ident.as_str()) {
          s.span()
          .unwrap()
          .error("reserved keyword: args, heap, file, opt, ret")
          .emit()
        }

        match typ.as_str() {
          "BufValue" => gen!(ident, parse_mut, sig_d, sig_std, " -> ", "->$", "->$ *"),
          "& BufValue" => {
            parse_mut.push_str(" & ");
            sig_d.push_str("$");
            sig_d.push_str(&ident);
            sig_d.push_str(" ");

            sig_std.push("$ *");
          },
          "& mut BufValue" => {
            parse_mut.push_str(" mut ");
            sig_d.push_str("->&$");
            sig_d.push_str(&ident);
            sig_d.push_str(" ");

            sig_std.push("->&$ *");
          },
          "& str" => {
            parse_mut.push_str(" str ");
            sig_d.push_str("<");
            sig_d.push_str(&ident);
            sig_d.push_str("> ");

            sig_std.push("*");
          },
          s => s
            .span()
            .unwrap()
            .error("invalid type: Expected `BufValue`, `&BufValue` or `&mut BufValue`")
            .emit(),
        }

        parse_mut.push_str(&ident);
        params_to_pass.push(ident);
      }
      _ => {}
    }

    if i != (inputs.len() - 1) {
      parse_mut.push_str(",");
    }
  });

  sig_d.push_str("\n```");

  parse_mut.push_str(");");

  if &parse_mut == "interpreter::parse!(file + heap + args:);" {
    parse_mut = String::from("");
  }

  let parse_mut = TokenStream2::from_str(&parse_mut).unwrap();

  let new = Ident::new(&format!("_call_{}", sig.ident), sig.ident.span());

  let params = inputs.to_token_stream();

  let ident = sig.ident;

  let new_doc = Ident::new(&format!("_inner_callable_{}_doc", ident), ident.span());

  let to_pass = TokenStream2::from_str(&params_to_pass.join(",")).unwrap();

  let other_tokens = {
    let output = sig.output.to_token_stream().to_string();
    let mut ret = "";

    match output.as_str() {
      "" | "-> ()" => {}
      "-> BufValue" => {
        ret = "opt.set_return_val(_option_code_result)";
      }
      "-> Box<dyn RuntimeValue>" => {
        ret = "opt.set_r_runtime(_option_code_result)";
      }
      e => {
        sig.output.span()
          .unwrap()
          .error(format!("invalid return type: Expected `BufValue` or `(String, BufKeyVal)` or `Box<dyn RuntimeValue>`, found `{e}`"))
          .emit();
      }
    }

    TokenStream2::from_str(ret).unwrap()
  };

  let out = sig.output;

  #[allow(unused_assignments)]
  let mut return_type = r#"*"#;

  let output = out.to_token_stream().to_string();
  let output = output.trim();

  if output == "" {
    if ret.is_some() {
      out.span()
        .unwrap()
        .warning("Output is not defined, but return type is mentioned!")
        .emit();
    }
    return_type = r#""#;
  } else if let Some(x) = ret {
    return_type = x.leak();
  }

  if params.is_empty() {
    doc = doc.replace("%sig%", "");
    assert!(regex_params.is_empty());

    let a = r"^(\$?)[a-z0-9_:]+";

    return quote! {
      #[allow(non_upper_case_globals)]
      #vis static #new_doc: &'static [&'static str; 3] = &[#a, #return_type, #doc];
  
      #[allow(unused)]
      #vis fn #ident(args: *const [*const str], mut heap: interpreter::types::HeapWrapper, file: &String, opt: &mut interpreter::types::Options) {
        let _option_code_result = #new(args, heap, file, opt);
        #other_tokens
      }

      #[allow(unused)]
      #vis fn #new(args: *const [*const str], mut heap: interpreter::types::HeapWrapper, file: &String, opt: &mut interpreter::types::Options) #out #block
    }.into()
  }

  doc = doc.replacen("%sig%", &sig_d, 1);

  let p_str = params.to_string();

  if p_str.contains("str") && regex_params.is_empty() {
    params.span()
      .unwrap()
      .error("You must provide params on define! macro if you use &str or any other primitive type value")
      .emit();

    return quote! {}.into();
  }

  let mut params_on_static = String::from(r"^(\$?)[a-z0-9_:]+ ");

  if !regex_params.is_empty() {
    let l = regex_params.len();
    regex_params.into_iter().enumerate().for_each(|(i, d)| {
      params_on_static.push_str(&d);

      if i+1 < l {
        params_on_static.push(' ');
      } else {
        params_on_static.push('$');
      }
    });
  } else if !sig_std.is_empty() {
    let l = sig_std.len();
    sig_std.into_iter().enumerate().for_each(|(i, d)| {
      match d {
        "->$ *" => params_on_static.push_str(r"->\$[a-z0-9_]+"),
        "$ *" => params_on_static.push_str(r"\$[a-z0-9_]+"),
        "->&$ *" => params_on_static.push_str(r"->&\$[a-z0-9_]+"),
        "*" => params_on_static.push_str(r#"$[a-z0-9_"']+"#),
        _ => unreachable!()
      }

      if i+1 < l {
        params_on_static.push(' ');
      } else {
        params_on_static.push('$');
      }
    });
  }

  if !p_str.contains("BufValue") {
    return quote! {
      #[allow(non_upper_case_globals)]
      #vis static #new_doc: &'static [&'static str; 3] = &[#params_on_static, #return_type, #doc];
  
      #[allow(unused)]
      #vis fn #ident(args: *const [*const str], mut heap: interpreter::types::HeapWrapper, file: &String, opt: &mut interpreter::types::Options) {
        #parse_mut

        let _option_code_result = #new(#to_pass, file, heap, opt);
        #other_tokens
      }
  
      #[allow(unused)]
      #vis fn #new(#params, file: &String, mut heap: interpreter::types::HeapWrapper, opt: &mut interpreter::types::Options) #out #block
    }.into()
  }

  quote! {
    #[allow(non_upper_case_globals)]
    #vis static #new_doc: &'static [&'static str; 3] = &[#params_on_static, #return_type, #doc];

    #[allow(unused)]
    #vis fn #ident(args: *const [*const str], mut heap: interpreter::types::HeapWrapper, file: &String, opt: &mut interpreter::types::Options) {
      #parse_mut

      let _option_code_result = #new(#to_pass, file, opt);
      #other_tokens
    }

    #[allow(unused)]
    #vis fn #new(#params, file: &String, opt: &mut interpreter::types::Options) #out #block
  }.into()
}


#[proc_macro_attribute]
pub fn gendoc(args: TokenStream, input: TokenStream) -> TokenStream {
  let argv = args.to_string();

  let Ok(doc): SpannedResult<Documentation> = ron::from_str(&argv) else {
    TokenStream2::from_str(&argv).unwrap().span()
     .unwrap()
     .error("Unable to parse documentation")
     .emit();

    return TokenStream::new();
  };

  let (mut doc, a_vec, b) = doc.to_fn();
  doc = doc.replacen("%sig%", "", 1);

  let input = parse_macro_input!(input as ItemFn);

  if a_vec.len() == 0 {
    input.sig.span()
    .unwrap()
    .error("gendoc function assumes that your function takes in >= 1 arguments")
    .help("Did you add RegExp pattern to the function doc (params field)?")
    .emit();
    
    return quote! {}.into();
  };

  let mut params = String::from(r"^(\$?)[a-z0-9_:]+ ");

  let l = a_vec.len();
  a_vec.into_iter().enumerate().for_each(|(i, d)| {
    params.push_str(&d);
    
    if i+1 < l {
      params.push(' ');
    } else {
      params.push('$');
    }
  });

  let span = input.sig.span().unwrap();
  let b = b.unwrap_or_else(move || {
    span
      .note("The return type is None")
      .emit();

    format!("")
  });

  let ItemFn {
    // The function signature
    sig,
    // The visibility specifier of this function
    vis,
    // The function block or body
    block,
    // Other attributes applied to this function
    attrs: _,
  } = input;

  let s = sig.to_token_stream();
  let ident = sig.ident;

  let new_doc = Ident::new(&format!("_inner_callable_{}_doc", ident), ident.span());

  quote! {
    #[allow(non_upper_case_globals)]
    #vis static #new_doc: &'static [&'static str; 3] = &[#params, #b, #doc];
    
    #vis #s #block
  }.into()
}

#[proc_macro]
pub fn methods(item: TokenStream) -> TokenStream {
  let item = item.to_string();

  let methods = item.split(",").into_iter()
    .map(|x| {
      let (x, y) = x.trim().split_once("=").unwrap();
      format!("(\"{}\", {})", x, y)
    })
    .collect::<Vec<_>>()
    .join(",\n");

  let doc = item.split(",").into_iter()
    .map(|x| {
      let (y, x) = x.trim().split_once("=").unwrap();
      format!("\"{}\" => _inner_callable_{}_doc", y, x)
    })
    .collect::<Vec<_>>()
    .join(",\n");
  
  let data = format!("
    fn doc(&self) -> std::collections::HashMap<&'static str, &'static [&'static str; 3]> {{
      interpreter::hashmap! {{
        {doc}
      }}
    }}

    fn methods(&self) -> interpreter::types::MethodRes {{
      &[
        {methods}
      ]
    }}
  ");

  TokenStream::from_str(&data).unwrap()
}