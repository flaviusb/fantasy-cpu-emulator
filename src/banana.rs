#[macro_use]
use crate::mc;

define_chip! {
  # banana_chip

  ## Misc

  - Instruction width: 36
  - CopyState: true

  ## Raw
  
  #[derive(Debug,PartialEq,Eq,Clone,Copy)]
  pub enum UpToThree<T> {
    Zero,
    One(T),
    Two(T, T,),
    Three(T, T, T,),
  }

  impl<T> IntoIterator for UpToThree<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
      match self {
        UpToThree::Zero => vec!().into_iter(),
        UpToThree::One(a) => vec!(a).into_iter(),
        UpToThree::Two(a, b) => vec!(a, b).into_iter(),
        UpToThree::Three(a, b, c) => vec!(a, b, c).into_iter(),
      }
    }
  }
  impl<T> UpToThree<T> {
    fn add(self, thing: T) -> UpToThree<T> {
      match self {
        UpToThree::Zero           => UpToThree::One(thing),
        UpToThree::One(a)         => UpToThree::Two(thing,a),
        UpToThree::Two(a, b)      => UpToThree::Three(thing,a,b),
        UpToThree::Three(a, b, c) => UpToThree::Three(thing, a, b), // Things fall off the end
      }
    }
  }
  #[derive(Debug,PartialEq,Eq,Clone,Copy)]
  pub enum Doing {
    Fetching { progress: u32, },
    Computing { progress: u32, instruction: Instruction, },
    StalledFetching,
    StalledComputing { forward_by: u32, progress: u32, instruction: Instruction, waiting_on: UpToThree<UpToThree<U10>>}, // waiting_on is ∨(∧(i)) with the ∨ in priority order
    Halted,
  }
  impl Default for Doing {
    fn default() -> Self {
       Doing::Halted
    }
  }
  #[derive(Debug,PartialEq,Eq,Clone,Copy)]
  pub struct IO {
  }
  pub fn fresh_mem() -> Memories::t {
    Memories::t{registers: Memories::registers{ip:0}, base: [0; 1024], stall: [0; 1024], currently_doing: Memories::currently_doing { state: Default::default() }, interfaces: Memories::interfaces { io: IO { } }, muarch_regs: Memories::muarch_regs { a: 0 }}
  }
  impl Default for Memories::t {
    fn default() -> Self {
      fresh_mem()
    }
  }
  pub fn fetch(mem: &Memories::t, input: U10) -> U36 {
    if input > 1023 {
      panic!(format!("fetch from outside of bounds: {}", input));
    }
    mem.base[input as usize]
  }
  pub fn get_stalled(mem: Memories::t, input: U10) -> bool {
    if input > 1023 {
      panic!(format!("check stall from outside of bounds: {}", input));
    }
    (mem.stall[input as usize] >> 7) == 1
  }
  pub fn advance_ip(ip: U10) -> U10 {
    let mut new_ip = ip + 1;
    if new_ip > 1023 {
      0
    } else {
      new_ip
    }
  }
  pub fn make_fetch(mem: &mut Memories::t) {
    mem.currently_doing.state = Doing::Fetching { progress: 0 };
  }
  const fetch_and_decode_timing: u32 = 1;
  pub fn tick(mem: Memories::t) -> Memories::t {
    match mem.currently_doing.state {
      Doing::Fetching { progress } => {
        if get_stalled(mem, mem.registers.ip) {
          let mut new_mem = mem;
          new_mem.currently_doing.state = Doing::StalledFetching;
          new_mem
        } else {
          match progress {
            x if x < fetch_and_decode_timing => { mem },
            x if x == fetch_and_decode_timing  => { let it = fetch(&mem, mem.registers.ip); let inst = Pipeline::Decode::decode(it); let mut new_mem = mem; new_mem.currently_doing.state = Doing::Computing { progress: 0, instruction: inst }; new_mem },
            _ => panic!("Too long fetching"),
          }
        }
      },
      Doing::StalledFetching => {
        if get_stalled(mem, mem.registers.ip) {
          mem
        } else {
          let mut new_mem = mem;
          make_fetch(&mut new_mem);
          new_mem
        }
      },
      x @ Doing::Computing { .. } => {
        Pipeline::BackEnd::back_end(mem)
      },
      x @ Doing::StalledComputing { .. } => {
        let maybe_stalled_mem = Pipeline::CheckStall::check_stall(mem);
        match maybe_stalled_mem.currently_doing.state {
          x @ Doing::Computing { .. } => tick(maybe_stalled_mem),
          _ => maybe_stalled_mem,
        }
      },
      Doing::Halted => { mem },
    }
  }

  ## Memory

  - base is scratch
    * 36 bit word
    * 10 bit address size
    * 1024 words
  - stall is scratch
    * 8 bit word
    * 10 bit address size
    * 1024 words
  - registers is register
    * ip: 10 bit
  - currently_doing is state
    * state: super::Doing
  - muarch_regs is state
    * a: super::U36
  - interfaces is state
    * io: super::IO

  ## Dis/Assembler

  ## Pipeline

  - fetch in Fetch = super::super::fetch
  - decode in Decode = super::super::Instructions::decode
  - check_stall in CheckStall: super::super::Memories::t ->  super::super::Memories::t
  - back_end in BackEnd: super::super::Memories::t -> super::super::Memories::t

  ## Instructions

  Nop,          0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0, CheckStall <- 0 (crate::mc!{Nop}) => { input } -> Nop *, BackEnd <- 1 (crate::mc!{Nop}) => { let mut new_mems = input; new_mems.registers.ip += 1; new_mems } -> Nop *,  "Nop."
  Nopn,         0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 n:[u; 10], CheckStall <- 0 (crate::mc!{Nopn n}) => { input } -> Nopn *, BackEnd <- 1 (crate::mc!{Nopn n; progress}) => { let mut new_mems = input; new_mems.registers.ip += 1; new_mems } -> Nopn *,  "Nopn."
  Nop1i,        0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 a:[u; 10], CheckStall <- 0 (crate::mc!{Nop1i a}) => {
    let mut mem = input;
    use super::super::{get_stalled, Doing, UpToThree, Instruction, Instructions};
    if get_stalled(input, input.registers.ip) {
      mem.currently_doing.state = Doing::StalledComputing { forward_by: 0, progress: 0, instruction: Instruction::Nop1i(Instructions::Nop1i{a: a}), waiting_on: UpToThree::One(UpToThree::One(a)) };
      mem
    } else {
      mem
    }
  } -> Nop1i *, BackEnd <- 2 (crate::mc!{Nop1i a; progress}) => { 
    use super::super::{fetch, advance_ip, make_fetch};
    match progress {
      0 => { let thing = fetch(&input, a); let mut new_mems = input; new_mems.muarch_regs.a = thing; new_mems },
      1 => { let thing = input.muarch_regs.a; let mut new_mems = input; new_mems.base[input.registers.ip as usize] = thing; new_mems },
      2 => { let mut new_mems = input; new_mems.registers.ip = advance_ip(new_mems.registers.ip); make_fetch(&mut new_mems); new_mems },
      _ => panic!("Too much progress"),
    }
  } -> Nop1i *,  "Nop1i."
  //F,            1 0 0 0 0 0 a:[u; 10] b:[u; 10] c:[u; 10], CheckStall <- 1 (crate::mc!{F a b c}) => { input }
}
