#[macro_use]
use crate::mt;

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
    StalledFetching { forward_by: u32, progress: u32, },
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
    Memories::t{registers: Memories::registers{ip:0}, base: [0; 1024], stall: [0; 1024], currently_doing: Memories::currently_doing { state: Default::default() }, interfaces: Memories::interfaces { io: IO { } }}
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
  - interfaces is state
    * io: super::IO

  ## Dis/Assembler

  ## Pipeline

  - fetch in Fetch = super::super::fetch
  - decode in Decode = super::super::Instructions::decode
  - check_stall in CheckStall: super::super::Memories::t ->  super::super::Memories::t
  - back_end in BackEnd: super::super::Memories::t -> super::super::Memories::t

  ## Instructions

  Nop,          0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0, CheckStall <- 0 (crate::mt!{Nop}) => { input } -> Nop *, BackEnd <- 1 (crate::mt!{Nop}) => { let mut new_mems = input; new_mems.registers.ip += 1; new_mems } -> Nop *,  "Nop."
  Nopi,         0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 a:[u; 10], CheckStall <- 0 (crate::mt!{Nopi a}) => { input } -> Nopi *, BackEnd <- 1 (crate::mt!{Nopi a}) => { let mut new_mems = input; new_mems.registers.ip += 1; new_mems } -> Nopi *,  "Nopi."
}
