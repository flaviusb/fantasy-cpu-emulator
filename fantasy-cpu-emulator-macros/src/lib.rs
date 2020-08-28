extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, Attribute, PathSegment, Result, Token};
use syn::parse::{Parse, ParseStream, Parser};
use syn::spanned::Spanned;
use syn::{Expr, Ident, Type, Visibility};

struct ChipInfo {
  
}

struct Section<T> {
  level: u8,
  name: String,
  contents: T,
}

impl Parse for Section<ChipInfo> {
  fn parse(input: ParseStream) -> Result<Self> {
    input.parse::<Token![#]>()?;
    let name = input.parse::<Ident>()?.to_string();
    Ok(Section { level:1, name:name, contents:ChipInfo {} })
  }
}


#[proc_macro]
pub fn define_chip(input: TokenStream) -> TokenStream {
  /*
   * instruction definitions
   *
   * pipeline
   * name, function, props (eg Fetch/Deposit, Decode/Encode)
   *
   * encoding tables/functions
   * (with predefined functions like uX, iX)
   *
   * instruction
   * mnemonic, bitpattern, description, (stage×action×timing)*
   *
   * bitpattern 0 1 are bits, name:enc is a name with encoding enc
   * 
   *
   * tick function
   * fetch (and deposit)
   * decode (and encode)
   * 
   */

  let chip_info: Section<ChipInfo> = syn::parse(input).unwrap();
  (quote! {
    1
  }).into()
}

