extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;
extern crate proc_macro2;

use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::{parse, Attribute, PathSegment, Result, Token};
use syn::parse::{Parse, ParseStream, Parser};
use syn::spanned::Spanned;
use syn::{Expr, Ident, Type, Visibility};

use std::collections::HashMap;

struct ChipInfo {
  name: String,
  instruction_width: u8,
  memories: Vec<Memory>,
  pipeline: Pipeline,
  instructions: Instructions,
  raw: Vec<syn::Item>,
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
  pipelines: Vec<Pipe>,
}

#[derive(PartialEq,Eq)]
enum Pipe {
  Use            { name: syn::Ident, real: syn::ExprPath },
  PerInstruction { name: syn::Ident, input: syn::TypePath, output: syn::TypePath },
}

impl Parse for Pipeline {
  fn parse(input: ParseStream) -> Result<Self> {
    syn::custom_punctuation!(H2, ##);
    let mut pipelines: Vec<Pipe> = vec!();
    while(!(input.peek(H2) || input.is_empty())) {
      input.parse::<Token![-]>()?;
      let name = input.parse::<Ident>()?; //.to_string();
      if (input.peek(Token![=])) {
        input.parse::<Token![=]>()?;
        let real = input.parse::<syn::ExprPath>()?;
        pipelines.push( Pipe::Use { name: name, real: real } );
      } else {
        input.parse::<Token![:]>()?;
        let fn_in = input.parse::<syn::TypePath>()?;
        input.parse::<Token![->]>()?;
        let fn_out = input.parse::<syn::TypePath>()?;
        pipelines.push( Pipe::PerInstruction { name: name, input: fn_in, output: fn_out } );
      }
    }
    Ok(Pipeline { pipelines: pipelines })
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
    let bitpattern = input.parse::<BitPattern>()?;
    input.parse::<Token![,]>()?;
    let description = input.parse::<syn::LitStr>()?.value();
    return Ok(Instruction { name: name, bitpattern: bitpattern, description: description, parts: vec!() });
  }
}

#[derive(PartialEq,Eq)]
struct Instruction {
  name: String,
  bitpattern: BitPattern,
  description: String,
  parts: Vec<(Stage, Action, Timing)>,
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

fn rationalise(ty: syn::Type) -> (syn::Type, u32, bool, Option<(String, syn::Type, syn::Ident)>) {
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
        "i8" => return (ty, 8, true, None),
        "u8" => return (ty, 8, false, None),
        y    => panic!(format!("I don't understand {:?}", y)),
      }
    },
    syn::Type::Array(syn::TypeArray{bracket_token, elem, semi_token, len}) => {
      match *elem {
        syn::Type::Verbatim(x) => {
          match x.to_string() .as_ref(){
            "mem" => {
              let len = match len {
                syn::Expr::Lit(syn::ExprLit{lit: syn::Lit::Int(x), ..}) => x.base10_parse::<u32>().unwrap(),
                y => panic!(format!("I don't understand {:?}", y)),
              };
              return (IDX.clone(), len, false, Some(("mem".to_string(), IDX, format_ident!("mem"))))
            },
            y     => panic!(format!("I don't understand {}", y)),
          }
        },
        syn::Type::Path(syn::TypePath{qself, path}) if qself.is_none() && path.is_ident("mem") => {
          let len = match len {
            syn::Expr::Lit(syn::ExprLit{lit: syn::Lit::Int(x), ..}) => x.base10_parse::<u32>().unwrap(),
            y => panic!(format!("I don't understand {:?}", y)),
          };
          return (IDX.clone(), len, false, Some(("mem".to_string(), IDX, format_ident!("mem"))))
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
            return (U128, len, false, None)
          } else if len > 64 {
            return (mkType2("super".to_string(), format!("U{}", len)), len, false, Some((name, U128, format_ident!("U{}", len))))
          } else if len == 64 {
            return (U64, len, false, None)
          } else if len > 32 {
            return (mkType2("super".to_string(), format!("U{}", len)), len, false, Some((name, U64, format_ident!("U{}", len))))
          } else if len == 32 {
            return (U32, len, false, None)
          } else if len > 16 {
            return (mkType2("super".to_string(), format!("U{}", len)), len, false, Some((name, U32, format_ident!("U{}", len))))
          } else if len == 16 {
            return (U16, len, false, None)
          } else if len > 8 {
            return (mkType2("super".to_string(), format!("U{}", len)), len, false, Some((name, U16, format_ident!("U{}", len))))
          } else if len == 8 {
            return (U8, len, false, None)
          } else {
            return (mkType2("super".to_string(), format!("U{}", len)), len, false, Some((name, U8, format_ident!("U{}", len))))
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
            return (I128, len, true, None)
          } else if len > 64 {
            return (mkType2("super".to_string(), format!("I{}", len)), len, true, Some((name, I128, format_ident!("I{}", len))))
          } else if len == 64 {
            return (I64, len, true, None)
          } else if len > 32 {
            return (mkType2("super".to_string(), format!("I{}", len)), len, true, Some((name, I64, format_ident!("I{}", len))))
          } else if len == 32 {
            return (I32, len, true, None)
          } else if len > 16 {
            return (mkType2("super".to_string(), format!("I{}", len)), len, true, Some((name, I32, format_ident!("I{}", len))))
          } else if len == 16 {
            return (I16, len, true, None)
          } else if len > 8 {
            return (mkType2("super".to_string(), format!("I{}", len)), len, true, Some((name, I16, format_ident!("I{}", len))))
          } else if len == 8 {
            return (I8, len, true, None)
          } else {
            return (mkType2("super".to_string(), format!("I{}", len)), len, true, Some((name, I8, format_ident!("I{}", len))))
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
    pat.reverse();
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
    let mut instruction_width: Option<u8> = None;
    let mut raw: Vec<syn::Item> = vec!();
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
          pipeline = Some(input.parse::<Pipeline>()?);
        },
        "Instructions" => {
          instructions = Some(input.parse::<Instructions>()?);
        },
        "Misc" => {
          input.parse::<Token![-]>()?;
          let ins = input.parse::<Ident>()?.to_string();
          if ins != "Instruction" {
            panic!("Expected 'Instruction', got {}.", ins);
          }
          let width = input.parse::<Ident>()?.to_string();
          if width != "width" {
            panic!("Expected 'width', got {}.", width);
          }
          input.parse::<Token![:]>()?;
          instruction_width = Some(input.parse::<syn::LitInt>()?.base10_parse::<u8>()?);
        },
        "Raw" => {
          while(!(input.peek(H2) || input.is_empty())) {
            raw.push(input.parse::<syn::Item>()?);
          };
        },
        section_name => {
          return Err(syn::Error::new_spanned(section, format!("Unexpected section name; got {}, expected Pipeline, Instructions, Memory, Dis/Assembler, Structs, or Misc.", section_name)));
        },
      }
    }
    Ok(ChipInfo { name:name, instruction_width: instruction_width.unwrap(), memories: memories, pipeline: pipeline.unwrap(), instructions: instructions.unwrap(), raw: raw })
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
   * name, bitpattern, description, (stage×action×timing)*
   *
   * bitpattern 0 1 _ are bits, name:enc is a name with encoding enc
   * 
   *
   * tick function
   * fetch (and deposit)
   * decode (and encode)
   * 
   */
  fn convert_if_needed(from: syn::Type, to: syn::Type, expr: syn::Expr) -> syn::Expr {
    if from != to {
      syn::parse_quote! {
        #to::try_from(#expr).unwrap()
      }
    } else {
      expr
    }
  }
  fn mkField(name: String, ty: syn::Type) -> syn::Field {
    syn::Field { attrs: vec!(), vis: syn::Visibility::Public(syn::VisPublic{pub_token: Token![pub](proc_macro2::Span::call_site())}), ident: Some(syn::Ident::new(&name, proc_macro2::Span::call_site())), colon_token: Some(Token![:](proc_macro2::Span::call_site())), ty: ty }
  }
  fn mkFieldPat(name: String, binding: String) -> syn::FieldPat {
    syn::FieldPat {
      attrs: vec!(), member: syn::Member::Named(syn::Ident::new(&name, proc_macro2::Span::call_site())),
      colon_token: Some(Token![:](proc_macro2::Span::call_site())),
      pat: Box::new(syn::Pat::Ident(syn::PatIdent { attrs: vec!(), by_ref: None, mutability: None, ident: syn::Ident::new(&binding, proc_macro2::Span::call_site()), subpat: None, })),
    }
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
  // Stuff in the instruction width right away
  {
    let decl_type = mkType(format!("U{}", chip_info.instruction_width));
    let backing_type = mkType(format!("u{}", chip_info.instruction_width.next_power_of_two()));
    rationalised_types.insert(format!("U{}", chip_info.instruction_width), syn::parse_quote!{ pub type #decl_type = #backing_type; });
  };
  let mut pipelines_with_arms: HashMap<String, Vec<syn::Arm>> = HashMap::new();
  let mut pipelines_with_generated_types: HashMap<String, (syn::ItemType, syn::ItemEnum, Vec<(String, syn::ItemStruct)>)> = HashMap::new();
  let mut decode: Vec<syn::Arm> = vec!();
  let decode_input_type = mkType(format!("U{}", chip_info.instruction_width));
  let mut encode: Vec<syn::Arm> = vec!();
  let encode_output_type = decode_input_type.clone();
  let mut instruction_structs: Vec<syn::ItemStruct> = vec!();
  for instr in chip_info.instructions.instructions.into_iter() {
    let name = quote::format_ident!("{}", instr.name);
    let mut args: syn::punctuated::Punctuated<syn::Field, syn::token::Comma> = syn::punctuated::Punctuated::new();

    // This does two things; it populates args and it also builds up an Arm which will eventually be pushed into decode
    let mut cmp: u128 = 0;
    let mut ands: u128 = 0;
    let mut idx: u32 = 0;
    let mut fields: Vec<syn::FieldValue> = vec!();
    let mut encoded_bit_segments: Vec<syn::Expr> = vec!(); // Each segment of the instruction goes in here, and they are or-ed together
    let mut encode_fields: Vec<syn::FieldPat> = vec!();

    for pat in instr.bitpattern.pat.iter() {
      match pat {
        PatBit::Zero       => {
          ands = ands | (1 << idx);
          idx += 1;
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
          let (ty2, len, signed, maybe_decl) = rationalise(*ty.clone());
          for (decl_name, backing_type, decl_type) in maybe_decl.clone() {
            rationalised_types.insert(decl_name, syn::parse_quote!{ pub type #decl_type = #backing_type; });
          }
          args.push(mkField(name.clone().to_string(), ty2.clone()));
          
          // Then we generate the field value corresponding to this variable in the match arm of the decoder
          let shift = idx;
          idx += len;
          let inner_shift: syn::Expr = syn::parse_quote! {
            (input >> #shift)
          };
          // We need the backing types of all the relevant variables, and then we use that to call convert_if_needed(from, to, expr)
          // backing_extract is the backing type for the bitfield being extracted
          // backing_input is the backing type for the input argument to the generated decode function
          let backing_extract = match (signed, maybe_decl.clone()) {
            (true,  _)                => mkType(format!("u{}", len.next_power_of_two())), // Backing extract type needs to be unsigned
            (false, Some((_, it, _))) => it.clone(),
            (false, None)             => ty2.clone(),
          };
          let backing_input  = mkType(format!("u{}", chip_info.instruction_width.next_power_of_two()));
          let big_type       = mkTypeR("u128");
          let idx_type       = mkTypeR("u32");
          let mask_inner: syn::Expr = syn::parse_quote! {
            ((1 << #len) - 1)
          };
          let mask_safe      = convert_if_needed(idx_type.clone(), big_type.clone(), mask_inner.clone());
          let mask_thing      = convert_if_needed(backing_extract.clone(), backing_input.clone(), mask_safe.clone());
          let shift_part: syn::Expr = syn::parse_quote! {
            (#inner_shift & #mask_thing)
          };
          let shift_safe     = convert_if_needed(backing_input.clone(), big_type.clone(), inner_shift.clone());
          let shifted_and_masked: syn::Expr = syn::parse_quote! {
            (#shift_safe & #mask_safe)
          };
          let shifted_and_masked_safe = convert_if_needed(big_type.clone(), backing_extract.clone(), shifted_and_masked.clone());

          //dbg!(shift_safe);
          //dbg!(mask_safe);
          // Handling the field extraction differs based on signed/unsigned and whether the field is a native size
          let variable_getter: syn::Expr = match (signed, len.next_power_of_two() == len) {
            (false, true) => {
              // This is the simplest path. We can use Rust's defaults
              shifted_and_masked_safe
            },
            (true, true) => {
              // Slightly subtle, but we can assume Rust sizes
              let unsigned_container = mkType(format!("u{}", len.next_power_of_two()));
              let variable_getter: syn::Expr = (syn::parse_quote! {
                {
                  #ty2::from_ne_bytes(#unsigned_container::to_ne_bytes(#shifted_and_masked_safe))
                }
              });
              variable_getter
            },
            (false, false) => {
              // We can extend without any special handling as the value is unsigned
              shifted_and_masked_safe
            },
            (true, false) => {
              // Add sign extension code in the generated code, as what to do depends on sign
              let backing_size = len.next_power_of_two();
              let unsigned_container = mkType(format!("u{}", backing_size));
              let top_bit: syn::Expr = syn::parse_quote! {
                ((1 as #backing_input) << (#len - 1))
              };
              let extension_mask: syn::Expr = syn::parse_quote! {
                ((((1 << #backing_size)-1) as #backing_extract) ^ (((1 << #len)-1) as #backing_extract))
              };
              let extension_mask_safe = convert_if_needed(idx_type.clone(), unsigned_container.clone(), extension_mask);
              let variable_getter: syn::Expr = (syn::parse_quote! {
                {
                  //println!("len {}", #len);
                  //println!("Extracting a signed thing.");
                  let top_bit: #backing_input = #top_bit;
                  //println!("Top bit.");
                  let mask = #mask_safe;
                  //println!("Mask.");
                  if (#shift_part & top_bit) == top_bit {
                    //println!("Top bit set; attempting to extract from {}, {}", #inner_shift, #mask_safe);
                    //println!("Shift safe value: {}", #shift_safe);
                    #ty2::from_ne_bytes(#unsigned_container::to_ne_bytes((#shifted_and_masked_safe) | #extension_mask_safe))
                  } else {
                    //println!("Top bit unset; attempting to extract from {}, {}", #inner_shift, #mask_safe);
                    //println!("Shift safe value: {}", #shift_safe);
                    #ty2::from_ne_bytes(#unsigned_container::to_ne_bytes(#shifted_and_masked_safe))
                  }
                }
              });
              variable_getter
            },
          };
          let field: syn::FieldValue = syn::FieldValue { attrs: vec!(), member: syn::Member::Named(name.clone()), colon_token: Some(Token![:](proc_macro2::Span::call_site())), expr: variable_getter.clone() };
          //println!("Variable getter for field {} of instruction {}: {}", name, instr.name.clone(), (variable_getter.clone()).to_token_stream());
          fields.push(field);

          // For encoding, mask and left shift
          let instruction_width = chip_info.instruction_width;
          let encode_mask_safe = convert_if_needed(idx_type.clone(), big_type.clone(), mask_inner);
          let name_safe: syn::Expr = if signed {
            let backing_size = len.next_power_of_two();
            let unsigned_container = mkType(format!("u{}", backing_size));
            convert_if_needed(unsigned_container.clone(), big_type.clone(), syn::parse_quote! { #unsigned_container::from_ne_bytes(#ty2::to_ne_bytes(#name)) } )
          } else {
            convert_if_needed(ty2.clone(), big_type.clone(), syn::parse_quote! { #name } )
          };
          let encode_shift_safe: syn::Expr = syn::parse_quote! { (#name_safe << #shift) };
          let encoded_bit_segment: syn::Expr = convert_if_needed(big_type.clone(), backing_input.clone(), syn::parse_quote! { (#encode_shift_safe & (#encode_mask_safe << #shift)) });
          //println!("{} encoding for {} is {}", instr.name.clone(), name.clone(), encoded_bit_segment.clone().to_token_stream());
          encoded_bit_segments.push(encoded_bit_segment);

          encode_fields.push(mkFieldPat(name.clone().to_string(), name.clone().to_string()));
        },
      }
    }

    let cmp_thing = convert_if_needed(mkTypeR("u128"), mkType(format!("u{}", chip_info.instruction_width.next_power_of_two())), syn::parse_quote! { (#cmp) });
    encoded_bit_segments.push(cmp_thing);
    // Build the instruction struct from the generated fields
    let v: syn::ItemStruct = syn::parse_quote! {
      #[derive(Debug,PartialEq,Eq)]
      pub struct #name {
        #args
      }
    };
    instruction_structs.push(v);

    // Then we build the decode arm
    let guard: syn::Expr = (syn::parse_quote! { (((input as u128) & #ands) == #cmp) });
    let chip_name = quote::format_ident!("{}", chip_info.name.clone());
    let result: syn::Expr = (syn::parse_quote! { super::Instruction::#name(#name { #(#fields),* }) });
    decode.push(syn::parse_quote! { _ if #guard => #result, });
    encode.push(syn::parse_quote! { super::Instruction::#name(#name { #(#encode_fields),* }) => #(#encoded_bit_segments)|*, });
  };
  let mut mems: syn::punctuated::Punctuated<syn::Field, syn::token::Comma> = syn::punctuated::Punctuated::new();
  for mem in chip_info.memories.iter() {
    // All memories are scratch for now
    //mems.push(mkField(mem.name, 
  }
  let mut pipelines: Vec<syn::Item> = vec!();
  for pipe in chip_info.pipeline.pipelines.iter() {
    match pipe {
      Pipe::Use            { name: name, real: real } => {
        pipelines.push(syn::parse_quote! { use #real as #name; } );
      },
      Pipe::PerInstruction { name: name, input: input, output: out }   => (),
    }
  }
  let decl_types: Vec<syn::ItemType> = rationalised_types.into_iter().map(|(k, v)| v).collect();
  let raw = chip_info.raw;
  (quote! {
    mod #mod_name {
      #(#decl_types)*
      #(#raw)*
      pub struct Memories {

      }
      pub mod Pipeline {
        #(#pipelines)*
      }
      pub mod Instructions {
        pub fn decode(input: super::#decode_input_type) -> super::Instruction {
          use std::convert::TryFrom;
          match input {
            #(#decode)*
            x => panic!(format!("Could not decode instruction: {}", x)),
          }
        }
        pub fn encode(input: super::Instruction) -> super::#encode_output_type {
          use std::convert::TryFrom;
          match input {
            #(#encode)*
            x => panic!(format!("Could not encode instruction: {:#?}", x)),
          }
        }
        #(#instruction_structs)*
      }
      #[derive(Debug,PartialEq,Eq)]
      pub enum Instruction {
        #instruction_seq
      }
    }
  }).into()
}

