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

impl Parse for Mnemonic {
  fn parse(input: ParseStream) -> Result<Self> {
    input.parse::<syn::LitStr>()?.value();
    Ok(Mnemonic { })
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
    let mnemonic = input.parse::<Mnemonic>()?;
    input.parse::<Token![,]>()?;
    let bitpattern = input.parse::<BitPattern>()?;
    input.parse::<Token![,]>()?;
    let description = input.parse::<syn::LitStr>()?.value();
    return Ok(Instruction { name: name, mnemonic: mnemonic, bitpattern: bitpattern, description: description, parts: vec!() });
  }
}

#[derive(PartialEq,Eq)]
struct Instruction {
  name: String,
  mnemonic: Mnemonic,
  bitpattern: BitPattern,
  description: String,
  parts: Vec<(Stage, Action, Timing)>,
}

#[derive(PartialEq,Eq)]
struct Mnemonic {
}

#[derive(PartialEq,Eq)]
struct BitPattern {
  pat: Vec<PatBit>,
}

#[derive(PartialEq,Eq,Debug)]
enum PatBit {
  Zero,
  One,
  Underscore,
  Var(syn::ExprType),
}

impl Parse for BitPattern {
  fn parse(input: ParseStream) -> Result<Self> {
    let mut pat: Vec<PatBit> = vec!();
    while(!input.peek(Token![,])) {
      //eprint!("Building {:?}", pat);
      if input.peek(syn::LitInt) {
        let digit = input.parse::<syn::LitInt>()?;
        match digit.base10_parse::<u16>()? {
          0 => pat.push(PatBit::Zero),
          1 => pat.push(PatBit::One),
          x => return Err(syn::Error::new_spanned(digit, format!("Expecting bit, got {}", x))),
        }
      } else if input.peek(Token![_]) {
        input.parse::<Token![_]>()?;
        pat.push(PatBit::Underscore);
      } else {
        pat.push(PatBit::Var(input.parse::<syn::ExprType>()?));
      }
    }
    return Ok(BitPattern { pat: pat });
  }
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
   * bitpattern 0 1 _ are bits, name:enc is a name with encoding enc
   * 
   *
   * tick function
   * fetch (and deposit)
   * decode (and encode)
   * 
   */

  let chip_info: ChipInfo = syn::parse(input).unwrap();
  let mod_name = format_ident!("{}", chip_info.name);
  let instruction_seq: syn::punctuated::Punctuated<syn::Variant, Token![,]> = chip_info.instructions.instructions.iter().map(|instr| {
    let name = quote::format_ident!("{}", instr.name);
    let v: syn::Variant = syn::parse_quote! {
      #name(Instructions::#name)
    };
    v
  }).collect();
  let instruction_structs: Vec<syn::ItemStruct> = chip_info.instructions.instructions.into_iter().map(|instr| {
    let name = quote::format_ident!("{}", instr.name);
    let v: syn::ItemStruct = syn::parse_quote! {
      #[derive(Debug,PartialEq,Eq)]
      pub struct #name {
      }
    };
    v
  }).collect();
  (quote! {
    mod #mod_name {
      pub mod Instructions {
        #(#instruction_structs)*
      }
      #[derive(Debug,PartialEq,Eq)]
      pub enum Instruction {
        #instruction_seq
      }
    }
  }).into()
}

