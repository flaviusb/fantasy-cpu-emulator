const SCRATCH_SIZE: usize = 8192;
// Reserved size = IP + (3 * stack) + state
const IP_SIZE: u32 = 1;
const STACK_SIZE: u32 = 256;
const EXTRA_STATE_SIZE: u32 = 1;
const START_POS: u32 = IP_SIZE + (3 * STACK_SIZE) + EXTRA_STATE_SIZE;

struct Chip {
  scratch:  [u32; SCRATCH_SIZE],
  memstate: [MemState; SCRATCH_SIZE],
}

fn set_chip_mem() -> [u32; SCRATCH_SIZE] {
  let mut state = [0; SCRATCH_SIZE];
  //state[0] = START_POS;
  return state;
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum MemState {
  rw, waiting_on_channel
}

fn set_chip_masks_paused() -> [MemState; SCRATCH_SIZE] {
  let mut memstate: [MemState; SCRATCH_SIZE] = [MemState::rw; SCRATCH_SIZE];
  memstate[0] = MemState::waiting_on_channel;
  return memstate;
}

fn main() {
    let mut chip: Chip = Chip {
      scratch: set_chip_mem(),
      memstate: set_chip_masks_paused(),
    };
    println!("Hello, world!");
}
