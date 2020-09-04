#[macro_use]
extern crate fantasy_cpu_emulator_macros;

#[test]
fn define_blank_chip() {
  /*define_chip! {
    #test_potato

    ##Pipeline

    ##Instructions

    Add,     "Add %a %b %c", 1 0 1 0 1 1 a:[mem; 10] b:[mem; 10] c:[mem; 10], "Add things."
    Addiu, "Addiu %a %b %c", 1 0 1 0 0 0 _ _ a:u8 b:[mem; 10] c:[mem; 10], "Add with an unsigned immediate."
    Addis, "Addis %a %b %c", 1 0 1 0 0 1 _ _ a:i8 b:[mem; 10] c:[mem; 10], "Add with a signed immediate."
  };*/
  define_chip! {
    #test_potato

    ##Pipeline

    ##Instructions

    Add,   "Add %a %b %c", 1 0 1 0 1 1 a:u8 _ , "Add things."
    Addiu, "Addiu %a %b %c", 1 0 1 0 1 1 _ , "Add things."
  };
  assert_ne!(test_potato::Instruction::Add(test_potato::Instructions::Add{a:3}), test_potato::Instruction::Addiu(test_potato::Instructions::Addiu{}));
}
