extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{parse::{Parse, ParseStream}, Result, parse_macro_input, Attribute, FnArg, Ident, ItemFn, Token};

// Parses a unit struct with attributes.
//
//     #[path = "s.tmpl"]
//     struct S;
struct UnitStruct {
  attrs: Vec<Attribute>,
  struct_token: Token![struct],
  name: Ident,
  semi_token: Token![;],
}

impl Parse for UnitStruct {
  fn parse(input: ParseStream) -> Result<Self> {
    Ok(UnitStruct {
      attrs: input.call(Attribute::parse_outer)?,
      struct_token: input.parse()?,
      name: input.parse()?,
      semi_token: input.parse()?,
    })
  }
}

#[proc_macro_attribute]
pub fn define(args: TokenStream, input: TokenStream) -> TokenStream {
  //let args = syn::parse(args).unwrap();

  let input = parse_macro_input!(input as ItemFn);

  let ItemFn {
    // The function signature
    sig,
    // The visibility specifier of this function
    vis,
    // The function block or body
    block,
    // Other attributes applied to this function
    attrs,
  } = input;

  sig.inputs.iter().for_each(|s| {
    match s {
      FnArg::Typed(s) => {
        println!("{:?}", s.ty.to_token_stream());
      }
      _ => {}
    }
  });

  let new = Ident::new(&format!("_call_{}", sig.ident), sig.ident.span());

  let params = sig.inputs.to_token_stream();
  println!("New {}", new.to_string());

  let ident = sig.ident;

  quote! {
    fn #ident(args: &Vec<String>) {

    }

    #vis fn #new(#params) #block
  }.into()
}