define_chip! {
  # unpipelined_potato_chip

  ## Misc

  - Instruction width: 36

  ## Raw

  type U10 = u16;
  type MachineState = (Instruction, Memories::t);
  pub enum Work {
    Fetching{ progress: u64, mem: Memories::t,},
    Computing{ progress: u64, instruction: Instruction, mem: Memories::t,},
    Waiting{ mem: Memories::t,},
  }
  pub fn fetch(mem: &Memories::t, input: U10) -> U36 {
    if input > 1023 {
      panic!(format!("fetch from outside of bounds: {}", input));
    }
    mem.base[input as usize]
  }
  pub fn tick(forward_by: u64, working: Work) -> Work {
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
  - back_end in BackEnd: super::super::MachineState -> super::super::Memories::t

  ## Instructions


  Nop,    0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0, BackEnd <- 1 (super::super::Instruction::Nop(super::super::Instructions::Nop{}), mems) => { let mut new_mems = mems; new_mems.registers.ip += 1; new_mems } *,  "Nop."
}

