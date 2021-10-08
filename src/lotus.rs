#[macro_use]
use crate::mc;

define_chip! {
  # lotus_chip

  ## Misc

  - Instruction width: 32
  - CopyState: true

  ## Raw
  type M1 = [u32; 256];
  type M2 = [bool; 256];
  #[derive(Debug,PartialEq,Eq,Clone,Copy)]
  pub struct BankedMem {
    bank_selection: [u8; 16],
    mems:      [M1; 15],
    mem_stall: [M2; 15],
  }
  impl Default for BankedMem {
    fn default() -> Self {
      BankedMem { bank_selection: [0;16], mems: [[0; 256]; 15], mem_stall: [[false; 256]; 15], }
    }
  }
  pub fn fetch(mem: BankedMem, i: u8) -> u32 {
    let which_bank = (i / 16) as usize;
    let bank_number = mem.bank_selection[which_bank] as usize;
    mem.mems[bank_number][i as usize]
  }

  ## Memory

  - base is state
    * banks: super::BankedMem
  
  ## Dis/Assembler

  ## Pipeline

  - fetch in Fetch = super::super::fetch
  - decode in Decode = super::super::Instructions::decode

  ## Instructions

  // Format 0
  $Nop,          0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0, "Nop."
  Sleep,        0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1, "Sleep until woken."
  Halt,         0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0, "Halt."
  // Format 1-full
  T,            0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 a:[u; 8], "T."
  F,            0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 1 a:[u; 8], "F."
  Not,          0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 1 0 a:[u; 8], "¬."
  BankSwitch_i, 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 1 1 a:[u; 8], "Bank Switch Indirect."
  // Format 1-imm
  BankSwitch_d, 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 1 segment:[u; 4] bank:[u; 4], "Bank Switch Direct."

  ## Prelude

  Nop = #@ Nop @#

}
