pub mod Core {
  const SCRATCH_SIZE: usize = 8192;
  // Reserved size = IP + (3 * stack) + state
  const IP_SIZE: u32 = 1;
  const STACK_SIZE: u32 = 256;
  const EXTRA_STATE_SIZE: u32 = 1;
  const START_POS: u32 = IP_SIZE + (3 * STACK_SIZE) + EXTRA_STATE_SIZE;

  pub struct Chip {
    pub scratch:  [u32; SCRATCH_SIZE],
    pub memstate: [MemState; SCRATCH_SIZE],
  }

  pub fn set_chip_mem() -> [u32; SCRATCH_SIZE] {
    let mut state = [0; SCRATCH_SIZE];
    //state[0] = START_POS;
    return state;
  }

  #[derive(Copy, Clone, Eq, PartialEq)]
  pub enum MemState {
    rw, waiting_on_channel
  }

  pub fn set_chip_masks_paused() -> [MemState; SCRATCH_SIZE] {
    let mut memstate: [MemState; SCRATCH_SIZE] = [MemState::rw; SCRATCH_SIZE];
    memstate[0] = MemState::waiting_on_channel;
    return memstate;
  }

  pub type Address = usize; // This should be a u13, but we don't have the machinery to do that.

  pub type Flags = u8; // This should be a u3, but we don't have the machinery to do that.

  pub type Stuff = u16; // This should be u13, but we don't have the machinery to do that. It contains various flags.

  // This represents the 16 possible 2 input binary pure functions
  pub enum LUTSelector {
    F,
    Nor,
    Xq,
    Notp,
    MaterialNonimplication,
    Notq,
    Xor,
    Nand,
    And,
    Xnor,
    Q,
    IfThen,
    P,
    ThenIf,
    Or,
    T
  }

  pub enum Instruction {
    Nop(),
    // 2 source, 2 sink
    // These take 2 steps to decode
    // First we get the intermediate form, which has 2 13 bit addresses
    // The words there have 2 13 bit addresses packed in each of them, half-word aligned
    DivRemIntermediate(Address, Address),
    DivRem(Address, Address, Address, Address),
    AddCarryIntermediate(Address, Address),
    AddCarry(Address, Address, Address, Address),
    MulCarryIntermediate(Address, Address),
    MulCarry(Address, Address, Address, Address),
    SubCarryIntermediate(Address, Address),
    SubCarry(Address, Address, Address, Address),
    ShiftOverflowIntermediate(Address, Address),
    ShiftOverflow(Address, Address, Address, Address),
    LUTIntermediate(Address, Address, LUTSelector), // The second address points to a word that contains a half word aligned address. The LUTSelector comes from the instruction decoding.
    LUT(Address, Address, Address, LUTSelector),
    // One source, one sink
    PopCnt(Address, Address), // This is immediate
    // Send
    SendMessage(Address, Stuff)
  }

  pub fn attempt_write_memory(mut chip: Chip, address: Address, value: u32) -> Result<u8, u8> {
    if address >= SCRATCH_SIZE {
      panic!("address >= SCRATCH_SIZE: {}", address);
    } else if chip.memstate[address] == MemState::waiting_on_channel {
      return Err(0);
    }
    chip.scratch[address] = value;
    return Ok(0);
  }

  pub fn attempt_read_memory(chip: Chip, address: Address) -> Option<u32> {
    if address >= SCRATCH_SIZE || chip.memstate[address] == MemState::waiting_on_channel {
      return None
    }
    return Some(chip.scratch[address]);
  }

  //pub fn attempt_tick(chip: Chip) -> Chip {
  //}
  pub fn get_lut_type(selector: u32) -> LUTSelector {
    return match selector {
      0  => LUTSelector::F,
      1  => LUTSelector::Nor,
      2  => LUTSelector::Xq,
      3  => LUTSelector::Notp,
      4  => LUTSelector::MaterialNonimplication,
      5  => LUTSelector::Notq,
      6  => LUTSelector::Xor,
      7  => LUTSelector::Nand,
      8  => LUTSelector::And,
      9  => LUTSelector::Xnor,
      10 => LUTSelector::Q,
      11 => LUTSelector::IfThen,
      12 => LUTSelector::P,
      13 => LUTSelector::ThenIf,
      14 => LUTSelector::Or,
      15 => LUTSelector::T,
      _  => panic!("Invalid selector.")
    }
  }
  pub fn instruction_decode_partial(cell: u32) -> Instruction {
    // 8 + 16 = 24, so we have 5 bits worth of selector
    let selector = cell & ((2^5) - 1);
    let left: usize  = ((cell & (((2^13) - 1) << 6)) >> 6) as usize;
    let right: usize = ((cell & (((2^13) -1) << 19)) >> 19) as usize;
    if selector > 23 {
      panic!("Invalid instruction.");
    }
    let insn = match selector {
      0  => Instruction::Nop(),
      1  => Instruction::DivRemIntermediate(left, right),
      2  => Instruction::AddCarryIntermediate(left, right),
      3  => Instruction::MulCarryIntermediate(left, right),
      4  => Instruction::SubCarryIntermediate(left, right),
      5  => Instruction::ShiftOverflowIntermediate(left, right),
      6  => Instruction::PopCnt(left, right),
      7  => Instruction::SendMessage(left, right as u16),
      _  => Instruction::LUTIntermediate(left, right, get_lut_type(selector - 8)),
    };
    return insn;
  }
}

fn main() {
    let mut chip: Core::Chip = Core::Chip {
      scratch: Core::set_chip_mem(),
      memstate: Core::set_chip_masks_paused(),
    };
    println!("Hello, world!");
}
