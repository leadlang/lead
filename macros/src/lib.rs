#![feature(proc_macro_diagnostic)]
#![allow(dead_code)]

mod utils;
extern crate proc_macro;

use std::str::FromStr;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, spanned::Spanned, Ident, ItemFn};

use serde::Deserialize;
use utils::{parse_output, Either, Parsed};

#[derive(Deserialize)]
struct Documentation {
  desc: String,
  #[serde(default)]
  root: Option<String>,
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
  pub fn to_fn(self) -> (String, Vec<String>, Option<String>, Option<String>) {
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

    (a, self.params, self.returns, self.root)
  }
}

#[proc_macro_attribute]
/// Defines a callable lead function
/// 
/// ```rs
/// use interpreter::BufValue;
/// 
/// #[define((
///   desc: "Prints a variable",
///   notes: Some("Optional Note"),
///   params: [
///     // "my-custom-regex, see example below",
///     r"\$[a-z0-9_]*" // This is a regex of a lead variable
///   ],
///   returns: Some("%null"), // %null means that it returns nothing
///   usage: [
///     (
///       desc: "Prints $hello",
///       code: "print $hello"
///     )
///   ]
///   root: Some("MyNewRT"), // This is a method on a RuntimeValue
/// ))]
/// fn foo(var: &BufValue) {
///   println!("{}", var.display());
/// }
/// ```
/// 
/// ## `returns` field
/// A few command returns field values are
/// - `*`: Returns Anything / Runtime Type
/// - `%null`: Returns Nothing
/// - `@rt:name`: Returns the RuntimeValue named `name`
/// - `int`: Int
/// - `u_int`: UInt
/// - `F`: Float
/// - `S`: String
/// - `B`: boolean
/// - `Arr`: Array
/// - `Obj`: Object
/// - `Fa`: Faillable
/// - `*ptr`: Raw **Immutable** Pointer
/// - `*m ptr`: Raw **Mutable** Pointer
/// - `JoinHandle`: Async Task
/// - `Listener`: Returns Listener that can be used to listen for events
/// - `RtRAW`: Returns RAWRuntime Values (You shouldn't return a RawRTValue::PKG and should focus on RuntimeValues ONLY!)
pub fn define(args: TokenStream, input: TokenStream) -> TokenStream {
  let Either::Ok((mut doc, regex_params, ret, root)) = utils::parse_args(args.to_string().as_str()) else {
    return TokenStream::new();
  };

  let arg0_tokens = match &root {
    Some(x) => TokenStream2::from_str(&format!("me: &mut {x},")).unwrap(),
    None => TokenStream2::new()
  };

  let call0 = match &root {
    Some(_) => TokenStream2::from_str(&format!("me,")).unwrap(),
    None => TokenStream2::new()
  };

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

  let Parsed {
    call_fn,
    doc_fn,
    orig_params,
    params_to_pass,
    parse_macro,
    sig_std,
    ident,
    doc_params_decl
  } = utils::construct_parse(&sig);

  let other_tokens = parse_output(&sig.output);

  let out = &sig.output;

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

  if orig_params.is_empty() {
    doc = doc.replace("%sig%", "");
    assert!(regex_params.is_empty());

    let a = r"^(\$?)[a-z0-9_:]+";

    return quote! {
      #[allow(non_upper_case_globals)]
      #vis static #doc_fn: &'static [&'static str; 3] = &[#a, #return_type, #doc];
  
      #[allow(unused)]
      #vis fn #ident(#arg0_tokens args: *const [*const str], mut heap: interpreter::types::HeapWrapper, file: &String, opt: &mut interpreter::types::Options) {
        let _option_code_result = #call_fn(#call0 args, heap, file, opt);
        #other_tokens
      }

      #[allow(unused)]
      #vis fn #call_fn(#arg0_tokens args: *const [*const str], mut heap: interpreter::types::HeapWrapper, file: &String, opt: &mut interpreter::types::Options) #out #block
    }.into()
  }

  doc = doc.replacen("%sig%", &doc_params_decl, 1);

  let p_str = orig_params.to_string();

  if p_str.contains("str") && regex_params.is_empty() {
    orig_params.span()
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
      #vis static #doc_fn: &'static [&'static str; 3] = &[#params_on_static, #return_type, #doc];
  
      #[allow(unused)]
      #vis fn #ident(#arg0_tokens args: *const [*const str], mut heap: interpreter::types::HeapWrapper, file: &String, opt: &mut interpreter::types::Options) {
        #parse_macro

        let _option_code_result = #call_fn(#call0 #params_to_pass, file, heap, opt);
        #other_tokens
      }
  
      #[allow(unused)]
      #vis fn #call_fn(#arg0_tokens #orig_params, file: &String, mut heap: interpreter::types::HeapWrapper, opt: &mut interpreter::types::Options) #out #block
    }.into()
  }

  quote! {
    #[allow(non_upper_case_globals)]
    #vis static #doc_fn: &'static [&'static str; 3] = &[#params_on_static, #return_type, #doc];

    #[allow(unused)]
    #vis fn #ident(#arg0_tokens args: *const [*const str], mut heap: interpreter::types::HeapWrapper, file: &String, opt: &mut interpreter::types::Options) {
      #parse_macro

      let _option_code_result = #call_fn(#call0 #params_to_pass, file, opt);
      #other_tokens
    }

    #[allow(unused)]
    #vis fn #call_fn(#arg0_tokens #orig_params, file: &String, opt: &mut interpreter::types::Options) #out #block
  }.into()
}


#[proc_macro_attribute]
pub fn gendoc(args: TokenStream, input: TokenStream) -> TokenStream {
  let Either::Ok((mut doc, a_vec, b, _root)) = utils::parse_args(args.to_string().as_str()) else {
    return TokenStream::new();
  };
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
pub fn runtime_value_methods(item: TokenStream) -> TokenStream {
  let item = item.to_string();

  let methods = item.split(",").into_iter()
    .map(|x| {
      let (x, y) = x.trim().split_once("=").unwrap();
      format!("\"{}\" => {}(self, args, heap, file, opt)", x, y)
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

    fn call_ptr(
      &mut self,
      caller: &str,
      args: *const [*const str],
      heap: interpreter::types::HeapWrapper,
      file: &String,
      opt: &mut Options,
    ) -> Option<()> {{
      match caller {{
        {methods},
        _ => {{
          return None;
        }}
      }}

      Some(())
    }}
  ");

  TokenStream::from_str(&data).unwrap()
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
