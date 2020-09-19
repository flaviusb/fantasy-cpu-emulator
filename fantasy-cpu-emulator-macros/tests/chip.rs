#[macro_use]
extern crate fantasy_cpu_emulator_macros;

define_chip! {
  # test_potato

  ## Misc

  - Instruction width: 36

  ## Memory

  - base is scratch
    * 36 bit word
    * 10 bit address size
    * 1024 words
  - registers is register
    * ip: 10 bit

  ## Dis/Assembler

  ## Pipeline
  
  ## Instructions

  Add,     "Add %a %b %c",     1 0 1 0 1 1 a:[mem; 10] b:[mem; 10] c:[mem; 10],                         "Add things."
  Addiu,   "Addiu %a %b %c",   1 0 1 0 0 0 _ _ a:u8 b:[mem; 10] c:[mem; 10],                            "Add with an unsigned immediate."
  Addis,   "Addis %a %b %c",   1 0 1 0 0 1 _ _ a:i8 b:[mem; 10] c:[mem; 10],                            "Add with a signed immediate."
  Addis3,  "Addis3 %a %b %c",  1 0 1 0 1 0 _ _ _ _ a:[i; 6] b:[mem; 10] c:[mem; 10],                    "Add with a six bit signed immediate."
  Addisl,  "Addisl %a %b %c",  1 1 1 0 1 0 a:[i; 10] b:[mem; 10] c:[mem; 10],                           "Add with a ten bit signed immediate."
  AddI,    "AddI %a %b %c %d", 1 1 1 1 1 0 0 0 a:[u; 6] b:[u; 6] c:[u; 6] d:[mem; 10],                  "Add indirect with three immediate offsets which can overlap."
  Nop,     "Nop",              0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0, "Nop."
  Nopi,    "Nopi",             0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 1, "Signed Nop."
}
/*

  ## Dis/Assembler

  read = #[default] read_assembly
  print = #[default] print_assembly

  ## Pipeline
  
  fetch = #[default(fetch, 36 bits)] fetch
  deposit = #[default(deposit, 36 bits)] deposit
  decode = #[default(decode)] decode
  encode = #[default(encode)] encode

*/
#[test]
fn test_potato_types_exist() {
  assert_eq!(3 as test_potato::I6, 3);
}
#[test]
fn test_potato_instructions_exist() {
  assert_ne!(test_potato::Instruction::Add(test_potato::Instructions::Add{a:3, b:3, c:3}), test_potato::Instruction::Addiu(test_potato::Instructions::Addiu{a:3, b:3, c:3}));
}
#[test]
fn test_potato_instruction_decode() {
  assert_eq!(test_potato::Instructions::decode(0b00000000000000000000000000000000_00000000000000000000000000000000), test_potato::Instruction::Nop(test_potato::Instructions::Nop {  } ));
  assert_eq!(test_potato::Instructions::decode(0b0000_0000_0000_0000_0000_0000_0000_0000__0000_0000_0000_0000_0000_0000_0000_0011), test_potato::Instruction::Nopi(test_potato::Instructions::Nopi {  } ));
  assert_eq!(test_potato::Instructions::decode(0b00000000000000000000000000001110_10000000000000000000000000000000), test_potato::Instruction::Addisl(test_potato::Instructions::Addisl { a:  0, b: 0, c: 0 } ));
  assert_eq!(test_potato::Instructions::decode(0b00000000000000000000000000001110_10000000000000000000000000000001), test_potato::Instruction::Addisl(test_potato::Instructions::Addisl { a:  0, b: 0, c: 1 } ) );
  assert_eq!(test_potato::Instructions::decode(0b00000000000000000000000000001110_10000000000000000000110000000001), test_potato::Instruction::Addisl(test_potato::Instructions::Addisl { a:  0, b: 3, c: 1 } ) );
  assert_eq!(test_potato::Instructions::decode(0b00000000000000000000000000001110_10000000000100000000110000000001), test_potato::Instruction::Addisl(test_potato::Instructions::Addisl { a:  1, b: 3, c: 1 } ) );
  assert_eq!(test_potato::Instructions::decode(0b00000000000000000000000000001110_10000001010110000001010000000011), test_potato::Instruction::Addisl(test_potato::Instructions::Addisl { a: 21, b: 517, c: 3 } ) );
  assert_eq!(test_potato::Instructions::decode(0b00000000000000000000000000001110_10111111111100000000110000000001), test_potato::Instruction::Addisl(test_potato::Instructions::Addisl { a: -1, b: 3, c: 1 } ) );
  assert_eq!(test_potato::Instructions::decode(0b00000000000000000000000000001010_10111111111100000000110000000001), test_potato::Instruction::Addis3(test_potato::Instructions::Addis3 { a: -1, b: 3, c: 1 } ) );
  assert_eq!(test_potato::Instructions::decode(0b00000000000000000000000000001010_10111111111000000000110000000001), test_potato::Instruction::Addis3(test_potato::Instructions::Addis3 { a: -2, b: 3, c: 1 } ) );
  assert_eq!(test_potato::Instructions::decode(0b00000000000000000000000000001111_10000001100100000110101010101101), test_potato::Instruction::AddI(  test_potato::Instructions::AddI   { a: 6, b: 16, c: 26, d: 685 } ) );
}
