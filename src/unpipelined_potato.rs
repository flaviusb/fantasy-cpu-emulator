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
          //dbg!("Computing, progress {} against timing {}", progress, timing);
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


  Nop,    0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0, BackEnd <- 1 (super::super::Instruction::Nop(super::super::Instructions::Nop{}), mems) => { let mut new_mems = mems; new_mems.registers.ip += 1; new_mems } -> Nop *,  "Nop."
  AddIS,  1 0 0 1 0 0 a:[u; 10] b:[u; 10] c:[u; 10], BackEnd <- 5 (super::super::Instruction::AddIS(super::super::Instructions::AddIS{a, b, c}), mems) => { use super::super::fetch; let (m, n) = (i64::from_be_bytes(fetch(&mems, a).to_be_bytes()), i64::from_be_bytes(fetch(&mems, b).to_be_bytes())); let mut new_mems = mems; new_mems.registers.ip += 1; new_mems.base[c as usize] = u64::from_be_bytes((m + n).to_be_bytes()); new_mems } -> AddIS *,  "Add integer signed."
  AddIU,  1 0 0 1 0 1 a:[u; 10] b:[u; 10] c:[u; 10], BackEnd <- 5 (super::super::Instruction::AddIU(super::super::Instructions::AddIU{a, b, c}), mems) => { use super::super::fetch; let (m, n) = (fetch(&mems, a), fetch(&mems, b)); let mut new_mems = mems; new_mems.registers.ip += 1; new_mems.base[c as usize] = m + n; new_mems } -> AddIU *,  "Add integer unsigned."
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
  mems.base[0] = up::Instructions::encode(up::Instruction::AddIS(up::Instructions::AddIS{a: 2, b: 3, c: 4}));
  mems.base[2] = 5;
  mems.base[3] = 10;
  let tick_1 = up::begin_tick(6, mems);
  let mems_out = up::get_mem(tick_1);
  assert_eq!(mems_out.base[4], 15);
}
