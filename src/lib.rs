extern crate proc_macro;
extern crate syn;
#[macro_use] extern crate quote;
extern crate proc_macro2;

#[macro_use] extern crate fantasy_cpu_emulator_macros;
extern crate getopts;

//extern crate arraydeque;
//extern crate generic_array;
extern crate dasp_ring_buffer;

mod potato;
use potato::potato_chip;

mod unpipelined_potato;
use unpipelined_potato::unpipelined_potato_chip;

pub mod jackfruit;
use jackfruit::jackfruit_chip;

pub mod banana;
use banana::banana_chip;

pub mod sound;
//use sound::RingBuffer;

pub mod clock;
use clock::Clocked;


//2-5: fn_y, -> x if x >= 2 && x < 5 => wait(),
//             x if x == 5 => fn_y(),
   
#[macro_export]
macro_rules! ticky {
  ($it:ident; $(($start:literal)(-($end:literal))?: ($action:block)),+) => {
    match $it {
      $(ticky_inner!{$start $(-$end)?: $action,}),+
    }
  };
}

macro_rules! ticky_inner {
  (($exact:literal): ($action:block),) => { it if it == $literal => $block };
  (($start:literal)-($end:literal): ($action:block),) => { it if (it <= $start) && (it > $end) => (), it if it == $end => block };
}

#[macro_export]
macro_rules! mc {
  ($name:ident $($n:ident),*) => { super::super::Memories::t{ currently_doing: super::super::Memories::currently_doing { state: super::super::Doing::Computing {progress, instruction: super::super::Instruction::$name(super::super::Instructions::$name{$($n),*}), ..} }, ..} };
}

