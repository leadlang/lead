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
  usage: Vec<Usage>,
  notes: Option<String>
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
  pub fn to_fn(self) -> String {
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

    format!("{}
%sig%

{fmtheader}
{format}

{notes}", self.desc)
  }
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

  let mut doc: String = doc.to_fn();

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
          "BufValue" => {
            parse_mut.push_str(" -> ");
            sig_d.push_str("->$");
            sig_d.push_str(&ident);
            sig_d.push_str(" ");
          },
          "& BufValue" => {
            parse_mut.push_str(" & ");
            sig_d.push_str("$");
            sig_d.push_str(&ident);
            sig_d.push_str(" ");
          },
          "& mut BufValue" => {
            parse_mut.push_str(" mut ");
            sig_d.push_str("->&$");
            sig_d.push_str(&ident);
            sig_d.push_str(" ");
          },
          "& str" => {
            parse_mut.push_str(" str ");
            sig_d.push_str("<");
            sig_d.push_str(&ident);
            sig_d.push_str("> ");
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
        ret = "_opt.set_return_val(__option_code_result)";
      }
      "-> (String, BufKeyVal)" | "-> (String,BufKeyVal)" => {
        ret = "let (_a, _b) = __option_code_result;\n_opt.set_return_ptr(_a, _b)";
      }
      "-> RuntimeValue" => {
        ret = "_opt.set_r_runtime(__option_code_result)";
      }
      e => {
        sig.output.span()
          .unwrap()
          .error(format!("invalid return type: Expected `BufValue` or `(String, BufKeyVal)` or `RuntimeValue`, found `{e}`"))
          .emit();
      }
    }

    TokenStream2::from_str(ret).unwrap()
  };

  let out = sig.output;

  if params.is_empty() {
    doc = doc.replace("%sig%", "");
    return quote! {
      #[allow(non_upper_case_globals)]
      #vis static #new_doc: &'static str = #doc;
  
      #[allow(unused)]
      #vis fn #ident(args: &Vec<*const str>, mut heap: interpreter::types::HeapWrapper, file: &String, _opt: &mut interpreter::types::Options) {
        let __option_code_result = #new(args, heap, file, _opt);
        #other_tokens
      }

      #[allow(unused)]
      #vis fn #new(args: &Vec<*const str>, mut heap: interpreter::types::HeapWrapper, file: &String, opt: &mut interpreter::types::Options) #out #block
    }.into()
  }

  doc = doc.replacen("%sig%", &sig_d, 1);

  if !params.to_string().contains("BufValue") {
    return quote! {
      #[allow(non_upper_case_globals)]
      #vis static #new_doc: &'static str = #doc;
  
      #[allow(unused)]
      #vis fn #ident(args: &Vec<*const str>, mut heap: interpreter::types::HeapWrapper, file: &String, _opt: &mut interpreter::types::Options) {
        #parse_mut

        let __option_code_result = #new(#to_pass, file, heap);
        #other_tokens
      }
  
      #[allow(unused)]
      #vis fn #new(#params, file: &String, mut heap: interpreter::types::HeapWrapper) #out #block
    }.into()
  }

  quote! {
    #[allow(non_upper_case_globals)]
    #vis static #new_doc: &'static str = #doc;

    #[allow(unused)]
    #vis fn #ident(args: &Vec<*const str>, mut heap: interpreter::types::HeapWrapper, file: &String, _opt: &mut interpreter::types::Options) {
      #parse_mut

      let __option_code_result = #new(#to_pass, file);
      #other_tokens
    }

    #[allow(unused)]
    #vis fn #new(#params, file: &String) #out #block
  }.into()
}


#[proc_macro_attribute]
pub fn gendoc(args: TokenStream, input: TokenStream) -> TokenStream {
  let args = args.to_string();

  let Ok(doc): SpannedResult<Documentation> = ron::from_str(&args) else {
    TokenStream2::from_str(&args).unwrap().span()
     .unwrap()
     .error("Unable to parse documentation")
     .emit();

    return TokenStream::new();
  };

  let doc = doc.to_fn();
  let doc = doc.replacen("%sig%", "", 1);

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

  let s = sig.to_token_stream();
  let ident = sig.ident;

  let new_doc = Ident::new(&format!("_inner_callable_{}_doc", ident), ident.span());

  quote! {
    #[allow(non_upper_case_globals)]
    #vis static #new_doc: &'static str = #doc;
    
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
    fn doc(&self) -> std::collections::HashMap<&'static str, &'static str> {{
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