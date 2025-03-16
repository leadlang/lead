use std::str::FromStr;

use proc_macro2::TokenStream as TokenStream2;
use ron::{error::SpannedResult, from_str};
use quote::ToTokens;
use syn::{spanned::Spanned, FnArg, Ident, ReturnType, Signature};

use crate::Documentation;

pub enum Either<T> {
  Ok(T),
  NotOk
}

pub fn parse_args(args: &str) -> Either<(String, Vec<String>, Option<String>, Option<String>)> {
  let Ok(data): SpannedResult<Documentation> = from_str(args) else {
    TokenStream2::from_str(args)
      .unwrap()
      .span()
      .unwrap()
      .error("Unable to parse documentation")
      .emit();

    return Either::NotOk;
  };

  let (doc, regex, ret, root) = data.to_fn();
  Either::Ok((doc, regex, ret, root))
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

pub struct Parsed<'a> {
  pub call_fn: Ident,
  pub parse_macro: TokenStream2,
  pub orig_params: TokenStream2,
  pub doc_fn: Ident,
  pub params_to_pass: TokenStream2,
  pub sig_std: Vec<&'static str>,
  pub ident: &'a Ident,
  pub doc_params_decl: String
}

pub fn construct_parse<'a>(sig: &'a Signature) -> Parsed<'a> {
  let mut parse_mut = String::from("interpreter::parse!(file + heap + args:");
  let mut params_to_pass = vec![];
  
  let mut sig_std = vec![];
  let mut sig_d = String::from("# Function Params\n\n```\n");

  sig.inputs.iter().enumerate().for_each(|(i, s)| {
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

    if i != (sig.inputs.len() - 1) {
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

  let params = sig.inputs.to_token_stream();

  let ident = &sig.ident;

  let new_doc = Ident::new(&format!("_inner_callable_{}_doc", ident), ident.span());

  let to_pass = TokenStream2::from_str(&params_to_pass.join(",")).unwrap();

  Parsed {
    call_fn: new,
    parse_macro: parse_mut,
    orig_params: params,
    doc_fn: new_doc,
    params_to_pass: to_pass,
    sig_std,
    ident,
    doc_params_decl: sig_d
  }
}

pub fn parse_output(output: &ReturnType) -> TokenStream2 {
  let output = output.to_token_stream().to_string();
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
      output.span()
        .unwrap()
        .error(format!("invalid return type: Expected `BufValue` or `(String, BufKeyVal)` or `Box<dyn RuntimeValue>`, found `{e}`"))
        .emit();
    }
  }

  TokenStream2::from_str(ret).unwrap()
}