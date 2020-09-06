#[macro_use]
extern crate fantasy_cpu_emulator_macros;

#[test]
fn define_blank_chip() {
  define_chip! {
    #test_potato

    ## Memory

    - base
      * scratch
      * 36 bit word
      * 10 bit address size
      * 1024 words
    - ip
      * register
      * 10 bit word
      * 1 bit address size
      * 1 word

    ## Dis/Assembler

    ## Pipeline
    
    ## Instructions

    Add,     "Add %a %b %c",    1 0 1 0 1 1 a:[mem; 10] b:[mem; 10] c:[mem; 10],      "Add things."
    Addiu,   "Addiu %a %b %c",  1 0 1 0 0 0 _ _ a:u8 b:[mem; 10] c:[mem; 10],         "Add with an unsigned immediate."
    Addis,   "Addis %a %b %c",  1 0 1 0 0 1 _ _ a:i8 b:[mem; 10] c:[mem; 10],         "Add with a signed immediate."
    Addis3,  "Addis3 %a %b %c", 1 0 1 0 1 0 _ _ _ _ a:[i; 6] b:[mem; 10] c:[mem; 10], "Add with a six bit signed immediate."
    Addisl,  "Addisl %a %b %c", 1 0 1 0 1 0 a:[i; 10] b:[mem; 10] c:[mem; 10],        "Add with a ten bit signed immediate."
  };
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

  assert_ne!(test_potato::Instruction::Add(test_potato::Instructions::Add{a:3, b:3, c:3}), test_potato::Instruction::Addiu(test_potato::Instructions::Addiu{a:3, b:3, c:3}));
}
