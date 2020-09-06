extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;
extern crate proc_macro2;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse, Attribute, PathSegment, Result, Token};
use syn::parse::{Parse, ParseStream, Parser};
use syn::spanned::Spanned;
use syn::{Expr, Ident, Type, Visibility};

struct ChipInfo {
  name: String,
  memories: Vec<Memory>,
  pipeline: Pipeline,
  instructions: Instructions,
}

#[derive(PartialEq,Eq)]
struct Memory {
  name: String,
  kind: MemoryType,
  word_size: u64,
  address_size: u64,
  words: u64,
}

#[derive(PartialEq,Eq)]
enum MemoryType {
  Scratch(),
  Register(),
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

fn rationalise(ty: syn::Type) -> syn::Type {
    let IDX:   syn::Type = syn::Type::Path(syn::TypePath{qself: None, path: syn::Path{leading_colon: None, segments: vec!(syn::punctuated::Pair::End(syn::PathSegment{ident:syn::Ident::new("usize", proc_macro2::Span::call_site()), arguments: syn::PathArguments::None})).into_iter().collect()}});
    let I8:    syn::Type = syn::Type::Path(syn::TypePath{qself: None, path: syn::Path{leading_colon: None, segments: vec!(syn::punctuated::Pair::End(syn::PathSegment{ident:syn::Ident::new("i8"   , proc_macro2::Span::call_site()), arguments: syn::PathArguments::None})).into_iter().collect()}});
    let I16:   syn::Type = syn::Type::Path(syn::TypePath{qself: None, path: syn::Path{leading_colon: None, segments: vec!(syn::punctuated::Pair::End(syn::PathSegment{ident:syn::Ident::new("i16"  , proc_macro2::Span::call_site()), arguments: syn::PathArguments::None})).into_iter().collect()}});
    let I32:   syn::Type = syn::Type::Path(syn::TypePath{qself: None, path: syn::Path{leading_colon: None, segments: vec!(syn::punctuated::Pair::End(syn::PathSegment{ident:syn::Ident::new("i32"  , proc_macro2::Span::call_site()), arguments: syn::PathArguments::None})).into_iter().collect()}});
    let I64:   syn::Type = syn::Type::Path(syn::TypePath{qself: None, path: syn::Path{leading_colon: None, segments: vec!(syn::punctuated::Pair::End(syn::PathSegment{ident:syn::Ident::new("i64"  , proc_macro2::Span::call_site()), arguments: syn::PathArguments::None})).into_iter().collect()}});
    let I128:  syn::Type = syn::Type::Path(syn::TypePath{qself: None, path: syn::Path{leading_colon: None, segments: vec!(syn::punctuated::Pair::End(syn::PathSegment{ident:syn::Ident::new("i128" , proc_macro2::Span::call_site()), arguments: syn::PathArguments::None})).into_iter().collect()}});
    let U8:    syn::Type = syn::Type::Path(syn::TypePath{qself: None, path: syn::Path{leading_colon: None, segments: vec!(syn::punctuated::Pair::End(syn::PathSegment{ident:syn::Ident::new("u8"   , proc_macro2::Span::call_site()), arguments: syn::PathArguments::None})).into_iter().collect()}});
    let U16:   syn::Type = syn::Type::Path(syn::TypePath{qself: None, path: syn::Path{leading_colon: None, segments: vec!(syn::punctuated::Pair::End(syn::PathSegment{ident:syn::Ident::new("u16"  , proc_macro2::Span::call_site()), arguments: syn::PathArguments::None})).into_iter().collect()}});
    let U32:   syn::Type = syn::Type::Path(syn::TypePath{qself: None, path: syn::Path{leading_colon: None, segments: vec!(syn::punctuated::Pair::End(syn::PathSegment{ident:syn::Ident::new("u32"  , proc_macro2::Span::call_site()), arguments: syn::PathArguments::None})).into_iter().collect()}});
    let U64:   syn::Type = syn::Type::Path(syn::TypePath{qself: None, path: syn::Path{leading_colon: None, segments: vec!(syn::punctuated::Pair::End(syn::PathSegment{ident:syn::Ident::new("u64"  , proc_macro2::Span::call_site()), arguments: syn::PathArguments::None})).into_iter().collect()}});
    let U128:  syn::Type = syn::Type::Path(syn::TypePath{qself: None, path: syn::Path{leading_colon: None, segments: vec!(syn::punctuated::Pair::End(syn::PathSegment{ident:syn::Ident::new("u128" , proc_macro2::Span::call_site()), arguments: syn::PathArguments::None})).into_iter().collect()}});
  // We need a registry of types
  match ty.clone() {
    syn::Type::Path(syn::TypePath{qself, path}) => {
      match path.get_ident().unwrap().to_string().as_ref() {
        "i8" => return ty,
        "u8" => return ty,
        y    => panic!(format!("I don't understand {:?}", y)),
      }
    },
    syn::Type::Array(syn::TypeArray{bracket_token, elem, semi_token, len}) => {
      match *elem {
        syn::Type::Verbatim(x) => {
          match x.to_string() .as_ref(){
            "mem" => {
              return IDX
            },
            y     => panic!(format!("I don't understand {}", y)),
          }
        },
        syn::Type::Path(syn::TypePath{qself, path}) if qself.is_none() && path.is_ident("mem") => {
          return IDX
        },
        syn::Type::Path(syn::TypePath{qself, path}) if qself.is_none() && path.is_ident("u") => {
          let len = match len {
            syn::Expr::Lit(syn::ExprLit{lit: syn::Lit::Int(x), ..}) => x.base10_parse::<u32>().unwrap(),
            y => panic!(format!("I don't understand {:?}", y)),
          };
          if len > 128 {
            panic!("Unsigned value too long: {} bits", len);
          } else if len > 64 {
            return U128
          } else if len > 32 {
            return U64
          } else if len > 16 {
            return U32
          } else if len > 8 {
            return U16
          } else {
            return U8
          }
        },
        syn::Type::Path(syn::TypePath{qself, path}) if qself.is_none() && path.is_ident("i") => {
          let len = match len {
            syn::Expr::Lit(syn::ExprLit{lit: syn::Lit::Int(x), ..}) => x.base10_parse::<u32>().unwrap(),
            y => panic!(format!("I don't understand {:?}", y)),
          };
          if len > 128 {
            panic!("Unsigned value too long: {} bits", len);
          } else if len > 64 {
            return I128
          } else if len > 32 {
            return I64
          } else if len > 16 {
            return I32
          } else if len > 8 {
            return I16
          } else {
            return I8
          }
        },
        x     => panic!(format!("I don't understand {:?}", x)),
      }
    },
    _ => panic!("???")
  }
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

impl Parse for Memory {
  fn parse(input: ParseStream) -> Result<Self> {
    let name = input.parse::<Ident>()?.to_string();
    let mut kind: Option<MemoryType> = None;
    let mut word_size: Option<u64> = None;
    let mut address_size: Option<u64> = None;
    let mut words: Option<u64> = None;
    while(input.peek(Token![*]) && !input.is_empty()) {
      input.parse::<Token![*]>()?;
      if input.peek(syn::Ident) {
        // memory type case
        // Currently can only be scratch
        match input.parse::<syn::Ident>()?.to_string().as_ref() {
          "scratch" | "Scratch"   => kind = Some(MemoryType::Scratch()),
          "register" | "Register" => kind = Some(MemoryType::Register()),
          x                       => panic!(format!("Expected memory type: scratch or Scratch or register or Register, got {} instead.", x)),
        };
      } else {
        let num = input.parse::<syn::LitInt>()?.base10_parse::<u64>()?;
        match input.parse::<syn::Ident>()?.to_string().as_ref() {
          "bit" => {
            match input.parse::<syn::Ident>()?.to_string().as_ref() {
              "word" => {
                word_size = Some(num);
              },
              "address" => {
                match input.parse::<syn::Ident>()?.to_string().as_ref() {
                  "size" => address_size = Some(num),
                  x      => panic!(format!("Expected size, got {}.", x)),
                };
              },
              x => panic!(format!("Expected word or address, got {}.", x)),
            };
          },
          "words" | "word" => words = Some(num),
          x => panic!(format!("Expected bit, got {}.", x)),
        };
      };
    }
    Ok(Memory { name: name, kind: kind.unwrap(), word_size: word_size.unwrap(), address_size: address_size.unwrap(), words: words.unwrap() })
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
    let mut memories: Vec<Memory> = vec!();
    syn::custom_punctuation!(H2, ##);
    while(input.peek(H2) && !input.is_empty()) {
      input.parse::<H2>()?;
      let section = input.parse::<Ident>()?;
      match section.to_string().as_str() {
        "Dis" => {
          input.parse::<Token![/]>()?;
          let section_continued = input.parse::<Ident>()?.to_string();
          if !(section_continued == "Assembler") {
            panic!(format!("Expected Dis/Assembler section, got Dis/{} instead.", section_continued));
          } else {
            //
          }
        },
        "Memory" => {
          while(input.peek(Token![-]) && !input.is_empty()) {
            input.parse::<Token![-]>()?;
            memories.push(input.parse::<Memory>()?);
          }
        },
        "Pipeline" => {
          pipeline = Some(Pipeline { });
        },
        "Instructions" => {
          instructions = Some(input.parse::<Instructions>()?);
        },
        section_name => {
          return Err(syn::Error::new_spanned(section, format!("Unexpected section name; got {}, expected Pipeline, Instructions, Memory, or Dis/Assembler.", section_name)));
          //return Err(input.error("Unexpected section name; expected Pipeline or Instructions."));
        },
      }
    }
    Ok(ChipInfo { name:name, memories: memories, pipeline: pipeline.unwrap(), instructions: instructions.unwrap() })
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
    let mut args: syn::punctuated::Punctuated<syn::Field, syn::token::Comma> = syn::punctuated::Punctuated::new();
    instr.bitpattern.pat.iter().for_each(|pat| {
      match pat {
        PatBit::Zero       => (),
        PatBit::One        => (),
        PatBit::Underscore => (),
        PatBit::Var(syn::ExprType{attrs, expr, colon_token, ty})   => {
          //let boxed = &exp.expr;
          let name = match &**expr {
            syn::Expr::Path(path) => path.path.get_ident().unwrap().clone(),
            x                     => panic!(format!("Got {:?}, expected a Path.", x)),
          };
          let ty2 = rationalise(*ty.clone());
          args.push(syn::Field { attrs: vec!(), vis: syn::Visibility::Public(syn::VisPublic{pub_token: Token![pub](proc_macro2::Span::call_site())}), ident: Some(syn::Ident::new(&name.to_string(), proc_macro2::Span::call_site())), colon_token: Some(Token![:](proc_macro2::Span::call_site())), ty: ty2 });
        },
      }
    });

    let v: syn::ItemStruct = syn::parse_quote! {
      #[derive(Debug,PartialEq,Eq)]
      pub struct #name {
        #args
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

