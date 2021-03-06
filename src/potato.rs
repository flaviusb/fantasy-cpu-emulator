define_chip! {
  # potato_chip

  ## Misc

  - Instruction width: 36

  ## Raw

  type U10 = u16;
  pub struct StateBundle {
    pub change_pc: Option<U10>, pub memory_writes: Vec<MemoryWrite>,
  }
  pub struct MemoryWrite {
    pub address: U10, pub value: U36,
  }
  pub fn fetch(mem: &Memories::t, input: U10) -> U36 {
    if input > 1023 {
      panic!(format!("fetch from outside of bounds: {}", input));
    }
    mem.base[input as usize]
  }
  pub fn write_out_state(input: StateBundle) -> () {
  }
  pub fn tick(forward_by: u64, mem: Memories::t, pipeline_outputs: (Option<U36>, Option<Instruction>, Option<Pipeline::MemoryToArchitecturalRegisters::Instruction>, Option<StateBundle>)) -> (Memories::t, (Option<U36>, Option<Instruction>, Option<Pipeline::MemoryToArchitecturalRegisters::Instruction>, Option<StateBundle>)) {
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


  Nop,    0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0, MemoryToArchitecturalRegisters <- 3 super::super::Instruction::Nop(super::super::Instructions::Nop{}) => { Instruction::Nop(Nop{}) } -> Nop -> pub struct Nop {}, Compute <- 2 super::MemoryToArchitecturalRegisters::Instruction::Nop(super::MemoryToArchitecturalRegisters::Nop{}) => { super::super::StateBundle{change_pc: None, memory_writes: vec!()} } -> Nop *,  "Nop."
}

