#![feature(proc_macro_diagnostic)]
#![allow(dead_code)]

//! <div class="warning">
//! A few rules:
//! 
//! - The variables are only valid till the function execution.
//! - Storage of LeadLang variable data **(except the moved ones)** requires cloning.
//! - You shouldn't store the **mutable or immutable** reference of a variable.
//! 
//! </div>
//! 
//! Got to [define](./attr.define.html) function
//! 
//! ## Simple LeadLang Package
//! ```rust
//! use interpreter::{generate, module, pkg_name};
//! use lead_lang_macros::{define, methods};
//! 
//! use interpreter::BufValue;
//! 
//! module! {
//!   Module1,
//!   pkg_name! { "Module" },
//!   methods! {
//!     foo
//!   }
//! }
//! 
//! generate! {
//!   Module1
//! }
//! 
//! #[define((
//!   desc: "Prints a variable",
//!   notes: Some("Optional Note"),
//!   params: [
//!    // "my-custom-regex, see example below",
//!    r"\$[a-z0-9_]*" // This is a regex of a lead variable
//!   ],
//!   returns: Some("%null"), // %null means that it returns nothing
//!   usage: [
//!    (
//!     desc: "Prints $hello",
//!     code: "print $hello"
//!    )
//!   ]
//!   root: Some("MyNewRT"), // This is a method on a RuntimeValue
//! ))]
//! fn foo(var: &BufValue) {
//!   println!("{}", var.display());
//! }
//! ```

mod utils;
extern crate proc_macro;

use std::str::FromStr;
use proc_macro2::{Span, TokenStream as TokenStream2};
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
/// <div class="warning">
///   Please read the full documentation
/// </div>
/// 
/// ```rust
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
/// ## Creating a prototype function
/// You just need to update the `root` variable
/// 
/// These special values are :-
/// | Value | Type |
/// |-------|------|
/// | `int` |  [`i64`] |
/// | `uint`| [`u64`] |
/// | `float`| [`f64`] |
/// | `str_slice`| [`String`] |
/// | `boolean`| [`bool`] |
/// | `array`| [`Vec<interpreter::types::BufValue>`] |
/// | `object`| [`std::collections::HashMap<String, Box<interpreter::types::BufValue>>`] |
/// | `faillable` | [`Result<Box<interpreter::types::BufValue>, String>`] |
/// | `str_ptr` | [`interpreter::types::StrPointer`] |
/// | `ptr` | [`*const`] [`interpreter::types::BufValue`] |
/// | `mut_ptr` | [`*mut`] [`interpreter::types::BufValue`] |
/// | `async_task` | [`interpreter::types::AppliesEq<JoinHandle<BufValue>>`] |
/// | `sender` | [`interpreter::types::AppliesEq<UnboundedSender<BufValue>>`] |
/// | `listener` | [`interpreter::types::AppliesEq<UnboundedReceiver<BufValue>>`] |
/// | `arc_mut` | [`std::sync::Arc<std::sync::Mutex<Box<interpreter::types::BufValue>>>`] |
/// | `arc_mut_ptr` | [`interpreter::types::AppliesEq<std::sync::Arc<std::sync::Mutex<Box<interpreter::types::BufValue>>>>`] |
/// 
/// ## `returns` field
/// The standardized values are :-
/// | Value| Description |
/// |------|-------------|
/// | `%null` | Returns Nothing |
/// | `@rt:name` | Returns the RuntimeValue named `name` |
/// | `int` | Int |
/// | `u_int` | UInt |
/// | `F` | Float |
/// | `S` | String |
/// | `B` | boolean |
/// | `Arr` | Array |
/// | `Obj` | Object |
/// | `Fa` | Faillable |
/// | `*ptr` | Raw **Immutable** Pointer |
/// | `*m ptr` | Raw **Mutable** Pointer |
/// | `JoinHandle` | Async Task |
/// | `Listener` | Returns Listener that can be used to listen for events |
/// | `RtRAW` | Returns RAWRuntime Values (You shouldn't return a RawRTValue::PKG and should focus on RuntimeValues ONLY!) |
pub fn define(args: TokenStream, input: TokenStream) -> TokenStream {
  let Either::Ok((mut doc, regex_params, ret, root)) = utils::parse_args(args.to_string().as_str()) else {
    return TokenStream::new();
  };

  let tok = match &root {
    Some(x) => match x.as_str() {
      "int" => { format!("me: *mut i64,") }
      "uint" => { format!("me: *mut u64,") }
      "float" => { format!("me: *mut f64,") }
      "str_slice" => { format!("me: *mut String,") }
      "boolean" => { format!("me: *mut bool,") }
      "array" => { format!("me: *mut Vec<interpreter::types::BufValue>,") }
      "object" => { format!("me: *mut std::collections::HashMap<String, Box<interpreter::types::BufValue>>,") }
      "faillable" => { format!("me: *mut Result<Box<interpreter::types::BufValue>, String>,") }
      "str_ptr" => { format!("me: *mut interpreter::types::StrPointer,") }
      "ptr" => { format!("me: *mut *const interpreter::types::BufValue,") }
      "mut_ptr" => { format!("me: *mut *mut interpreter::types::BufValue,") }
      "async_task" => { format!("me: *mut interpreter::types::AppliesEq<interpreter::JoinHandle<interpreter::types::BufValue>>,") }
      "sender" => { format!("me: *mut interpreter::types::AppliesEq<interpreter::types::UnboundedSender<interpreter::types::BufValue>>,") }
      "listener" => { format!("me: *mut interpreter::types::AppliesEq<interpreter::types::UnboundedReceiver<interpreter::types::BufValue>>,") }
      "arc_ptr" => format!("me: *mut std::sync::Arc<Box<interpreter::types::BufValue>>,"),
      "arc_mut_ptr" => format!("me: *mut interpreter::types::AppliesEq<std::sync::Arc<std::sync::Mutex<Box<interpreter::types::BufValue>>>>,"),
      e => { format!("me: &mut {e},") }
    }
    _ => {
      format!("")
    }
  };
  let arg0_tokens = TokenStream2::from_str(&tok).unwrap();

  let arg0_tokens_parsed = TokenStream2::from_str(&tok.replacen("*mut", "&mut", 1)).unwrap();
  let ext0 = if tok.contains("*mut") {
      quote! {
        let me: &mut _ = unsafe { &mut *me };
      }
    } else {
      quote! {

      }
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
      #vis fn #ident(#arg0_tokens args: *const [&'static str], mut heap: interpreter::types::HeapWrapper, file: &str, opt: &mut interpreter::types::Options) {        
        #ext0

        let _option_code_result = #call_fn(#call0 args, heap, file, opt);
        #other_tokens
      }

      #[allow(unused)]
      #vis fn #call_fn(#arg0_tokens_parsed args: *const [&'static str], mut heap: interpreter::types::HeapWrapper, file: &str, opt: &mut interpreter::types::Options) #out #block
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
      #vis fn #ident(#arg0_tokens args: *const [&'static str], mut heap: interpreter::types::HeapWrapper, file: &str, opt: &mut interpreter::types::Options) {
        #parse_macro

        let _option_code_result = #call_fn(#call0 #params_to_pass, file, heap, opt);
        #other_tokens
      }
  
      #[allow(unused)]
      #vis fn #call_fn(#arg0_tokens #orig_params, file: &str, mut heap: interpreter::types::HeapWrapper, opt: &mut interpreter::types::Options) #out #block
    }.into()
  }

  quote! {
    #[allow(non_upper_case_globals)]
    #vis static #doc_fn: &'static [&'static str; 3] = &[#params_on_static, #return_type, #doc];

    #[allow(unused)]
    #vis fn #ident(#arg0_tokens args: *const [&'static str], mut heap: interpreter::types::HeapWrapper, file: &str, opt: &mut interpreter::types::Options) {
      #parse_macro

      let _option_code_result = #call_fn(#call0 #params_to_pass, file, opt);
      #other_tokens
    }

    #[allow(unused)]
    #vis fn #call_fn(#arg0_tokens #orig_params, file: &str, opt: &mut interpreter::types::Options) #out #block
  }.into()
}


#[proc_macro_attribute]
/// <div class="warning">
/// This is not recommended to be used
/// </div>
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
/// Define the Methods of the RuntimeValue
/// 
/// ```rust
/// use interpreter::{runtime_value, rtval_name};
/// use lead_lang_macros::methods;
/// 
/// runtime_value! {
///   MyModule,
///   { },
///   rtval_name! { "📦 MyModule" }
///   methods! {
///     mod::load=myfn
///   }
/// }
/// ```
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
      args: *const [&'static str],
      heap: interpreter::types::HeapWrapper,
      file: &str,
      opt: &mut interpreter::types::Options,
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
/// Define the Module Methods
/// 
/// ```rust
/// use interpreter::{module, pkg_name};
/// use lead_lang_macros::methods;
/// 
/// module! {
///   MyModule,
///   pkg_name! { "📦 MyModule" }
///   methods! {
///     mod::load=myfn
///   }
/// }
/// ```
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


#[proc_macro]
/// Define the Package's prototypes
/// 
/// ```rust
/// use interpreter::{module, pkg_name};
/// use lead_lang_macros::define_prototypes;
/// 
/// module! {
///   MyModule,
///   pkg_name! { "📦 MyModule" }
///   define_prototypes! {
///     int: {
///       fnname=function,
///     };
///   }
/// }
/// ```
pub fn define_prototypes(item: TokenStream) -> TokenStream {
  let valid = [
    "int", "uint", "float", "str_slice", "boolean", "array", "object", "faillable", "str_ptr", "ptr", "mut_ptr", "async_task", "sender", "listener", "arc_ptr", "arc_mut_ptr"
  ];

  let item = item.to_string();

  let mut extnds = vec![];
  let mut proto = vec![];

  item.split(";").into_iter()
    .map(|x| x.trim())
    .filter(|x| !x.is_empty())
    .for_each(|x| {
      let (class, list) = x.split_once(":").unwrap();

      let class = class.trim();

      if !valid.contains(&class) {
        panic!("Invalid prototype `{}`", class);
      }

      let list = list.trim();
      let list = &list[1..list.len() - 1];
      let list = list.trim();

      let list = list.split(",")
        .into_iter()
        .map(|x| x.trim())
        .map(|x| {
          let (name, fname) = x.split_once("=").unwrap();

          let docfn = format!("_inner_callable_{}_doc", fname);
          let docfn = Ident::new(&docfn, Span::call_site());

          let fname = Ident::new(fname, Span::call_site());

          (
            quote! {
              (#name, #fname)
            },
            quote! {
              #name => #docfn
            }
          )
        })
        .collect::<Vec<_>>();

      let fns = list.iter()
        .map(|(f,_)| f)
        .collect::<Vec<_>>();

      let prot = list.iter()
        .map(|(_,f)| f)
        .collect::<Vec<_>>();
      
      let class = Ident::new(class, Span::call_site());

      extnds.push(
        quote! {
          #class: &[
            #(#fns),*
          ]
        }
      );
      proto.push(
        quote! {
          proto.#class = interpreter::hashmap! {
            #(#prot),*
          };
        }
      );
    });
  
  quote! {
    fn prototype_docs(&self) -> interpreter::PrototypeDocs {
      let mut proto = interpreter::PrototypeDocs::default();

      #(#proto)*

      proto
    }

    fn prototype(&self) -> interpreter::Extends {
      interpreter::Extends {
        #(#extnds,)*
        ..std::default::Default::default()
      }
    }
  }.into()
}
