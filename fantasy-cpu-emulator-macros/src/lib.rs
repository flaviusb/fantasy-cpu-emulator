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

use std::collections::HashMap;

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
}

#[derive(PartialEq,Eq)]
enum MemoryType { // Still to add: stacks (for eg hardware return stack), ringbuffers, queues, ...
  Scratch(ScratchMemory),
  Register(Vec<RegisterMemory>),
}

#[derive(PartialEq,Eq)]
struct ScratchMemory {
  word_size: u64,
  address_size: u64,
  words: u64,
}

#[derive(PartialEq,Eq)]
struct RegisterMemory {
  name: String, // we need geometry information in here as well
  width: u64,   // but we don't handle vector registers, lanes, register sets, subaddressing schemes, flags, special casing IP, registers with a fixed value etc yet
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

fn mkTypeR(name: &str) -> syn::Type {
  return syn::Type::Path(syn::TypePath{qself: None, path: syn::Path{leading_colon: None, segments: vec!(syn::punctuated::Pair::End(syn::PathSegment{ident:syn::Ident::new(name, proc_macro2::Span::call_site()), arguments: syn::PathArguments::None})).into_iter().collect()}});
}
fn mkType(name: String) -> syn::Type {
  return syn::Type::Path(syn::TypePath{qself: None, path: syn::Path{leading_colon: None, segments: vec!(syn::punctuated::Pair::End(syn::PathSegment{ident:syn::Ident::new(&name, proc_macro2::Span::call_site()), arguments: syn::PathArguments::None})).into_iter().collect()}});
}
fn mkType2(segment1: String, name: String) -> syn::Type {
  return syn::Type::Path(syn::TypePath{qself: None, path: syn::Path{leading_colon: None, segments: vec!(syn::punctuated::Pair::Punctuated(syn::PathSegment{ident:syn::Ident::new(&segment1, proc_macro2::Span::call_site()), arguments: syn::PathArguments::None}, Token![::](proc_macro2::Span::call_site())), syn::punctuated::Pair::End(syn::PathSegment{ident:syn::Ident::new(&name, proc_macro2::Span::call_site()), arguments: syn::PathArguments::None})).into_iter().collect()}});
}

fn rationalise(ty: syn::Type) -> (syn::Type, u32, Option<(String, syn::Type, syn::Ident)>) {
    let idx_len = 64; // Find a better way to do this
    let IDX = mkTypeR("usize");
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
        "i8" => return (ty, 8, None),
        "u8" => return (ty, 8, None),
        y    => panic!(format!("I don't understand {:?}", y)),
      }
    },
    syn::Type::Array(syn::TypeArray{bracket_token, elem, semi_token, len}) => {
      match *elem {
        syn::Type::Verbatim(x) => {
          match x.to_string() .as_ref(){
            "mem" => {
              return (IDX.clone(), idx_len, Some(("mem".to_string(), IDX, format_ident!("mem"))))
            },
            y     => panic!(format!("I don't understand {}", y)),
          }
        },
        syn::Type::Path(syn::TypePath{qself, path}) if qself.is_none() && path.is_ident("mem") => {
          return (IDX.clone(), idx_len, Some(("mem".to_string(), IDX, format_ident!("mem"))))
        },
        syn::Type::Path(syn::TypePath{qself, path}) if qself.is_none() && path.is_ident("u") => {
          let len = match len {
            syn::Expr::Lit(syn::ExprLit{lit: syn::Lit::Int(x), ..}) => x.base10_parse::<u32>().unwrap(),
            y => panic!(format!("I don't understand {:?}", y)),
          };
          let name = format!("U{}", len);
          if len > 128 {
            panic!("Unsigned value too long: {} bits", len);
          } else if len == 128 {
            return (U128, len, None)
          } else if len > 64 {
            return (mkType2("super".to_string(), format!("U{}", len)), len, Some((name, U128, format_ident!("U{}", len))))
          } else if len == 64 {
            return (U64, len, None)
          } else if len > 32 {
            return (mkType2("super".to_string(), format!("U{}", len)), len, Some((name, U64, format_ident!("U{}", len))))
          } else if len == 32 {
            return (U32, len, None)
          } else if len > 16 {
            return (mkType2("super".to_string(), format!("U{}", len)), len, Some((name, U32, format_ident!("U{}", len))))
          } else if len == 16 {
            return (U16, len, None)
          } else if len > 8 {
            return (mkType2("super".to_string(), format!("U{}", len)), len, Some((name, U16, format_ident!("U{}", len))))
          } else if len == 8 {
            return (U8, len, None)
          } else {
            return (mkType2("super".to_string(), format!("U{}", len)), len, Some((name, U8, format_ident!("U{}", len))))
          }
        },
        syn::Type::Path(syn::TypePath{qself, path}) if qself.is_none() && path.is_ident("i") => {
          let len = match len {
            syn::Expr::Lit(syn::ExprLit{lit: syn::Lit::Int(x), ..}) => x.base10_parse::<u32>().unwrap(),
            y => panic!(format!("I don't understand {:?}", y)),
          };
          let name = format!("I{}", len);
          if len > 128 {
            panic!("Unsigned value too long: {} bits", len);
          } else if len == 128 {
            return (I128, len, None)
          } else if len > 64 {
            return (mkType2("super".to_string(), format!("I{}", len)), len, Some((name, I128, format_ident!("I{}", len))))
          } else if len == 64 {
            return (I64, len, None)
          } else if len > 32 {
            return (mkType2("super".to_string(), format!("I{}", len)), len, Some((name, I64, format_ident!("I{}", len))))
          } else if len == 32 {
            return (I32, len, None)
          } else if len > 16 {
            return (mkType2("super".to_string(), format!("I{}", len)), len, Some((name, I32, format_ident!("I{}", len))))
          } else if len == 16 {
            return (I16, len, None)
          } else if len > 8 {
            return (mkType2("super".to_string(), format!("I{}", len)), len, Some((name, I16, format_ident!("I{}", len))))
          } else if len == 8 {
            return (I8, len, None)
          } else {
            return (mkType2("super".to_string(), format!("I{}", len)), len, Some((name, I8, format_ident!("I{}", len))))
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
    mod kw {
      syn::custom_keyword!(is);
      syn::custom_keyword!(bit);
    }
    let name = input.parse::<Ident>()?.to_string();
    input.parse::<kw::is>();
    //let mut kind: Option<MemoryType> = None;
    match input.parse::<syn::Ident>()?.to_string().as_ref() {
      "scratch" | "Scratch"   => {
        let mut word_size: Option<u64> = None;
        let mut address_size: Option<u64> = None;
        let mut words: Option<u64> = None;
        while(input.peek(Token![*]) && !input.is_empty()) {
          input.parse::<Token![*]>()?;
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
        return Ok(Memory { name: name, kind: MemoryType::Scratch(ScratchMemory {word_size: word_size.unwrap(), address_size: address_size.unwrap(), words: words.unwrap() }) });
      },
      "register" | "Register" => {
        let mut registers: Vec<RegisterMemory> = vec!();
        while(input.peek(Token![*]) && !input.is_empty()) {
          input.parse::<Token![*]>()?;
          let name = input.parse::<syn::Ident>()?.to_string();
          input.parse::<Token![:]>()?;
          let num = input.parse::<syn::LitInt>()?.base10_parse::<u64>()?;
          input.parse::<kw::bit>()?;
          registers.push(RegisterMemory { name: name, width: num });
        };
        return  Ok(Memory { name: name, kind: MemoryType::Register(registers) });
      },
      x => panic!(format!("Expected memory type: scratch or Scratch or register or Register, got {} instead.", x)),
    };
    panic!("Failed to parse memory.");
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

  fn mkField(name: String, ty: syn::Type) -> syn::Field {
    syn::Field { attrs: vec!(), vis: syn::Visibility::Public(syn::VisPublic{pub_token: Token![pub](proc_macro2::Span::call_site())}), ident: Some(syn::Ident::new(&name, proc_macro2::Span::call_site())), colon_token: Some(Token![:](proc_macro2::Span::call_site())), ty: ty }
  }

  let chip_info: ChipInfo = syn::parse(input).unwrap();
  let mod_name = format_ident!("{}", chip_info.name.clone());
  let instruction_seq: syn::punctuated::Punctuated<syn::Variant, Token![,]> = chip_info.instructions.instructions.iter().map(|instr| {
    let name = quote::format_ident!("{}", instr.name);
    let v: syn::Variant = syn::parse_quote! {
      #name(Instructions::#name)
    };
    v
  }).collect();
  let mut rationalised_types: HashMap<String, syn::ItemType> = HashMap::new();
  let mut decode: Vec<syn::Arm> = vec!();
  let mut instruction_structs: Vec<syn::ItemStruct> = vec!();
  for instr in chip_info.instructions.instructions.into_iter() {
    let name = quote::format_ident!("{}", instr.name);
    let mut args: syn::punctuated::Punctuated<syn::Field, syn::token::Comma> = syn::punctuated::Punctuated::new();

    // This does two things; it populates args and it also builds up an Arm which will eventually be pushed into decode
    let mut cmp = 0;
    let mut ands = 0;
    let mut idx = 0;
    let mut fields: Vec<syn::FieldValue> = vec!();

    for pat in instr.bitpattern.pat.iter() {
      match pat {
        PatBit::Zero       => {
          idx += 1;
          ands = ands | (1 << idx);
        },
        PatBit::One        => {
          ands = ands | (1 << idx);
          cmp = cmp | (1 << idx);
          idx += 1;
        },
        PatBit::Underscore => {
          idx += 1;
        },
        PatBit::Var(syn::ExprType{attrs, expr, colon_token, ty})   => {
          // First we generate the declation field corresponding to this variable in the declaration of the instruction stucture
          let name = match &**expr {
            syn::Expr::Path(path) => path.path.get_ident().unwrap().clone(),
            x                     => panic!(format!("Got {:?}, expected a Path.", x)),
          };
          let (ty2, len, maybe_decl) = rationalise(*ty.clone());
          for (decl_name, backing_type, decl_type) in maybe_decl {
            rationalised_types.insert(decl_name, syn::parse_quote!{ pub type #decl_type = #backing_type; });
          }
          args.push(mkField(name.to_string(), ty2));
          
          // Then we generate the field value corresponding to this variable in the match arm of the decoder
          let shift = idx;
          idx += len;
          let mask = ((2^len) - 1);
          let variable_getter: syn::Expr = (syn::parse_quote! { (((input >> #shift) & #mask) as #ty) });
          let field: syn::FieldValue = syn::FieldValue { attrs: vec!(), member: syn::Member::Named(name), colon_token: Some(Token![:](proc_macro2::Span::call_site())), expr: variable_getter };
          fields.push(field);
        },
      }
    }

    // Build the instruction struct from the generated fields
    let v: syn::ItemStruct = syn::parse_quote! {
      #[derive(Debug,PartialEq,Eq)]
      pub struct #name {
        #args
      }
    };
    instruction_structs.push(v);

    // Then we build the decode arm
    let guard: syn::Expr = (syn::parse_quote! { ((input & #ands) == #cmp) });
    let chip_name = quote::format_ident!("{}", chip_info.name.clone());
    let result: syn::Expr = (syn::parse_quote! { #chip_name::Instruction::#name(#chip_name::Instructions::#name { #(#fields),* }) });
    decode.push(syn::parse_quote! { _ if #guard => #result, });
  };
  let mut mems: syn::punctuated::Punctuated<syn::Field, syn::token::Comma> = syn::punctuated::Punctuated::new();
  for mem in chip_info.memories.iter() {
    // All memories are scratch for now
    //mems.push(mkField(mem.name, 
  }
  let decl_types: Vec<syn::ItemType> = rationalised_types.into_iter().map(|(k, v)| v).collect();
  (quote! {
    mod #mod_name {
      #(#decl_types)*

      pub struct Memories {

      }
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

