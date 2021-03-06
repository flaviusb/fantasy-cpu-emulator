#[macro_use]
extern crate fantasy_cpu_emulator_macros;

define_chip! {
  # test_potato

  ## Misc

  - Instruction width: 36

  ## Raw

  pub struct StateBundle {
    pub change_pc: Option<U10>, pub memory_writes: Vec<MemoryWrite>,
  }
  pub struct MemoryWrite {
    pub address: U10, pub value: U36,
  }
  pub fn fetch(input: U10) -> U36 {
    1
  }
  pub fn write_out_state(input: StateBundle) -> () {
  }
  pub fn tick(forward_by: u64, mem: Memories::t, pipeline_outputs: (Option<U36>, Option<Instruction>, Option<Pipeline::MemoryToArchitecturalRegisters::Instruction>, Option<StateBundle>)) -> (Memories::t, (Option<U36>, Option<Instruction>, Option<Pipeline::MemoryToArchitecturalRegisters::Instruction>, Option<StateBundle>)) {
    panic!("tick not implemented.");
  }

  ## Memory

  - base is scratch
    * 36 bit word
    * 10 bit address size
    * 1024 words
  - registers is register
    * ip: 10 bit

  ## Dis/Assembler

  ## Pipeline

  - fetch in Fetch = super::super::fetch
  - decode in Decode = super::super::Instructions::decode
  - memory_to_architecture_registers in MemoryToArchitecturalRegisters: super::super::Instruction -> Instruction
  - compute in Compute: super::MemoryToArchitecturalRegisters::Instruction -> super::super::StateBundle
  - write_out_state in WriteOutState = super::super::write_out_state

  ## Instructions

  Add,    1 0 1 0 1 1 a:[u; 10] b:[u; 10] c:[u; 10], MemoryToArchitecturalRegisters <- 1 super::super::Instruction::Add(super::super::Instructions::Add{a, b, c}) => {use super::super::fetch; let (m, n) = (fetch(a), fetch(b)); Instruction::Add(Add{a: m, b: n, c: c})} -> Add -> pub struct Add{pub a: super::super::U36, pub b: super::super::U36, pub c: super::super::U10}, Compute <- 1 super::MemoryToArchitecturalRegisters::Instruction::Add(super::MemoryToArchitecturalRegisters::Add{a, b, c}) => { let res = a + b; super::super::StateBundle{change_pc: None, memory_writes: vec!(super::super::MemoryWrite{address: c, value: res})} } -> Add * , "Add things, 36 bit."
  Addiu,  1 0 1 0 0 0 _ _ a:u8 b:[mem; 10] c:[mem; 10],                            "Add with an unsigned immediate."
  Addis,  1 0 1 0 0 1 _ _ a:i8 b:[mem; 10] c:[mem; 10],                            "Add with a signed immediate."
  Addis3, 1 0 1 0 1 0 _ _ _ _ a:[i; 6] b:[mem; 10] c:[mem; 10],                    "Add with a six bit signed immediate."
  Addisl, 1 1 1 0 1 0 a:[i; 10] b:[mem; 10] c:[mem; 10],                           "Add with a ten bit signed immediate."
  AddI,   1 1 1 1 1 0 0 0 a:[u; 6] b:[u; 6] c:[u; 6] d:[mem; 10],                  "Add indirect with three immediate offsets which can overlap."
  Nop,    0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0, "Nop."
  Nopi,   0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 1, "Signed Nop."
}
/*

  ## Dis/Assembler

  read = #[default] read_assembly
  print = #[default] print_assembly

  pipeline does three things
  1) Collects pipeline info from Instructions section
  2) generates instruction enums/structures when a pipeline is per-instruction and doesn't use an exiting fn
  3) generates functions for each stage of the pipeline or reexports existing functions

  ## Pipeline
  
  - fetch = super::fetch
  - decode = super::Instructions::decode
  - memory_to_architectural_registers: ~ -> ~
  - compute: ~ -> super::StateBundle
  - write_out_state = super::write_out_state

  Generates
  pub mod Pipeline {
    pub use super::fetch as fetch;
    pub use super::Instruction::decode as decode;
    pub mod memory_to_architectural_registers {
      pub struct Instructions
    }
    pub memory_to_architectural_registers(input: memory_to_architectural_registers::Instruction) -> compute::Instruction {
      match input {
        <collect arms here>
      }
    }
    pub mod compute {
      pub struct Instruction
    }
    pub fn compute(input: compute::Instruction) -> super::StateBundle {
      match input {
        <collect arms here>
      }
    }
    pub use super::write_out_state as write_out_state;
  }

*/
#[test]
fn test_potato_types_exist() {
  assert_eq!(3 as test_potato::I6, 3);
  assert_eq!((test_potato::StateBundle { change_pc: Some(35), memory_writes: vec!() }).change_pc, Some(35))
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
  assert_eq!(test_potato::Instructions::decode(0b00000000000000000000000000001010_10000011111100000000110000000001), test_potato::Instruction::Addis3(test_potato::Instructions::Addis3 { a: -1, b: 3, c: 1 } ) ); // Also test that _ works
  assert_eq!(test_potato::Instructions::decode(0b00000000000000000000000000001010_10111111111000000000110000000001), test_potato::Instruction::Addis3(test_potato::Instructions::Addis3 { a: -2, b: 3, c: 1 } ) );
  assert_eq!(test_potato::Instructions::decode(0b00000000000000000000000000001111_10000001100100000110101010101101), test_potato::Instruction::AddI(  test_potato::Instructions::AddI   { a: 6, b: 16, c: 26, d: 685 } ) );
}

#[test]
fn test_potato_instruction_encode() {
  assert_eq!(test_potato::Instructions::encode(test_potato::Instruction::Nop(test_potato::Instructions::Nop {  } )), 0b00000000000000000000000000000000_00000000000000000000000000000000);
  assert_eq!(test_potato::Instructions::encode(test_potato::Instruction::Nopi(test_potato::Instructions::Nopi {  } )), 0b0000_0000_0000_0000_0000_0000_0000_0000__0000_0000_0000_0000_0000_0000_0000_0011);
  assert_eq!(test_potato::Instructions::encode(test_potato::Instruction::Addisl(test_potato::Instructions::Addisl { a:  0, b: 0, c: 0 } )), 0b00000000000000000000000000001110_10000000000000000000000000000000);
  assert_eq!(test_potato::Instructions::encode(test_potato::Instruction::Addisl(test_potato::Instructions::Addisl { a:  0, b: 0, c: 1 } )), 0b00000000000000000000000000001110_10000000000000000000000000000001);
  assert_eq!(test_potato::Instructions::encode(test_potato::Instruction::Addisl(test_potato::Instructions::Addisl { a:  0, b: 3, c: 1 } )), 0b00000000000000000000000000001110_10000000000000000000110000000001);
  assert_eq!(test_potato::Instructions::encode(test_potato::Instruction::Addisl(test_potato::Instructions::Addisl { a:  1, b: 3, c: 1 } )), 0b00000000000000000000000000001110_10000000000100000000110000000001);
  assert_eq!(test_potato::Instructions::encode(test_potato::Instruction::Addisl(test_potato::Instructions::Addisl { a: 21, b: 517, c: 3 } )), 0b00000000000000000000000000001110_10000001010110000001010000000011);
  assert_eq!(test_potato::Instructions::encode(test_potato::Instruction::Addisl(test_potato::Instructions::Addisl { a: -1, b: 3, c: 1 } )), 0b00000000000000000000000000001110_10111111111100000000110000000001);
  assert_eq!(test_potato::Instructions::encode(test_potato::Instruction::Addis3(test_potato::Instructions::Addis3 { a: -1, b: 3, c: 1 } )), 0b00000000000000000000000000001010_10000011111100000000110000000001);
  assert_eq!(test_potato::Instructions::encode(test_potato::Instruction::Addis3(test_potato::Instructions::Addis3 { a: -2, b: 3, c: 1 } )), 0b00000000000000000000000000001010_10000011111000000000110000000001);
  assert_eq!(test_potato::Instructions::encode(test_potato::Instruction::AddI(  test_potato::Instructions::AddI   { a: 6, b: 16, c: 26, d: 685 } )), 0b00000000000000000000000000001111_10000001100100000110101010101101);
}

#[test]
fn test_presence_of_memory() {
  let mems = test_potato::Memories::t{registers: test_potato::Memories::registers{ip:0}, base:[0; 1024],};
  assert_eq!(mems.registers.ip, 0);
}

#[test]
fn test_assembler() {
  assert_eq!(test_potato::Instruction::Nop(test_potato::Instructions::Nop {  } ), test_potato::Instructions::from_string("Nop", vec!()));
  assert_eq!(test_potato::Instruction::AddI(test_potato::Instructions::AddI     { a: 0,  b: 0, c: 0, d: 0 } ), test_potato::Instructions::from_string("AddI", vec!("0", "0", "0", "0")));
  assert_eq!(test_potato::Instruction::AddI(test_potato::Instructions::AddI     { a: 1,  b: 2, c: 3, d: 4 } ), test_potato::Instructions::from_string("AddI", vec!("1", "2", "3", "4")));
  assert_eq!(test_potato::Instruction::Addisl(test_potato::Instructions::Addisl { a: 0,  b: 0, c: 0       } ), test_potato::Instructions::from_string("Addisl", vec!("0",  "0", "0")));
  assert_eq!(test_potato::Instruction::Addisl(test_potato::Instructions::Addisl { a: 10, b: 3, c: 99      } ), test_potato::Instructions::from_string("Addisl", vec!("10", "3", "99")));
}


define_chip! {
  # test_notato

  ## Misc

  - Instruction width: 8

  ## Raw

  #[derive(Debug,PartialEq,Eq,Clone,Copy)]
  pub enum Foo {
    a,
    b,
  }
  type U12 = u16;
  pub fn fresh_mem() -> Memories::t {
    Memories::t{base: Memories::base{a:0, b:0, acc:0}, bundle: Memories::bundle{stack: Foo::a},}
  }

  ## Memory

  - base is register
    * a:   12 bit
    * b:   12 bit
    * acc: 12 bit
  - bundle is state
    * stack: super::Foo

  ## Dis/Assembler

  ## Pipeline
    - decode in Decode = super::super::Instructions::decode

  ## Instructions

  Nop, 0 0 0 0 0 0 0 0, "Nop."

}

#[test]
fn test_extra_state() {
  let mems = test_notato::Memories::t{base: test_notato::Memories::base{a:0, b:0, acc:0}, bundle: test_notato::Memories::bundle{stack: test_notato::Foo::a},};
  assert_eq!(mems, test_notato::fresh_mem());
}
