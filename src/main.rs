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
    LUTIntermediate(Address, Address), // The second address points to a word that contains a half word aligned (address, 4 bit LUT specifier)
    LUT(Address, Address, Address, LUTSelector),
    // One source, one sink
    PopCnt(Address, Address), // This is immediate
    // Send
    SendMessage(Address, Flags, Address, Flags)
  }

  pub fn attempt_read_memory(chip: Chip, address: Address) -> Option<u32> {
    if address >= SCRATCH_SIZE || chip.memstate[address] == MemState::waiting_on_channel {
      return None
    }
    return Some(chip.scratch[address]);
  }

  //pub fn attempt_tick(chip: Chip) -> Chip {
  //}
}

fn main() {
    let mut chip: Core::Chip = Core::Chip {
      scratch: Core::set_chip_mem(),
      memstate: Core::set_chip_masks_paused(),
    };
    println!("Hello, world!");
}
