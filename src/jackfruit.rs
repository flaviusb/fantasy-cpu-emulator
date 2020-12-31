define_chip! {
  # jackfruit_chip
  // Smaller, jammier

  ## Misc

  - Instruction width: 36

  ## Raw

  type MachineState = (Instruction, Memories::t);
  #[derive(Debug,PartialEq,Eq,Clone)]
  pub enum Work {
    Fetching{ progress: u32, mem: Memories::t,},
    Computing{ progress: u32, instruction: Instruction, mem: Memories::t,},
    StalledFetching { forward_by: u32, progress: u32, mem: Memories::t,},
    StalledComputing { forward_by: u32, progress: u32, instruction: Instruction, mem: Memories::t, waiting_on: Vec<Vec<U9>>}, // waiting_on is ∨(∧(i)) with the ∨ in priority order
  }
  pub fn fetch(mem: &Memories::t, input: U9) -> U36 {
    if input > 511 {
      panic!(format!("fetch from outside of bounds: {}", input));
    }
    mem.base[input as usize]
  }
  pub fn get_mem(work: Work) -> Memories::t {
    match work {
      Work::Fetching { mem, .. } => mem,
      Work::Computing { mem, .. } => mem,
      Work::StalledFetching { mem, .. } => mem,
      Work::StalledComputing { mem, .. } => mem,
    }
  }
  pub fn get_stalled(mem: Memories::t, input: U9) -> bool {
    (mem.stall[input as usize] >> 7) == 1
  }
  pub fn ticker(forward_by: u32, working: Work) -> Work {
    let mut forward_prog = forward_by;
    let mut working_prog = working;
    while forward_prog > 0 {
      let x = tick(forward_prog, working_prog.clone());
      forward_prog = x.0;
      working_prog = x.1;
    }
    working_prog
  }
  pub fn begin_tick(forward_by: u32, mem: Memories::t) -> Work {
    ticker(forward_by, Work::Fetching { progress: 0, mem: mem } )
  }
  pub fn tick(forward_by: u32, working: Work) -> (u32, Work) {
    //println!("tick {:?} {}.", working.clone(), forward_by);
    if (forward_by > 0) {
      match working {
        Work::StalledFetching { forward_by: forward_by_2, progress, mem } => {
          if get_stalled(mem, mem.registers.ip) {
            (0, Work::StalledFetching { forward_by: 0, progress, mem })
          } else {
            (forward_by+forward_by_2, Work::Fetching { progress: 0, mem: mem })
          }
        },
        Work::Fetching { progress, mem } => {
          // We have fetching take 1 cycle.
          if (progress != 0) {
            panic!("Fetch taking too long.");
          }
          let ip = mem.registers.ip;
          if get_stalled(mem, mem.registers.ip) {
            (0, Work::StalledFetching { forward_by, progress, mem })
          } else {
            let new_forward_by = forward_by - 1;
            let instruction = Instructions::decode(fetch(&mem, ip));
            let work_new = Work::Computing { progress: 0, instruction: instruction, mem: mem };
            if new_forward_by == 0 {
              (0, work_new)
            } else {
              (new_forward_by, work_new)
            }
          }
        },
        Work::Computing { progress, instruction, mem } => {
          let timing = Pipeline::BackEnd::timing_from_instruction(instruction.clone());
          if timing < progress {
            panic!("Computing took too long for {:?}.", instruction);
          }
          match Pipeline::CheckStall::check_stall((instruction, mem)) {
            Work::Fetching {..} => panic!("Invalid fetch."),
            Work::StalledFetching {..} => panic!("Invalid stall on fetch."),
            Work::Computing { progress, instruction, mem } => {
              if forward_by >= (timing - progress) {
                let new_forward_by = forward_by - (timing - progress);
                let new_progress = 0;
                let new_mem = Pipeline::BackEnd::back_end((instruction, mem));
                (new_forward_by, Work::Fetching { progress: new_progress, mem: new_mem })
              } else {
                (0, Work::Computing { progress: progress + forward_by, instruction: instruction, mem: mem })
              }
            },
            Work::StalledComputing { forward_by: forward_by_2, progress, instruction, mem, waiting_on } => {
              (0, Work::StalledComputing { forward_by: 0, progress, instruction, mem, waiting_on })
            },
          }
        },
        Work::StalledComputing { forward_by: forward_by_2, progress, instruction, mem, waiting_on } => {
          let timing = Pipeline::BackEnd::timing_from_instruction(instruction.clone());
          if timing < progress {
            panic!("Computing took too long for {:?}.", instruction);
          }
          for wait_and in waiting_on.clone() {
            let mut run = false;
            for wait in wait_and {
              if !get_stalled(mem, wait) {
                run = true;
              } else {
                run = false;
                break;
              }
            }
            if run {
              if forward_by >= (timing - progress) {
                let new_forward_by = forward_by - (timing - progress);
                let new_progress = 0;
                let new_mem = Pipeline::BackEnd::back_end((instruction, mem));
                return (new_forward_by, Work::Fetching { progress: new_progress, mem: new_mem });
              } else {
                return (0, Work::Computing { progress: progress + forward_by, instruction: instruction, mem: mem });
              }
            }
          }
          (0, Work::StalledComputing { forward_by: 0, progress, instruction, mem, waiting_on })
        },
      }
    } else {
      (0, working)
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
  pub fn u36_to_i32(from: U36) -> i32 {
    let small = (from & ((1<<32)-1)) as u32;
    i32::from_ne_bytes(u32::to_ne_bytes(small))
  }
  pub fn i32_to_u36(from: i32) -> U36 {
    u32::from_ne_bytes(i32::to_ne_bytes(from)) as u64
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
    * 9 bit address size
    * 512 words
  - stall is scratch
    * 8 bit word
    * 9 bit address size
    * 512 words
  - registers is register
    * ip: 9 bit

  ## Dis/Assembler

  ## Pipeline

  - fetch in Fetch = super::super::fetch
  - decode in Decode = super::super::Instructions::decode
  - check_stall in CheckStall: super::super::MachineState -> super::super::Work
  - back_end in BackEnd: super::super::MachineState -> super::super::Memories::t

  ## Instructions


  Nop,          0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0, CheckStall <- 0 (instruction @ super::super::Instruction::Nop(super::super::Instructions::Nop{}), mems) => { super::super::Work::Computing { progress: 0, instruction, mem: mems } } -> Nop *, BackEnd <- 1 (super::super::Instruction::Nop(super::super::Instructions::Nop{}), mems) => { let mut new_mems = mems; new_mems.registers.ip += 1; new_mems } -> Nop *,  "Nop."
  //F,          1 0 0 0 0 0 0 0 0 a:[u; 9] b:[u; 9] c:[u; 9], BackEnd <- 1 (super::super::Instruction::F(super::super::Instructions::f{a, b, c}), mems) => { let mut new_mems = mems; new_mems.registers.ip += 1; new_mems } -> Nop *,  "Nop."
  AddIS36Sat,   1 0 1 0 0 0 0 0 0 a:[u; 9] b:[u; 9] c:[u; 9], CheckStall <- 0 (super::super::Instruction::AddIS36Sat(super::super::Instructions::AddIS36Sat{a, b, c}), mems) => { 
    use super::super::{get_stalled};
    let instruction = super::super::Instruction::AddIS36Sat(super::super::Instructions::AddIS36Sat{a, b, c});
    let (a_1, b_1, c_1) = (get_stalled(mems, a), get_stalled(mems, b), get_stalled(mems, c));
    if !(a_1 | b_1 | c_1) {
      super::super::Work::Computing { progress: 0, instruction, mem: mems }
    } else {
      let mut blocker = vec!();
      if a_1 {
        blocker.push(a);
      }
      if b_1 {
        blocker.push(b);
      }
      if c_1 {
        blocker.push(c);
      }
      super::super::Work::StalledComputing { forward_by: 0, progress: 0, instruction, mem: mems, waiting_on: vec!(blocker) }
    }
  } -> AddIS36Sat *, BackEnd <- 5 (super::super::Instruction::AddIS36Sat(super::super::Instructions::AddIS36Sat{a, b, c}), mems) => { use super::super::{fetch, u36_to_i64, i64_to_u36, clampS}; let (m, n) = (u36_to_i64(fetch(&mems, a)), u36_to_i64(fetch(&mems, b))); let mut new_mems = mems; new_mems.registers.ip += 1; new_mems.base[c as usize] = i64_to_u36(clampS(m + n, (1 << 35) - 1, - (1 << 35))); new_mems } -> AddIS36Sat *,  "Add 36 bit signed integer."
  AddIU36Sat,   1 0 1 0 0 0 0 0 1 a:[u; 9] b:[u; 9] c:[u; 9], CheckStall <- 0 (super::super::Instruction::AddIU36Sat(super::super::Instructions::AddIU36Sat{a, b, c}), mems) => { 
    use super::super::{get_stalled};
    let instruction = super::super::Instruction::AddIU36Sat(super::super::Instructions::AddIU36Sat{a, b, c});
    let (a_1, b_1, c_1) = (get_stalled(mems, a), get_stalled(mems, b), get_stalled(mems, c));
    if !(a_1 | b_1 | c_1) {
      super::super::Work::Computing { progress: 0, instruction, mem: mems }
    } else {
      let mut blocker = vec!();
      if a_1 {
        blocker.push(a);
      }
      if b_1 {
        blocker.push(b);
      }
      if c_1 {
        blocker.push(c);
      }
      super::super::Work::StalledComputing { forward_by: 0, progress: 0, instruction, mem: mems, waiting_on: vec!(blocker) }
    }
  } -> AddIU36Sat *, BackEnd <- 5 (super::super::Instruction::AddIU36Sat(super::super::Instructions::AddIU36Sat{a, b, c}), mems) => { use super::super::{fetch, u36_to_u64, u64_to_u36, clampU}; let (m, n) = (u36_to_u64(fetch(&mems, a)), u36_to_u64(fetch(&mems, b))); let mut new_mems = mems; new_mems.registers.ip += 1; new_mems.base[c as usize] = u64_to_u36(clampU(m + n, (1 << 36) - 1)); new_mems } -> AddIU36Sat *,  "Add 36-bit unsigned integer."
  AddIS32SatZ,  1 0 1 0 0 0 0 1 0 a:[u; 9] b:[u; 9] c:[u; 9], CheckStall <- 0 (super::super::Instruction::AddIS32SatZ(super::super::Instructions::AddIS32SatZ{a, b, c}), mems) => { 
    use super::super::{get_stalled};
    let instruction = super::super::Instruction::AddIS32SatZ(super::super::Instructions::AddIS32SatZ{a, b, c});
    let (a_1, b_1, c_1) = (get_stalled(mems, a), get_stalled(mems, b), get_stalled(mems, c));
    if !(a_1 | b_1 | c_1) {
      super::super::Work::Computing { progress: 0, instruction, mem: mems }
    } else {
      let mut blocker = vec!();
      if a_1 {
        blocker.push(a);
      }
      if b_1 {
        blocker.push(b);
      }
      if c_1 {
        blocker.push(c);
      }
      super::super::Work::StalledComputing { forward_by: 0, progress: 0, instruction, mem: mems, waiting_on: vec!(blocker) }
    }
  } -> AddIS32SatZ *, BackEnd <- 5 (super::super::Instruction::AddIS32SatZ(super::super::Instructions::AddIS32SatZ{a, b, c}), mems) => { use super::super::{fetch, u36_to_i64, i64_to_u36, clampS}; let (m, n) = (u36_to_i64(fetch(&mems, a)) & ((1 << 32) - 1), u36_to_i64(fetch(&mems, b)) & ((1 << 32) - 1)); let mut new_mems = mems; new_mems.registers.ip += 1; new_mems.base[c as usize] = i64_to_u36(clampS(m + n, (1 << 31) - 1, - (1 << 31))); new_mems } -> AddIS32SatZ *,  "Add 32 bit signed integer, zeroing the high bits."
  AddIU32SatZ,  1 0 1 0 0 0 0 1 1 a:[u; 9] b:[u; 9] c:[u; 9], CheckStall <- 0 (super::super::Instruction::AddIU32SatZ(super::super::Instructions::AddIU32SatZ{a, b, c}), mems) => { 
    use super::super::{get_stalled};
    let instruction = super::super::Instruction::AddIU32SatZ(super::super::Instructions::AddIU32SatZ{a, b, c});
    let (a_1, b_1, c_1) = (get_stalled(mems, a), get_stalled(mems, b), get_stalled(mems, c));
    if !(a_1 | b_1 | c_1) {
      super::super::Work::Computing { progress: 0, instruction, mem: mems }
    } else {
      let mut blocker = vec!();
      if a_1 {
        blocker.push(a);
      }
      if b_1 {
        blocker.push(b);
      }
      if c_1 {
        blocker.push(c);
      }
      super::super::Work::StalledComputing { forward_by: 0, progress: 0, instruction, mem: mems, waiting_on: vec!(blocker) }
    }
  } -> AddIS32SatZ *, BackEnd <- 5 (super::super::Instruction::AddIU32SatZ(super::super::Instructions::AddIU32SatZ{a, b, c}), mems) => { use super::super::{fetch, u36_to_u64, u64_to_u36, clampU}; let (m, n) = (u36_to_u64(fetch(&mems, a)) & ((1 << 32) - 1), u36_to_u64(fetch(&mems, b)) & ((1 << 32) - 1)); let mut new_mems = mems; new_mems.registers.ip += 1; new_mems.base[c as usize] = u64_to_u36(clampU(m + n, (1 << 32) - 1)); new_mems } -> AddIU32SatZ *,  "Add 32-bit unsigned integer, zeroing the high bits."
  AddIS32SatP,  1 0 1 0 0 0 1 0 0 a:[u; 9] b:[u; 9] c:[u; 9], CheckStall <- 0 (super::super::Instruction::AddIS32SatP(super::super::Instructions::AddIS32SatP{a, b, c}), mems) => { 
    use super::super::{get_stalled};
    let instruction = super::super::Instruction::AddIS32SatP(super::super::Instructions::AddIS32SatP{a, b, c});
    let (a_1, b_1, c_1) = (get_stalled(mems, a), get_stalled(mems, b), get_stalled(mems, c));
    if !(a_1 | b_1 | c_1) {
      super::super::Work::Computing { progress: 0, instruction, mem: mems }
    } else {
      let mut blocker = vec!();
      if a_1 {
        blocker.push(a);
      }
      if b_1 {
        blocker.push(b);
      }
      if c_1 {
        blocker.push(c);
      }
      super::super::Work::StalledComputing { forward_by: 0, progress: 0, instruction, mem: mems, waiting_on: vec!(blocker) }
    }
  } -> AddIS32SatZ *, BackEnd <- 6 (super::super::Instruction::AddIS32SatP(super::super::Instructions::AddIS32SatP{a, b, c}), mems) => {
    use super::super::{fetch, u36_to_i64, i64_to_u36, clampS};
    let (m, n, o_high) = (u36_to_i64(fetch(&mems, a) & ((1 << 32) - 1)), u36_to_i64(fetch(&mems, b) & ((1 << 32) - 1)), fetch(&mems, c) & (((1 << 36) - 1) ^ ((1 << 32) - 1)));
    let mut new_mems = mems;
    new_mems.registers.ip += 1;
    new_mems.base[c as usize] = o_high | i64_to_u36(clampS(m + n, (1 << 31) - 1, - (1 << 31)));
    new_mems
  } -> AddIS32SatP *,  "Add 32 bit signed integer, preserving the high bits in the destination."
  AddIU32SatP,  1 0 1 0 0 0 1 0 1 a:[u; 9] b:[u; 9] c:[u; 9], CheckStall <- 0 (super::super::Instruction::AddIU32SatP(super::super::Instructions::AddIU32SatP{a, b, c}), mems) => { 
    use super::super::{get_stalled};
    let instruction = super::super::Instruction::AddIU32SatP(super::super::Instructions::AddIU32SatP{a, b, c});
    let (a_1, b_1, c_1) = (get_stalled(mems, a), get_stalled(mems, b), get_stalled(mems, c));
    if !(a_1 | b_1 | c_1) {
      super::super::Work::Computing { progress: 0, instruction, mem: mems }
    } else {
      let mut blocker = vec!();
      if a_1 {
        blocker.push(a);
      }
      if b_1 {
        blocker.push(b);
      }
      if c_1 {
        blocker.push(c);
      }
      super::super::Work::StalledComputing { forward_by: 0, progress: 0, instruction, mem: mems, waiting_on: vec!(blocker) }
    }
  } -> AddIS32SatZ *, BackEnd <- 6 (super::super::Instruction::AddIU32SatP(super::super::Instructions::AddIU32SatP{a, b, c}), mems) => {
    use super::super::{fetch, u36_to_u64, u64_to_u36, clampU};
    let (m, n, o_high) = (u36_to_u64(fetch(&mems, a) & ((1 << 32) - 1)), u36_to_u64(fetch(&mems, b) & ((1 << 32) - 1)), fetch(&mems, c) & (((1 << 36) - 1) ^ ((1 << 32) - 1)));
    let mut new_mems = mems;
    new_mems.registers.ip += 1;
    new_mems.base[c as usize] = u64_to_u36(o_high | clampU(m + n, (1 << 32) - 1));
    new_mems
  } -> AddIU32SatP *,  "Add 32-bit unsigned integer, preserving the high bits in the destination."
  AddIS32SatF,  1 0 1 0 0 0 1 1 0 a:[u; 9] b:[u; 9] c:[u; 9], CheckStall <- 0 (super::super::Instruction::AddIS32SatF(super::super::Instructions::AddIS32SatF{a, b, c}), mems) => { 
    use super::super::{get_stalled};
    let instruction = super::super::Instruction::AddIS32SatF(super::super::Instructions::AddIS32SatF{a, b, c});
    let (a_1, b_1, c_1) = (get_stalled(mems, a), get_stalled(mems, b), get_stalled(mems, c));
    if !(a_1 | b_1 | c_1) {
      super::super::Work::Computing { progress: 0, instruction, mem: mems }
    } else {
      let mut blocker = vec!();
      if a_1 {
        blocker.push(a);
      }
      if b_1 {
        blocker.push(b);
      }
      if c_1 {
        blocker.push(c);
      }
      super::super::Work::StalledComputing { forward_by: 0, progress: 0, instruction, mem: mems, waiting_on: vec!(blocker) }
    }
  } -> AddIS32SatZ *, BackEnd <- 6 (super::super::Instruction::AddIS32SatF(super::super::Instructions::AddIS32SatF{a, b, c}), mems) => {
    use super::super::{fetch, u36_to_i32, i32_to_u36};
    let (m, n) = (u36_to_i32(fetch(&mems, a)), u36_to_i32(fetch(&mems, b)));
    let mut new_mems = mems;
    new_mems.registers.ip += 1;
    let res = m.saturating_add(n);
    let flags = if m.checked_add(n) == None {
      (1 << 35)
    } else {
      0
    };
    new_mems.base[c as usize] = flags | i32_to_u36(res);
    new_mems
  } -> AddIS32SatF *,  "Add 32 bit signed integer, with flags in the high bits."
  AddIU32SatF,  1 0 1 0 0 0 1 1 1 a:[u; 9] b:[u; 9] c:[u; 9], CheckStall <- 0 (super::super::Instruction::AddIU32SatF(super::super::Instructions::AddIU32SatF{a, b, c}), mems) => { 
    use super::super::{get_stalled};
    let instruction = super::super::Instruction::AddIU32SatF(super::super::Instructions::AddIU32SatF{a, b, c});
    let (a_1, b_1, c_1) = (get_stalled(mems, a), get_stalled(mems, b), get_stalled(mems, c));
    if !(a_1 | b_1 | c_1) {
      super::super::Work::Computing { progress: 0, instruction, mem: mems }
    } else {
      let mut blocker = vec!();
      if a_1 {
        blocker.push(a);
      }
      if b_1 {
        blocker.push(b);
      }
      if c_1 {
        blocker.push(c);
      }
      super::super::Work::StalledComputing { forward_by: 0, progress: 0, instruction, mem: mems, waiting_on: vec!(blocker) }
    }
  } -> AddIS32SatZ *, BackEnd <- 6 (super::super::Instruction::AddIU32SatF(super::super::Instructions::AddIU32SatF{a, b, c}), mems) => {
    use super::super::{fetch, u36_to_u64, u64_to_u36, clampU};
    let (m, n) = (u36_to_u64(fetch(&mems, a) & ((1 << 32) - 1)), u36_to_u64(fetch(&mems, b) & ((1 << 32) - 1)));
    let mut new_mems = mems;
    new_mems.registers.ip += 1;
    let clamped_result = clampU(m + n, (1 << 32) - 1);
    let flags = if (m + n) != clamped_result {
      (1 << 35)
    } else {
      0
    };
    new_mems.base[c as usize] = u64_to_u36(flags | clamped_result);
    new_mems
  } -> AddIU32SatF *,  "Add 32-bit unsigned integer, with flags in the high bits."
}

#[test]
fn run_nops() {
  use jackfruit_chip as jc;
  let mems = jc::Memories::t{registers: jc::Memories::registers{ip:0}, base:[0; 512], stall:[0; 512],};
  let mems_output = jc::Memories::t{registers: jc::Memories::registers{ip:1}, base:[0; 512], stall:[0; 512],};
  let mems_output_2 = jc::Memories::t{registers: jc::Memories::registers{ip:10}, base:[0; 512], stall:[0; 512],};
  let tick_1 = jc::begin_tick(2, mems);
  let new_mems = jc::get_mem(tick_1.clone()).clone();
  let tick_2 = jc::ticker(18, tick_1);
  let new_mems_2 = jc::get_mem(tick_2);
  assert_eq!(mems_output, new_mems);
  assert_eq!(mems_output_2, new_mems_2);
}
#[test]
fn run_add() {
  use jackfruit_chip as jc;
  let mut mems = jc::Memories::t{registers: jc::Memories::registers{ip:0}, base:[0; 512], stall:[0; 512],};
  mems.base[0] = jc::Instructions::encode(jc::Instruction::AddIS36Sat(jc::Instructions::AddIS36Sat{a: 2, b: 3, c: 4}));
  mems.base[2] = 5;
  mems.base[3] = 10;
  let tick_1 = jc::begin_tick(6, mems);
  let mems_out = jc::get_mem(tick_1);
  assert_eq!(mems_out.base[4], 15);
}
#[test]
fn roundtrip_signed_ints() {
  use jackfruit_chip as jc;
  assert_eq!(jc::u36_to_i64(jc::i64_to_u36(-(1 << 35))), -(1 << 35));
}
#[test]
fn run_adds() {
  use jackfruit_chip as jc;
  fn split_flags_i64(from: u64) -> (u8, i64) {
    let flags = (0b1111 & (from >> 32)) as u8;
    let out   = jc::u36_to_i64(from & ((1<<32) - 1));
    (flags, out)
  }
  let mut mems = jc::Memories::t{registers: jc::Memories::registers{ip:0}, base:[0; 512], stall:[0; 512],};
  mems.base[0]  = jc::Instructions::encode(jc::Instruction::AddIS36Sat(jc::Instructions::AddIS36Sat{a: 116,  b: 117,  c: 230}));
  mems.base[1]  = jc::Instructions::encode(jc::Instruction::AddIS36Sat(jc::Instructions::AddIS36Sat{a: 117,  b: 117,  c: 231}));
  mems.base[2]  = jc::Instructions::encode(jc::Instruction::AddIU36Sat(jc::Instructions::AddIU36Sat{a: 230, b: 117,  c: 232}));
  mems.base[3]  = jc::Instructions::encode(jc::Instruction::AddIS36Sat(jc::Instructions::AddIS36Sat{a: 118,  b: 117,  c: 233}));
  mems.base[4]  = jc::Instructions::encode(jc::Instruction::AddIS36Sat(jc::Instructions::AddIS36Sat{a: 118,  b: 119,  c: 234}));
  mems.base[5]  = jc::Instructions::encode(jc::Instruction::AddIS36Sat(jc::Instructions::AddIS36Sat{a: 234, b: 120, c: 235}));
  mems.base[6]  = jc::Instructions::encode(jc::Instruction::AddIS36Sat(jc::Instructions::AddIS36Sat{a: 114, b: 114, c: 236}));
  mems.base[7]  = jc::Instructions::encode(jc::Instruction::AddIU36Sat(jc::Instructions::AddIU36Sat{a: 115, b: 115, c: 237}));
  mems.base[8]  = jc::Instructions::encode(jc::Instruction::AddIU32SatZ(jc::Instructions::AddIU32SatZ{a: 115, b: 115, c: 238}));
  mems.base[9]  = jc::Instructions::encode(jc::Instruction::AddIS32SatF(jc::Instructions::AddIS32SatF{a: 114, b: 114, c: 239}));
  mems.base[10] = jc::Instructions::encode(jc::Instruction::AddIU32SatF(jc::Instructions::AddIU32SatF{a: 115, b: 115, c: 240}));
  mems.base[11] = jc::Instructions::encode(jc::Instruction::AddIU32SatF(jc::Instructions::AddIU32SatF{a: 121, b: 121, c: 241}));
  mems.base[12] = jc::Instructions::encode(jc::Instruction::AddIS32SatF(jc::Instructions::AddIS32SatF{a: 122, b: 122, c: 242}));
  mems.base[13] = jc::Instructions::encode(jc::Instruction::AddIS32SatF(jc::Instructions::AddIS32SatF{a: 123, b: 124, c: 243}));
  mems.base[114]  = jc::i64_to_u36(-(1 << 35) + 1);
  mems.base[115]  = (1 << 36) - 1;
  mems.base[116]  = 5;
  mems.base[117]  = 10;
  mems.base[118]  = jc::i64_to_u36(-2);
  mems.base[119]  = jc::i64_to_u36(-3);
  mems.base[120]  = jc::i64_to_u36(-1024);
  mems.base[121]  = (1 << 32) - 1;
  mems.base[122]  = jc::i64_to_u36(- (1 << 31));
  mems.base[123]  = 0;
  mems.base[124]  = (1 << 31) - 1;
  let tick_1 = jc::begin_tick(89, mems);
  let mems_out = jc::get_mem(tick_1);
  assert_eq!(mems_out.base[230], 15);
  assert_eq!(mems_out.base[231], 20);
  assert_eq!(mems_out.base[232], 25);
  assert_eq!(mems_out.base[233],  8);
  assert_eq!(jc::u36_to_i64(mems_out.base[234]), -5);
  assert_eq!(jc::u36_to_i64(mems_out.base[235]), -1029);
  assert_eq!(jc::u36_to_i64(mems_out.base[236]), -(1 << 35));
  assert_eq!(jc::u36_to_u64(mems_out.base[237]), (1 << 36) - 1);
  assert_eq!(jc::u36_to_u64(mems_out.base[238]), (1 << 32) - 1);
  //assert_eq!(jc::u36_to_u64(mems_out.base[239]), (1 << 32) - 1);
  //assert_eq!(split_flags_i64(jc::u36_to_u64(mems_out.base[239])), (0b1000, 2));
  let flag = split_flags_i64(jc::u36_to_u64(mems_out.base[242])).0;
  let out = jc::u36_to_i32(mems_out.base[242]);
  assert_eq!((flag, out), (0b1000, i32::MIN));
  assert_eq!(jc::u36_to_u64(mems_out.base[241]), (1 << 35) | ((1 << 32) - 1));
  assert_eq!(jc::u36_to_u64(mems_out.base[243]), ((1 << 31) - 1));
}
