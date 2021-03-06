define_chip! {
  # unpipelined_potato_chip

  ## Misc

  - Instruction width: 36

  ## Raw

  type MachineState = (Instruction, Memories::t);
  #[derive(Debug,PartialEq,Eq,Clone)]
  pub enum Work {
    Fetching{ progress: u32, mem: Memories::t,},
    Computing{ progress: u32, instruction: Instruction, mem: Memories::t,},
    //Waiting{ mem: Memories::t,},
  }
  pub fn fetch(mem: &Memories::t, input: U10) -> U36 {
    if input > 1023 {
      panic!(format!("fetch from outside of bounds: {}", input));
    }
    mem.base[input as usize]
  }
  pub fn get_mem(work: Work) -> Memories::t {
    match work {
      Work::Fetching { mem, .. } => mem,
      Work::Computing { mem, .. } => mem,
    }
  }
  pub fn begin_tick(forward_by: u32, mem: Memories::t) -> Work {
    tick(forward_by, Work::Fetching { progress: 0, mem: mem } )
  }
  pub fn tick(forward_by: u32, working: Work) -> Work {
    if (forward_by > 0) {
      match working {
        Work::Fetching { progress, mem } => {
          // We have fetching take 1 cycle.
          if (progress != 0) {
            panic!("Fetch taking too long.");
          }
          let new_forward_by = forward_by - 1;
          let ip = mem.registers.ip;
          let instruction = Instructions::decode(fetch(&mem, ip));
          let work_new = Work::Computing { progress: 0, instruction: instruction, mem: mem };
          if new_forward_by == 0 {
            work_new
          } else {
            tick(new_forward_by, work_new)
          }
        },
        Work::Computing { progress, instruction, mem } => {
          let timing = Pipeline::BackEnd::timing_from_instruction(instruction.clone());
          if timing < progress {
            panic!("Computing took too long for {:?}.", instruction);
          }
          if forward_by >= (timing - progress) {
            let new_forward_by = forward_by - (timing - progress);
            let new_progress = 0;
            let new_mem = Pipeline::BackEnd::back_end((instruction, mem));
            tick(new_forward_by, Work::Fetching { progress: new_progress, mem: new_mem })
          } else {
            Work::Computing { progress: progress + forward_by, instruction: instruction, mem: mem }
          }
        },
      }
    } else {
      working
    }
  }
  pub fn u36_to_u64(from: U36) -> u64 {
    from
  }
  pub fn u64_to_u36(from: u64) -> U36 {
    (from & ((1 << 36) - 1))
  }
  pub fn u36_to_i64(from: U36) -> i64 {
    // Sign extend, then transmute
    let test_bit = (1 << 35);
    if (from & test_bit) == test_bit {
      let sign_extension: u64 = (((1u128 << 64) - 1) as u64) ^ ((1 << 36) - 1);
      i64::from_ne_bytes(u64::to_ne_bytes(from | sign_extension))
    } else {
      i64::from_ne_bytes(from.to_ne_bytes())
    }
  }
  pub fn i64_to_u36(from: i64) -> U36 {
    // We don't need to worry about sign extension, as trucation works
    // So we convert into raw and then mask
    (u64::from_ne_bytes(i64::to_ne_bytes(from)) & ((1 << 36) - 1))
  }
  pub fn clampU(from: u64, max: u64) -> u64 {
    if from > max {
      max
    } else {
      from
    }
  }
  pub fn clampS(from: i64, max: i64, min: i64) -> i64 {
    if from > max {
      max
    } else if from < min {
      min
    } else {
      from
    }
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


  Nop,          0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0, BackEnd <- 1 (super::super::Instruction::Nop(super::super::Instructions::Nop{}), mems) => { let mut new_mems = mems; new_mems.registers.ip += 1; new_mems } -> Nop *,  "Nop."
  AddIS36Sat,   1 0 0 1 0 0 a:[u; 10] b:[u; 10] c:[u; 10], BackEnd <- 5 (super::super::Instruction::AddIS36Sat(super::super::Instructions::AddIS36Sat{a, b, c}), mems) => { use super::super::{fetch, u36_to_i64, i64_to_u36, clampS}; let (m, n) = (u36_to_i64(fetch(&mems, a)), u36_to_i64(fetch(&mems, b))); let mut new_mems = mems; new_mems.registers.ip += 1; new_mems.base[c as usize] = i64_to_u36(clampS(m + n, (1 << 35) - 1, - (1 << 35))); new_mems } -> AddIS36Sat *,  "Add 36 bit signed integer."
  AddIU36Sat,   1 0 0 1 0 1 a:[u; 10] b:[u; 10] c:[u; 10], BackEnd <- 5 (super::super::Instruction::AddIU36Sat(super::super::Instructions::AddIU36Sat{a, b, c}), mems) => { use super::super::{fetch, u36_to_u64, u64_to_u36, clampU}; let (m, n) = (u36_to_u64(fetch(&mems, a)), u36_to_u64(fetch(&mems, b))); let mut new_mems = mems; new_mems.registers.ip += 1; new_mems.base[c as usize] = u64_to_u36(clampU(m + n, (1 << 36) - 1)); new_mems } -> AddIU36Sat *,  "Add 36-bit unsigned integer."
}

#[test]
fn run_nops() {
  use unpipelined_potato_chip as up;
  let mems = up::Memories::t{registers: up::Memories::registers{ip:0}, base:[0; 1024],};
  let mems_output = up::Memories::t{registers: up::Memories::registers{ip:1}, base:[0; 1024],};
  let mems_output_2 = up::Memories::t{registers: up::Memories::registers{ip:10}, base:[0; 1024],};
  let tick_1 = up::begin_tick(2, mems);
  let new_mems = up::get_mem(tick_1.clone()).clone();
  let tick_2 = up::tick(18, tick_1);
  let new_mems_2 = up::get_mem(tick_2);
  assert_eq!(mems_output, new_mems);
  assert_eq!(mems_output_2, new_mems_2);
}
#[test]
fn run_add() {
  use unpipelined_potato_chip as up;
  let mut mems = up::Memories::t{registers: up::Memories::registers{ip:0}, base:[0; 1024],};
  mems.base[0] = up::Instructions::encode(up::Instruction::AddIS36Sat(up::Instructions::AddIS36Sat{a: 2, b: 3, c: 4}));
  mems.base[2] = 5;
  mems.base[3] = 10;
  let tick_1 = up::begin_tick(6, mems);
  let mems_out = up::get_mem(tick_1);
  assert_eq!(mems_out.base[4], 15);
}
#[test]
fn roundtrip_signed_ints() {
  use unpipelined_potato_chip as up;
  assert_eq!(up::u36_to_i64(up::i64_to_u36(-(1 << 35))), -(1 << 35));
}
#[test]
fn run_adds() {
  use unpipelined_potato_chip as up;
  let mut mems = up::Memories::t{registers: up::Memories::registers{ip:0}, base:[0; 1024],};
  mems.base[0]  = up::Instructions::encode(up::Instruction::AddIS36Sat(up::Instructions::AddIS36Sat{a: 16,  b: 17,  c: 30}));
  mems.base[1]  = up::Instructions::encode(up::Instruction::AddIS36Sat(up::Instructions::AddIS36Sat{a: 17,  b: 17,  c: 31}));
  mems.base[2]  = up::Instructions::encode(up::Instruction::AddIU36Sat(up::Instructions::AddIU36Sat{a: 30, b: 17,  c: 32}));
  mems.base[3]  = up::Instructions::encode(up::Instruction::AddIS36Sat(up::Instructions::AddIS36Sat{a: 18,  b: 17,  c: 33}));
  mems.base[4]  = up::Instructions::encode(up::Instruction::AddIS36Sat(up::Instructions::AddIS36Sat{a: 18,  b: 19,  c: 34}));
  mems.base[5]  = up::Instructions::encode(up::Instruction::AddIS36Sat(up::Instructions::AddIS36Sat{a: 34, b: 20, c: 35}));
  mems.base[6]  = up::Instructions::encode(up::Instruction::AddIS36Sat(up::Instructions::AddIS36Sat{a: 14, b: 14, c: 36}));
  mems.base[7]  = up::Instructions::encode(up::Instruction::AddIU36Sat(up::Instructions::AddIU36Sat{a: 15, b: 15, c: 37}));
  mems.base[14]  = up::i64_to_u36(-(1 << 35) + 1);
  mems.base[15]  = (1 << 36) - 1;
  mems.base[16]  = 5;
  mems.base[17]  = 10;
  mems.base[18]  = up::i64_to_u36(-2);
  mems.base[19]  = up::i64_to_u36(-3);
  mems.base[20] = up::i64_to_u36(-1024);
  let tick_1 = up::begin_tick(48, mems);
  let mems_out = up::get_mem(tick_1);
  assert_eq!(mems_out.base[30], 15);
  assert_eq!(mems_out.base[31], 20);
  assert_eq!(mems_out.base[32], 25);
  assert_eq!(mems_out.base[33],  8);
  assert_eq!(up::u36_to_i64(mems_out.base[34]), -5);
  assert_eq!(up::u36_to_i64(mems_out.base[35]), -1029);
  assert_eq!(up::u36_to_i64(mems_out.base[36]), -(1 << 35));
  assert_eq!(up::u36_to_u64(mems_out.base[37]), (1 << 36) - 1);
}
