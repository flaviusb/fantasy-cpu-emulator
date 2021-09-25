#[macro_use]
use crate::mc;

define_chip! {
  # lotus_chip

  ## Misc

  - Instruction width: 32
  - CopyState: true

  ## Raw
  
  pub type MemBank = [u32; 256];
  //impl Default for MemBank {
  //  fn default() -> Self {
  //    [0; 256]
  //  }
  //}
  #[derive(Debug,PartialEq,Eq,Clone,Copy)]
  pub struct BankedMem {
    bank_selection: [u8; 16],
    mems: [MemBank; 15],
  }
  impl Default for BankedMem {
    fn default() -> Self {
      BankedMem { bank_selection: [0;16], mems: [[0; 256]; 15], }
    }
  }

  ## Memory

  - base is state
    * banks: super::BankedMem
  
  ## Dis/Assembler

  ## Pipeline

  //- fetch in Fetch = super::super::fetch
  - decode in Decode = super::super::Instructions::decode

  ## Instructions

  Nop,          0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0, "Nop."
}
