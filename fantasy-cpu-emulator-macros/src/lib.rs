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
  instructions: Vec<Instruction>,
} 

impl Parse for Instruction {
  fn parse(input: ParseStream) -> Result<Self> {
    let name = input.parse::<Ident>()?.to_string();
    input.parse::<Token![,]>()?;
    let description = input.parse::<syn::LitStr>()?.value();
    return Ok(Instruction { name: name, mnemonic: Mnemonic { }, bitpattern: Bitpattern { }, description: description, parts: vec!() });
  }
}

#[derive(PartialEq,Eq)]
struct Instruction {
  name: String,
  mnemonic: Mnemonic,
  bitpattern: Bitpattern,
  description: String,
  parts: Vec<(Stage, Action, Timing)>,
}

#[derive(PartialEq,Eq)]
struct Mnemonic {
}

#[derive(PartialEq,Eq)]
struct Bitpattern {
}

#[derive(PartialEq,Eq)]
struct Stage {
}

#[derive(PartialEq,Eq)]
struct Action {
}

#[derive(PartialEq,Eq)]
struct Timing {
}

impl Parse for Instructions {
  fn parse(input: ParseStream) -> Result<Self> {
    syn::custom_punctuation!(H2, ##);
    let mut instructions: Vec<Instruction> = vec!();
    // Peek ahead to see whether we have more instructions
    while(!(input.peek(H2) || input.is_empty())) {
      instructions.push(input.parse::<Instruction>()?);
    }
    Ok(Instructions { instructions: instructions })
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
    syn::custom_punctuation!(H2, ##);
    while(pipeline == None || instructions == None) {
      input.parse::<H2>()?;
      //input.parse::<Token![#]>()?;
      let section = input.parse::<Ident>()?;
      match section.to_string().as_str() {
        "Pipeline" => {
          pipeline = Some(Pipeline { });
        },
        "Instructions" => {
          instructions = Some(input.parse::<Instructions>()?);
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
   * name, mnemonic, bitpattern, description, (stage×action×timing)*
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

