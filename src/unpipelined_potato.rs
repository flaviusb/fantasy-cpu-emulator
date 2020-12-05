define_chip! {
  # unpipelined_potato_chip

  ## Misc

  - Instruction width: 36

  ## Raw

  type U10 = u16;
  type MachineState = (Instruction, Memories::t);
  pub fn fetch(mem: &Memories::t, input: U10) -> U36 {
    if input > 1023 {
      panic!(format!("fetch from outside of bounds: {}", input));
    }
    mem.base[input as usize]
  }
  /*pub fn tick(forward_by: u64, mem: Memories::t, pipeline_outputs: (Option<U36>, Option<Instruction>, Option<Pipeline::MemoryToArchitecturalRegisters::Instruction>, Option<StateBundle>)) -> (Memories::t, (Option<U36>, Option<Instruction>, Option<Pipeline::MemoryToArchitecturalRegisters::Instruction>, Option<StateBundle>)) {
    let mut fetched = fetch(&mem, mem.registers.ip);
    let mut decoded = match pipeline_outputs.0 {
      None    => None,
      Some(x) => Some(Pipeline::Decode::decode(x)),
    };
    let mut assign_architectural_registers = match pipeline_outputs.1 {
      None              => None,
      Some(instruction) => Some(Pipeline::MemoryToArchitecturalRegisters::memory_to_architecture_registers(instruction)),
    };
    panic!("tick not implemented.");
  }*/

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

