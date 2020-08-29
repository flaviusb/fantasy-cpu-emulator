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
  name: String,
  pipeline: Pipeline,
  instructions: Instructions,
}

#[derive(PartialEq,Eq)]
struct Pipeline {
}

impl Parse for Pipeline {
  fn parse(input: ParseStream) -> Result<Self> {
    Ok(Pipeline { })
  }
}

#[derive(PartialEq,Eq)]
struct Instructions {
} 

impl Parse for Instructions {
  fn parse(input: ParseStream) -> Result<Self> {
    Ok(Instructions { })
  }
}

impl Parse for ChipInfo {
  fn parse(input: ParseStream) -> Result<Self> {
    input.parse::<Token![#]>()?;
    let name = input.parse::<Ident>()?.to_string();
    // Get sections if they exist
    // Sections: Pipeline, Instructions, Encoding Tables
    let mut pipeline: Option<Pipeline> = None;
    let mut instructions: Option<Instructions> = None;
    while(pipeline == None || instructions == None) {
      input.parse::<Token![#]>()?;
      input.parse::<Token![#]>()?;
      let section = input.parse::<Ident>()?;
      match section.to_string().as_str() {
        "Pipeline" => {
          pipeline = Some(Pipeline { });
        },
        "Instructions" => {
          instructions = Some(Instructions { });
        },
        section_name => {
          return Err(syn::Error::new_spanned(section, format!("Unexpected section name; got {}, expected Pipeline or Instructions.", section_name)));
          //return Err(input.error("Unexpected section name; expected Pipeline or Instructions."));
        },
      }
    }
    Ok(ChipInfo { name:name, pipeline: pipeline.unwrap(), instructions: instructions.unwrap() })
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

  let chip_info: ChipInfo = syn::parse(input).unwrap();
  let mod_name = format_ident!("{}", chip_info.name);
  (quote! {
    mod #mod_name {
      pub fn witness() -> u8 {
        3
      }
    }
  }).into()
}

